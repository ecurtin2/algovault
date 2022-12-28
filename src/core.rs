use anyhow::anyhow;
use chrono::offset::Utc;
use chrono::DateTime;
use futures::future::join_all;
use sqlx::sqlite::{SqlitePool, SqliteQueryResult};
use sqlx::{query, query_as, FromRow, Row};
use std::fmt;
use std::fs::File;
use std::path::Path;
use std::time::SystemTime;
use uuid::Uuid;

#[derive(Debug, FromRow)]
struct SqliteSchema {
    name: String,       // Col name
    r#type: String,     // col type
    notnull: bool,      // !nullable?
    dflt_value: String, // default
    pk: i32, // primary key - 0 if not part of pk, else 1-based index of the column in key
}

#[derive(Debug, Clone)]
enum ColType {
    Text,
    Real,
    // Int,
    Time,
    Uuid,
    Category,
}

impl fmt::Display for ColType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

trait SqlValue: fmt::Display {
    fn sqlite_type(&self) -> String;
}

impl SqlValue for String {
    fn sqlite_type(&self) -> String {"TEXT".to_string()}
}

impl SqlValue for f64 {
    fn sqlite_type(&self) -> String {"REAL".to_string()}
}

#[derive(Debug, Clone)]
struct Column {
    name: String,
    dtype: ColType,
}

fn to_sqlite(c: &Column) -> String {
    match c.dtype {
        ColType::Text => format!("{} {}", c.name, "TEXT"),
        ColType::Real => format!("{} {}", c.name, "REAL"),
        ColType::Uuid => format!("{} {}", c.name, "TEXT"),
        ColType::Time => format!("{} {}", c.name, "TEXT"),
        ColType::Category => format!("{} {}", c.name, "TEXT"),
    }
}

fn now_as_str() -> String {
    let now: DateTime<Utc> = SystemTime::now().into();
    format!("{}", now.format("%d/%m/%Y %T"))
}

#[derive(Debug)]
struct Table<'a> {
    name: String,
    columns: Vec<Column>,
    pool: &'a SqlitePool,
}

impl Table<'_> {
    async fn create(&self) -> anyhow::Result<()> {
        let mut conn = self.pool.acquire().await.unwrap();
        // TODO: maybe not have sql injection I just wanted to get this working

        let mut run_cols: Vec<Column> = vec![
            Column {
                name: "run_id".to_string(),
                dtype: ColType::Uuid,
            },
            Column {
                name: "created_at".to_string(),
                dtype: ColType::Time,
            },
            Column {
                name: "last_modified".to_string(),
                dtype: ColType::Time,
            },
            Column {
                name: "completed_at".to_string(),
                dtype: ColType::Time,
            },
        ];

        run_cols.extend(self.columns.clone());
        let col_stmnts: String = run_cols
            .iter()
            .map(to_sqlite)
            .collect::<Vec<String>>()
            .join(", ");

        let q = format!("CREATE TABLE IF NOT EXISTS {} ({})", self.name, col_stmnts);
        match query(&q).execute(&mut conn).await {
            Ok(_r) => Ok(()),
            Err(_e) => Err(anyhow!("Could not create table: {}", self.name)),
        }
    }

    async fn start_run(&self, run_id: Option<Uuid>) -> anyhow::Result<Uuid> {
        let mut conn = self.pool.acquire().await.unwrap();
        match run_id {
            Some(id) => Ok(id),
            None => {
                let q = format!(
                    "INSERT INTO {} (run_id, created_at) VALUES (?, ?)",
                    self.name
                );
                let id = Uuid::new_v4();
                query(&q)
                    .bind(format!("{}", id.hyphenated()))
                    .bind(now_as_str())
                    .execute(&mut conn)
                    .await?;
                Ok(id)
            }
        }
    }

    async fn set(&self, run_id: Uuid, key: &str, value: String) -> anyhow::Result<i64> {
        // TODO: union types but the rust docs where down when i wrote this

        let q = format!(
            "UPDATE {} SET {} = '{}', last_modified = '{}' WHERE run_id = '{}';",
            self.name,
            key,
            value,
            now_as_str(),
            run_id
        );
        let id = query(&q).execute(self.pool).await;
        match id {
            Ok(id) => Ok(id.last_insert_rowid()),
            Err(e) => {
                // TODO: There's gotta be a smarter way to do this right?
                if e.to_string().contains("no such column") {
                    self.add_column(key.to_string()).await?;
                    Ok(query(&q).execute(self.pool).await?.last_insert_rowid())
                } else {
                    Err(anyhow!(e))
                }
            }
        }
    }

    // async fn has_column(&self, col: &str) -> anyhow::Result<bool> {
    //     let schema = self.get_schema().await?;
    //     Ok(schema.iter().any(|c| c.name == col))
    // }

    async fn add_column(&self, name: String) -> anyhow::Result<u64> {
        // let mut conn = self.pool.acquire().await.unwrap();
        let q = format!("ALTER TABLE {} ADD {} {}", self.name, name, "TEXT");
        Ok(query(&q).execute(self.pool).await?.rows_affected())
    }

    async fn get_schema(&self) -> anyhow::Result<Vec<SqliteSchema>> {
        let schema = query_as::<_, SqliteSchema>("pragma table_info(persons)")
            .bind(&self.name)
            .fetch_all(self.pool)
            .await?;
        Ok(schema)
    }

    async fn show(&self) -> anyhow::Result<()> {
        let q = format!("SELECT * FROM {}", self.name);
        let rows = query(&q).fetch_all(self.pool).await?;
        let schema = self.get_schema().await?;

        for col in schema.iter() {
            print!(" {val:^30} ", val = col.name);
        }
        println!("");

        for row in rows {
            for (i, col) in schema.iter().enumerate() {
                match col.r#type.as_str() {
                    "REAL" => print!(" {val:^30} ", val = row.get::<f64, usize>(i)),
                    _ => print!(" {val:^30} ", val = row.get::<String, usize>(i)),
                }
            }
            println!("");
        }
        Ok(())
    }
}

pub async fn setup(path: &Path) -> anyhow::Result<()> {
    if !path.exists() {
        File::create(path)?;
    }
    // let pool = SqlitePool::connect(path.to_str().unwrap()).await?;
    let pool = SqlitePool::connect("sqlite::memory:").await?;
    let table = Table {
        name: "persons".to_string(),
        columns: vec![Column {
            name: "first_name".to_string(),
            dtype: ColType::Text,
        }],
        pool: &pool,
    };

    table.create().await?;
    println!("Table Created");
    let mut futures = Vec::new();
    for _ in 0..1000 {
        let run = table.start_run(None).await?;
        // println!("Run {}", run);
        futures.push(table.set(run, "first_name", "Evan".to_string()));
        futures.push(table.set(run, "last_name", "Steve".to_string()));
        // table.set(run, "some_float", &1.3).await?;
    }
    join_all(futures);
    println!("Done!");
    // table.show().await?;

    Ok(())

}
