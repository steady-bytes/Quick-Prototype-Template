use rand::distributions::{Alphanumeric, DistString};
use axum::{
    Extension,
    Router, 
    response::{Redirect, IntoResponse, Html},
    routing::{get, post},
};
use axum_session::{Session, SessionPgPool};
use crate::common::templates;

pub fn router() -> Router {
    Router::new()
        .route("/login", get(render_login_page))
        .route("/login", post(login_user))
}

#[axum_macros::debug_handler]
pub async fn render_login_page(
    Extension(templates): Extension<templates::Templates>,
    session: Session<SessionPgPool>,
) -> impl IntoResponse {
    let mut context = templates::new_template_context();
    let authenticity_token = Alphanumeric.sample_string(&mut rand::thread_rng(), 64);

    context.insert("authenticity_token", &authenticity_token);
    session.set("authenticity_token", authenticity_token);
    Html(templates.render("login_page", &context).unwrap())
}

#[axum_macros::debug_handler]
pub async fn login_user() -> impl IntoResponse {
    Redirect::to("/_/app")
}