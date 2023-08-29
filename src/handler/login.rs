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

    // attempt login
    match attempt_user_login(&pool, req.email.clone(), req.password).await {
        Ok(_v) => println!("good auth"),
        Err(_e) => return Redirect::to("/login?error=incorrect_email_password")
    }

    let scopes: Vec<String> = vec!["admin".to_string()];
    let audience: Vec<String> = vec!["webapp".to_string()];

    // generate access, refresh tokens with the role (default, admin)
    let tokens = jwt::ForgeOptions::new()
        .offline(req.offline)
        .subject(req.email)
        .issuer(String::from("https://steady-bytes.com"))
        .audience(audience)
        .authorized_parties(String::from("client_id"))
        .scopes(scopes)
        .forge();
     
    // forge tokens, if fail redirect to signup page with internal_server_error
    match tokens {
        Ok(tokens) => {
            // add access token to session
            session.set("access_token", &tokens.access_token);
            session.set("id_token", &tokens.id_token); 
            // todo -> if refresh_token is forged save it to the refresh token table
            session.set("refresh_token", &tokens.refresh_token.unwrap_or_default());

            return Redirect::to("/app")
        },
        Err(e) => {
            println!("and error occurred when forging the tokens {:?}", e);
            return Redirect::to("/signup?error=internal_server_error")
        }
    }
}