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

use axum_session::{Session, SessionPgPool};
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
    session: Session<SessionPgPool>,
) -> impl IntoResponse {
    let mut context = templates::new_template_context();
    context.insert("authenticity_token", "sfsdfs");

    let mut count : usize = session.get("count").unwrap_or(0);
    count += 1;
    session.set("count", count);

    println!("{}", count.to_string());

    Html(templates.render("signup_page", &context).unwrap())
}

#[derive(Deserialize, Debug, sqlx::FromRow)]
pub struct NewUserRequest {
    email: String,
    password: String,
    confirm_password: String,
    authenticity_token: String,
}

pub async fn signup_user(
    Extension(pool): Extension<PgPool>,
    session: Session<SessionPgPool>,
    Form(req): Form<NewUserRequest>,
) -> String { 
    let mut count : usize = session.get("count").unwrap_or(0);
    count += 1;
    session.set("count", count);
    println!("{}", count.to_string());

    // verify the csrf token

    // validate input
    if &req.password != &req.confirm_password {
        println!("passwords don't match");
        return "passwords don't match".to_string()
    } else if &req.password.len() < &8 {
        println!("password to short");
        return "password is to short".to_string()
    }

    println!("{}", &req.email);

    let _ = sqlx::query("INSERT INTO users (email, password) values ($1, $2)")
        .bind(&req.email)
        .bind(&req.password)
        .fetch_one(&pool)
        .await;


    // add access_token, and refresh_token to the session

    // redirect to app page

    "Implement me".to_string()
}