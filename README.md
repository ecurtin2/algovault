# algovault
Experimentation tracking



## Features

- Fully embrace the relational model
    - i.e. run is many-to-many with experiment, so reuse of run results across experiments is possible
- all operations idempotent
    - Suitable for use in a workflow orchestration system
- built-in checking for presence of results
    - can be used to cache runs
- high performance read and write
    - I'm looking at you, mlflow.get_metric_history
- dead-simple integration points
    - The data model is simply sqlite files
- serverless
- built-in aggregations
    - computing common aggregates is crazy fast and requires little memory
- no magic
    - no global context means you can paralellize fearlessly
-

## Design

- writers write to a local copy of sqlite database (maybe in-memory?)
- runs end and the databases sent to checkpoint location
- upon read, compact checkpoints to a single database instance (read replica)
- read replica knows which instances have already been ingested, incremental update
