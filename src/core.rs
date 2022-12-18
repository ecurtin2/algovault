use rusqlite::{Connection, Result};
use std::fmt;


#[derive(Debug)]
enum ColType {
    Text,
    Real,
}

impl fmt::Display for ColType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
struct Column {
    name: String,
    dtype: ColType,
}

#[derive(Debug)]
struct Table {
    name: String,
    columns: Vec<Column>,
}

impl Table {
    fn create(&self, conn: &Connection) -> Result<usize, rusqlite::Error> {
        // TODO: maybe not have sql injection I just wanted to get this working
        let query = format!(
            "CREATE TABLE IF NOT EXISTS {} (
                id INTEGER PRIMARY KEY,
                {} {} 
            )",
            self.name,
            self.columns[0].name,
            self.columns[0].dtype,
        );

        println!("{}", query);

        conn.execute(&query, ())
    }

    fn add(&self, conn: &Connection, val: &str) -> Result<usize, rusqlite::Error>{
        let query = format!(
            "INSERT INTO {} ({}) VALUES (?1)", self.name, self.columns[0].name
        );
        println!("{}", query);
        conn.execute(&query, (val,))
    }
}

pub fn setup() -> Result<()> {
    let path = "./algovault.sqlite";
    let conn = Connection::open(path)?;
    let table = Table {
        name: "persons".to_string(),
        columns: vec![Column {
            name: "first_name".to_string(),
            dtype: ColType::Text,
        }],
    };

    table.create(&conn)?;
    table.add(&conn, "steven")?;


    // let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
    // let person_iter = stmt.query_map([], |row| {
    //     Ok(Person {
    //         id: row.get(0)?,
    //         name: row.get(1)?,
    //         data: row.get(2)?,
    //     })
    // })?;

    // for person in person_iter {
    //     println!("Found person {:?}", person.unwrap());
    // }
    Ok(())
}
