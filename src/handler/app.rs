use axum::{
    Extension,
    response::{Html, IntoResponse},
    Router,
    middleware,
};

use axum_session::{Session, SessionPgPool};
use crate::{
    common::{templates}, 
    middleware::authentication_token::authenticity_token_protected,
    middleware::identification_token::identification_token,
    middleware::refresh_token::refresh_token, 
};

use tower_http::services::{ServeDir, ServeFile};

pub fn router() -> Router {
    // this setup is specific to spa'. on any route the starts with `/app` we want the client application
    // to handle all routing. So if something is not found route the user back to the index.html were 
    // the page is loaded from
    let serve_dir = ServeDir::new("webapp/dist")
        .not_found_service(ServeFile::new("webapp/dist/index.html"));

    Router::new()
        .nest_service("/app", serve_dir.clone())
        .fallback_service(serve_dir)
        .route_layer(middleware::from_fn(authenticity_token_protected))
        .route_layer(middleware::from_fn(identification_token))
        .route_layer(middleware::from_fn(refresh_token))
}

pub async fn render_app(
    Extension(templates): Extension<templates::Templates>,
    session: Session<SessionPgPool>,
) -> impl IntoResponse {
    let context = templates::new_template_context();
 
    Html(templates.render("app", &context).unwrap()).into_response()
}