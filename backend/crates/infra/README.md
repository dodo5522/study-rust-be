# Running Entity Generator CLI

- Generate entity files from database schema
    ```sh
    sea-orm-cli generate entity -o crates/infra/src/models \
        -u postgresql://migrator:password@localhost:5432/energy \
        -s generation
    ```
