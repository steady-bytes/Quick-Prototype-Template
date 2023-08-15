use serde::Deserialize;
use rand::distributions::{Alphanumeric, DistString};
use axum::{
    Extension,
    Router, 
    Form,
    response::{Redirect, IntoResponse, Html},
    routing::{get, post},
    extract::{Query},
};
use axum_session::{Session, SessionPgPool};
use sqlx::postgres::PgPool;
use crate::common::{templates, jwt};
use crate::controller::users::attempt_user_login;

pub fn router() -> Router {
    Router::new()
        .route("/login", get(render_login_page))
        .route("/login", post(login_user))
}

#[derive(Deserialize)]
pub struct LoginErrorParams {
    pub error: Option<String>,
}

#[axum_macros::debug_handler]
pub async fn render_login_page(
    params: Query<LoginErrorParams>,
    Extension(templates): Extension<templates::Templates>,
    session: Session<SessionPgPool>,
) -> impl IntoResponse {
    let mut context = templates::new_template_context();
    let authenticity_token = Alphanumeric.sample_string(&mut rand::thread_rng(), 64);
    context.insert("error", &params.error);
    context.insert("authenticity_token", &authenticity_token);
    session.set("authenticity_token", authenticity_token);
    Html(templates.render("login_page", &context).unwrap())
}

#[derive(Deserialize, Debug)]
pub struct LoginRequest {
    email: String,
    password: String,
    authenticity_token: String,
    offline: Option<bool>,
}

#[axum_macros::debug_handler]
pub async fn login_user(
    Extension(pool): Extension<PgPool>,
    session: Session<SessionPgPool>,
    Form(req): Form<LoginRequest>,
) -> impl IntoResponse {
    let authenticity_token = session
        .get("authenticity_token")
        .unwrap_or(String::from(""));

    if &authenticity_token != &req.authenticity_token {
        return Redirect::to("/signup?error=internal_server_error")
    } 

    println!("{:?}", req);

    // attempt login
    match attempt_user_login(&pool, req.email.clone(), req.password).await {
        Ok(_v) => println!("good auth"),
        Err(_e) => return Redirect::to("/login?error=incorrect_email_password")
    }

    // generate access, refresh tokens with the role (default, admin)
    let mut token_options = jwt::ForgeOptions{ offline_mode: false};
    if req.offline == Some(true) {
        token_options.offline_mode = true;
    }
    
    match jwt::forge_tokens(&req.email.clone(), Some(token_options)) {
        Ok(tokens) => {
            // add access token to session
            session.set("access_token", tokens.access_token);
            session.set("id_token", tokens.id_token);
            session.set("refresh_token", tokens.refresh_token);
        },
        Err(e) => {
            println!("and error occurred when forging the tokens {:?}", e);
            return Redirect::to("/signup?error=internal_server_error")
        }
    }

    Redirect::to("/_/app")
}