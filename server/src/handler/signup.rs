use rand::distributions::{Alphanumeric, DistString};
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
use crate::controller::users::{
    count_users, 
    insert_user, 
    InsertUserParams,
    UsersError,
};

pub fn router() -> Router {
    Router::new()
        .route("/greet/:name", get(greet))
        .route("/signup", get(render_signup_page))
        .route("/signup", post(signup_user))
}

pub async fn greet(
    Path(name): Path<String>,
    Extension(templates): Extension<templates::Templates>,
    session: Session<SessionPgPool>,
) -> impl IntoResponse {
    let mut context = templates::new_template_context();
    context.insert("name", &name);

    let mut count : usize = session.get("count").unwrap_or(0);
    count += 1;
    session.set("count", count); 
    println!("{}", count);

    Html(templates.render("hello", &context).unwrap())
}

#[derive(Deserialize)]
pub struct SignupErrorParams {
    pub error: Option<String>,
}

///
/// render_signup_page
/// Will render an html signup page. 
/// @TODO -> Generate a strong `authenticity_token` using something that is signed with a secret key
///          example: https://medium.com/@web3developer/signing-and-verifying-messages-with-hmac-in-rust-using-ring-69e6ed93ee78
#[axum_macros::debug_handler]
pub async fn render_signup_page(
    params: Query<SignupErrorParams>,
    Extension(templates): Extension<templates::Templates>,
    Extension(pool): Extension<PgPool>,
    session: Session<SessionPgPool>,
) -> impl IntoResponse {
    let mut context = templates::new_template_context();
    let authenticity_token = Alphanumeric.sample_string(&mut rand::thread_rng(), 64);
    println!("{}", authenticity_token);
    context.insert("authenticity_token", &authenticity_token);
    session.set("authenticity_token", authenticity_token);
    context.insert("error", &params.error);

    match count_users(&pool).await {
        Ok(v) => {
            if v < 1 {
                context.insert("admin", &true)
            } else {
                context.insert("admin", &false)
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
    authenticity_token: String,
    admin: Option<bool>,
}

pub async fn signup_user(
    Extension(pool): Extension<PgPool>,
    session: Session<SessionPgPool>,
    Form(req): Form<NewUserRequest>,
) -> Redirect { 
    let authenticity_token = session.get("authenticity_token").unwrap_or(String::from(""));

    if &authenticity_token != &req.authenticity_token {
        return Redirect::to("/signup?error=internal_server_error")
    } 

    if &req.password != &req.confirm_password {
        return Redirect::to("/signup?error=password_match")
    } else if &req.password.len() < &8 || &req.password.len() > &20 {
        return Redirect::to("/signup?error=password_strength")
    }

    let role_name = match count_users(&pool).await {
        Ok(v) => if v > 0 {String::from("default")} else {String::from("admin")}
        Err(_e) => String::from("default"),
    };

    let insert_params = &InsertUserParams{
        email: req.email.clone(),
        password: req.password.clone(),
        role_name: role_name,
    };

    match insert_user(&pool, insert_params).await {
       Ok(v) => v,
       Err(e) => {
            if let UsersError::FailedUserInsertUniqueEmail = e {
                return Redirect::to("/signup?error=unique_email") 
            } else {
                return Redirect::to("/signup?error=internal_server_error")
            }
       }
    };

    println!("user saved");

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
            return Redirect::to("/signup?error=internal_server_error")
        }
    }

    println!("tokens added to session");

    Redirect::to("/_/app")
}