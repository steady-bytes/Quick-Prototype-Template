use sqlx::{
    postgres::{PgPoolOptions, Postgres},
    Pool,
};

use std::time::Duration;

pub async fn connect() -> Pool<Postgres> {
    let db_connection_str = "postgres://application:password@localhost:5432/application".to_string();

    println!("{}", db_connection_str);

    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database")
}