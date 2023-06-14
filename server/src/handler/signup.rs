use serde::Deserialize;
use axum::{
    extract,
    Extension,
    response::{Html, IntoResponse},
    Form,
};

use axum_csrf::{CsrfToken};

use crate::common::templates;

pub async fn greet(
    extract::Path(name): extract::Path<String>,
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
    token: CsrfToken, 
    Form(payload): Form<NewUserRequest>
) -> &'static str {

    println!("csrf token: {}", &payload.authenticity_token);

    if token.verify(&payload.authenticity_token).is_err() {
        "Token is invalid"
    } else {
       "Token is Valid lets do stuff!"
    }
}