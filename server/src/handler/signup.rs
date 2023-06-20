use serde::Deserialize;
use sqlx::postgres::PgPool;
use axum::{
    extract::{State, Path},
    Extension,
    response::{Html, IntoResponse},
    Form,
    http::{StatusCode},
};

use crate::common::templates;

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
    email: String,
    password: String,
    confirm_password: String,
    authenticity_token: String,
}

pub async fn signup_user(
    Extension(pool): Extension<PgPool>,
    Form(payload): Form<NewUserRequest>,
) -> String { 
    "Implement me".to_string()
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}