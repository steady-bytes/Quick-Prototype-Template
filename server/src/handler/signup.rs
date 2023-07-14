use serde::Deserialize;
use sqlx::postgres::PgPool;

use axum::{
    extract::{Path, Query},
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

#[derive(Deserialize)]
pub struct SignupErrorParams {
    pub error: Option<String>,
}

#[axum_macros::debug_handler]
pub async fn render_signup_page(
    params: Query<SignupErrorParams>,
    Extension(templates): Extension<templates::Templates>,
    Extension(pool): Extension<PgPool>,
    session: Session<SessionPgPool>,
) -> impl IntoResponse {
    let mut context = templates::new_template_context();
    context.insert("authenticity_token", "sfsdfs");
    context.insert("error", &params.error);

    let mut count : usize = session.get("count").unwrap_or(0);
    count += 1;
    session.set("count", count);

    match sqlx::query!("SELECT COUNT(*) FROM users").fetch_all(&pool).await {
        Ok(v) => {
            match v[0].count {
                Some(0) => context.insert("admin", &true),
                _ => context.insert("admin", &false),
            }
        },
        Err(_e) => context.insert("admin", &false)
    }

    Html(templates.render("signup_page", &context).unwrap())
}

#[derive(Deserialize, Debug, sqlx::FromRow)]
pub struct NewUserRequest {
    email: String,
    password: String,
    confirm_password: String,
    offline: bool,
    admin: Option<bool>,
}

pub async fn signup_user(
    Extension(pool): Extension<PgPool>,
    session: Session<SessionPgPool>,
    Form(req): Form<NewUserRequest>,
) -> Redirect { 
    let mut count : usize = session.get("count").unwrap_or(0);
    count += 1;
    session.set("count", count); 

    if &req.password != &req.confirm_password {
        return Redirect::to("/signup?error=password_match")
    } else if &req.password.len() < &8 || &req.password.len() > &20 {
        return Redirect::to("/signup?error=password_strength")
    }

    match sqlx::query("INSERT INTO users (email, password) values ($1, $2)")
        .bind(&req.email)
        .bind(&req.password)
        .execute(&pool)
        .await {
            Ok(_row) => {
                // generate access, refresh tokens with the role (default, admin)
                let mut token_options = jwt::ForgeOptions{ offline_mode: false};
                if req.offline {
                    token_options.offline_mode = true;
                }            

                match jwt::forge_tokens(&req.email, Some(token_options)) {
                    Ok(tokens) => {
                        // add access token to session
                        session.set("access_token", tokens.access_token);
                        session.set("id_token", tokens.id_token);
                        session.set("refresh_token", tokens.refresh_token);
                    },
                    Err(e) => {
                        println!("and error occurred when forging the tokens {:?}", e);
                    }
                }

                // TODO: Change insert to a transaction, if it's an admin setup then
                // add the admin role, else add the default role

            },
            Err(_e) => {
                // redirect back to the signup page with an error
                return Redirect::to("/signup?error=unique_email")
            }
        }

    Redirect::to("/_/app")
}