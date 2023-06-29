use axum::{
    Extension,
    response::{Html, IntoResponse},
    Router,
    routing::get,
};

use axum_session::{Session, SessionPgPool};
use crate::common::{templates};

pub fn router() -> Router {
    Router::new()
        .route("/_/app", get(render_app))
}

pub async fn render_app(
    Extension(templates): Extension<templates::Templates>,
    session: Session<SessionPgPool>,
) -> impl IntoResponse {
    let context = templates::new_template_context();

    let mut count : usize = session.get("count").unwrap_or(0);
    count += 1;
    session.set("count", count);
    println!("{}", count.to_string());

    Html(templates.render("app", &context).unwrap())
}