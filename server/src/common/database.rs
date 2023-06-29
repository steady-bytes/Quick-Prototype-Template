use sqlx::{
    postgres::{PgPoolOptions},
    PgPool,
};

use std::time::Duration;

pub async fn connect() -> PgPool {
    let db_connection_str = "postgres://application:password@localhost:5432/application".to_string();

    println!("{}", db_connection_str);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    // todo: run our migrations on startup
    // sqlx::migrate!().run(<pool>).await?;

    pool
}