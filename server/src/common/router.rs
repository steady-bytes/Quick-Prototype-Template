use sqlx::{
    postgres::{PgPool}
};

use axum::{Router, Extension};
use crate::common::templates;

pub async fn new(pool: PgPool) -> Router {
    let html_templates = templates::new();

    Router::new() 
        .merge(crate::handler::signup::router())
        .layer(Extension(html_templates))
        .layer(Extension(pool))
}