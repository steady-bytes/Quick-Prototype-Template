# Running Migrator CLI

- Apply all pending migrations
    ```sh
    cargo run
    ```
    ```sh
    cargo run -- up
    ```
- Apply first 10 pending migrations
    ```sh
    cargo run -- up -n 10
    ```
- Rollback last applied migrations
    ```sh
    cargo run -- down
    ```
- Rollback last 10 applied migrations
    ```sh
    cargo run -- down -n 10
    ```
- Drop all tables from the database, then reapply all migrations
    ```sh
    cargo run -- fresh
    ```
- Rollback all applied migrations, then reapply all migrations
    ```sh
    cargo run -- refresh
    ```
- Rollback all applied migrations
    ```sh
    cargo run -- reset
    ```
- Check the status of all migrations
    ```sh
    cargo run -- status
    ```

Define A Migration:

A macro example, so that you can copy, paste and then find and replace `NAME` in the migration.
```rust
#[derive(Iden)]
enum NAME {
    Table,
    Uuid,
    CreatedBy,
    CreatedAt,
    DeletedAt,
    UpdatedAt,
    // concrete table fields
}

async fn setup_NAME_table(manager: &SchemaManager<'_>) {
    let table = manager.create_table(
        Table::create().table(NAME::Table).if_not_exists()
        .col(ColumnDef::new(NAME::Uuid).uuid().not_null().unique_key().primary_key())
        .col(ColumnDef::new(NAME::CreatedBy).uuid().not_null())
        .col(ColumnDef::new(NAME::CreatedAt).timestamp().not_null())
        .col(ColumnDef::new(NAME::UpdatedAt).timestamp().not_null())
        .col(ColumnDef::new(NAME::DeletedAt).timestamp())
        .to_owned()
    ).await;

    match table {
        Err(e) => panic!("{e}"),
        Ok(()) => {
            println!("NAME table created")
        }
    }

    // If `CreatedBy` is used, supply a fk to the users table
    let fk = manager.create_foreign_key(
        ForeignKey::create().name("fk_created_by")
            .from(NAME::Table, NAME::CreatedBy)
            .to(Users::Table, Users::Uuid)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned()
    ).await;

    match fk {
        Err(e) => panic!("{e}"),
        Ok(()) => {
            println!("fk create_by => users::uuid")
        }
    }

    // FK EXAMPLE
    let fk = manager.create_foreign_key(
        ForeignKey::create().name("fk_NAME_")
            .from(NAME::Table, NAME::UserUuid)
            .to(RELATION::Table, RELATION::Uuid)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned()
    ).await;

    match fk {
        Err(e) => panic!("{e}"),
        Ok(()) => {
            println!("fk FROM => TO")
        }
    }
}

async fn drop_NAME_table(manager: &SchemaManager<'_>) {
    match manager.drop_table(Table::drop().table(NAME::Table).to_owned()).await {
        Err(e) => panic!("{e}"),
        Ok(()) => {
            println!("NAME table dropped")
        }
    }
}
```