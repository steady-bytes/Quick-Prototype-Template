use serde::Deserialize;
use sqlx::postgres::PgPool;
use axum::{
    extract::Path,
    Extension,
    response::{Html, IntoResponse},
    Form,
    Router,
    routing::{get, post},
};

use crate::common::templates;

pub fn router() -> Router {
    Router::new()
        .route("/greet/:name", get(greet))
        .route("/signup", get(signup_page))
        .route("/api/signup", post(signup_user))
}

pub async fn greet(
    Path(name): Path<String>,
    Extension(templates): Extension<templates::Templates>,
) -> impl IntoResponse {
    let mut context = templates::new_template_context();
    context.insert("name", &name);

    Html(templates.render("hello", &context).unwrap())
}

pub async fn signup_page(
    Extension(templates): Extension<templates::Templates>,
) -> impl IntoResponse {
    let mut context = templates::new_template_context();
    context.insert("authenticity_token", "sfsdfs");

    Html(templates.render("signup_page", &context).unwrap())
}

#[derive(Deserialize, Debug)]
pub struct NewUserRequest {
    _email: String,
    _password: String,
    _confirm_password: String,
    _authenticity_token: String,
}

pub async fn signup_user(
    Extension(_pool): Extension<PgPool>,
    Form(_payload): Form<NewUserRequest>,
) -> String { 
    "Implement me".to_string()
}