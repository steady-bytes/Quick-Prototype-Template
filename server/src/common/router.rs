use std::time::Duration;

use sqlx::{
    Pool, Postgres,
    postgres::{PgPool, PgPoolOptions}
};

use axum::{
    routing::{get},
    Router,
    Extension,
};

use axum_csrf::{CsrfConfig};

use crate::common::templates;
use crate::handler::signup::{
    greet,
    signup_page,
    signup_user,
};

pub async fn new(pool: Pool<Postgres>) -> Router {
    let html_templates = templates::new();
    let csrf_config = CsrfConfig::default();

    Router::new()
        .route("/greet/:name", get(greet))
        .route("/signup", get(signup_page).post(signup_user))
        .layer(Extension(html_templates))
        .with_state(csrf_config)
        .with_state(pool)
}