# import algovault

# print(dir(algovault))
# print(algovault.sum_as_string(3, 4))


import duckdb
import polars as pl




conn = duckdb.connect(database="algovault.duckdb", read_only=False)
conn.execute("create table if not exists people(id INTEGER, name TEXT)")
conn.execute("insert into people values (1, 'steve')")

df = pl.DataFrame(conn.query("select * from people").arrow())
print(df)