use serde::Deserialize;
use sqlx::postgres::PgPool;
use axum::{
    extract::{FromRef, FromRequestParts, State, Path},
    Extension,
    response::{Html, IntoResponse},
    Form,
    http::{StatusCode},
};

use axum_csrf::{CsrfToken};

use crate::common::templates;
use crate::common::database::DatabaseConnection;

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
    token: CsrfToken,
) -> impl IntoResponse {
    let mut context = templates::new_template_context();
    let token = &token.authenticity_token();
    context.insert("authenticity_token", token);
    println!("csrf token: {}", token);

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
    Form(payload): Form<NewUserRequest>,
) -> Result<String, (StatusCode, String)> { 
    let mut conn = conn;
    sqlx::query_scalar("select 'hello world from pg'")
        .fetch_one(&mut conn)
        .await
        .map_err(internal_error);

    Ok("Implement me".to_string())
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}