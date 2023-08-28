use axum_session::{SessionStore, SessionPgPool, SessionConfig, Key, SecurityMode};
use sqlx::{Pool, Postgres};


pub async fn new(pool: Pool<Postgres>) -> SessionStore<SessionPgPool> {
    let key = Key::generate();
    let key2 = Key::generate();
    let session_config = SessionConfig::default()
        .with_table_name("user_sessions")
        .with_key(key)
        .with_database_key(key2)
        .with_security_mode(SecurityMode::PerSession);
    let session_store = SessionStore::<SessionPgPool>::new(Some(pool.clone().into()), session_config);

    session_store.initiate().await.unwrap();

    session_store
}