use serde::Deserialize;
use sqlx::postgres::PgPool;

use axum::{
    extract::Path,
    Extension,
    response::{Html, IntoResponse, Redirect},
    Form,
    Router,
    routing::{get, post},
};

use axum_session::{Session, SessionPgPool};

use crate::common::{templates, jwt};

pub fn router() -> Router {
    Router::new()
        .route("/greet/:name", get(greet))
        .route("/signup", get(render_signup_page))
        .route("/signup", post(signup_user))
}

pub async fn greet(
    Path(name): Path<String>,
    Extension(templates): Extension<templates::Templates>,
) -> impl IntoResponse {
    let mut context = templates::new_template_context();
    context.insert("name", &name);

    Html(templates.render("hello", &context).unwrap())
}

pub async fn render_signup_page(
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
    offline: bool,
}

pub async fn signup_user(
    Extension(pool): Extension<PgPool>,
    session: Session<SessionPgPool>,
    Form(req): Form<NewUserRequest>,
) -> Redirect { 
    let mut count : usize = session.get("count").unwrap_or(0);
    count += 1;
    session.set("count", count);
    println!("{}", count.to_string());
    println!("offline value {}", &req.offline);

    // validate input
    if &req.password != &req.confirm_password {
        println!("passwords don't match");
        return Redirect::to("/signup?error=password_match_error")
    } else if &req.password.len() < &8 {
        println!("password to short");
        return Redirect::to("/signup?error=password_length")
    }

    match sqlx::query("INSERT INTO users (email, password) values ($1, $2)")
        .bind(&req.email)
        .bind(&req.password)
        .execute(&pool)
        .await {
            Ok(row) => {
                println!("user: {:?}", row);
                // generate access, refresh tokens with the role (default, admin)
                let mut token_options = jwt::ForgeOptions{ offline_mode: false};
                if req.offline {
                    token_options.offline_mode = true;
                }            

                match jwt::forge_tokens(&req.email, Some(token_options)) {
                    Ok(tokens) => {
                        println!("tokens result: {:?}", tokens);
                        // add access token to session
                        session.set("access_token", tokens.access_token);
                        session.set("id_token", tokens.id_token);
                        session.set("refresh_token", tokens.refresh_token);
                    },
                    Err(e) => {
                        println!("and error occurred when forging the tokens {:?}", e);
                    }
                }
            },
            Err(e) => {
                println!("error: {:?}", e); 
                // redirect back to the signup page with an error
                return Redirect::to("/signup?error=unique_email")
            }
        }

    Redirect::to("/_/app")
}