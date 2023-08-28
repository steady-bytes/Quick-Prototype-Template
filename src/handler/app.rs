use std::process::id;

use axum::{
    Extension,
    response::{Html, IntoResponse, Redirect, Response},
    Router,
    routing::get,
};

use axum_session::{Session, SessionPgPool};
use crate::common::{templates, jwt};

pub fn router() -> Router {
    Router::new()
        .route("/app", get(render_app))
}

pub async fn render_app(
    Extension(templates): Extension<templates::Templates>,
    session: Session<SessionPgPool>,
) -> impl IntoResponse {
    let context = templates::new_template_context();

    let access_token: String = match session.get("access_token") {
        Some(v) => v,
        None => {
            println!("access token not found"); 
            return Redirect::to("/login").into_response()
        }
    };

    if jwt::is_token_valid::<jwt::AccessTokenClaims>(&access_token) == false {
        return Redirect::to("/login").into_response()
    };

    match jwt::decode_token::<jwt::AccessTokenClaims>(&access_token) {
        Err(_e) => {
            println!("failed to decode access token");
            return Redirect::to("/login").into_response()
        },
        Ok(v) => session.set("access_token", &v.claims)
    }

    session.set("access_token", access_token);

    let id_token: String = match session.get("id_token") {
        Some(v) => {
            v
        },
        None => {
            println!("id token not found");
            return Redirect::to("/login").into_response()
        }
    };

    if jwt::is_token_valid::<jwt::IdTokenClaims>(&id_token) == false {
        return Redirect::to("/login").into_response()
    }

    match jwt::decode_token::<jwt::IdTokenClaims>(&id_token) {
        Err(_e) => {
            println!("failed to decode identity token");
            return Redirect::to("/login").into_response()
        },
        Ok(v) => {
            session.set("id_token", &v.claims)
        },
    }

    session.set("id_token", id_token);

    // set the navigation items into the sidebar

    Html(templates.render("app", &context).unwrap()).into_response()
}