use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

use axum_session::{Session, SessionPgPool};
use crate::common::jwt;

pub async fn identification_token<B>(
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let _headers = req.headers();
    let extentions = req.extensions_mut();

    let session = match extentions.get::<Session<SessionPgPool>>() {
        None => return {
            println!("failed to get session");
            Err(StatusCode::BAD_REQUEST)
        },
        Some(v) => v,
    };

    match session.get::<String>("id_token") {
        None => {
            println!("failed to get id_token from session");
            return Err(StatusCode::BAD_REQUEST)
        },
        Some(id_token) => {
            println!("id_token: {}", id_token);

            if jwt::is_token_valid::<jwt::IdTokenClaims>(&id_token) == false {
                println!("failed to get session");
                return Err(StatusCode::BAD_REQUEST)
            }
        
            match jwt::decode_token::<jwt::IdTokenClaims>(&id_token) {
                Err(_e) => {
                    println!("failed to decode identity token");
                    return Err(StatusCode::BAD_REQUEST)
                },
                Ok(v) => {
                    session.set("id_token_claims", &v.claims)
                },
            } 

            session.set("id_token", id_token);
            Ok(next.run(req).await)
        }
    }
}