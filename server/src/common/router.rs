use sqlx::{
    postgres::{PgPool}
};

use axum::{
    routing::{get, post},
    Router,
    Extension,
};

use crate::common::templates;
use crate::handler::signup::{
    greet,
    signup_page,
    signup_user,
};

pub async fn new(pool: PgPool) -> Router {
    let html_templates = templates::new();

    Router::new()
        .route("/greet/:name", get(greet))
        .route("/signup", get(signup_page))
        .route("/api/signup", post(signup_user))
        .layer(Extension(html_templates))
        .layer(Extension(pool))
}