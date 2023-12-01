use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response
};

use axum_session::{Session, SessionPgPool};
use crate::common::jwt;

pub async fn refresh_token<B>(
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let _headers = req.headers();
    let extentions = req.extensions_mut();

    // get the session extractor from the request context
    let session = match extentions.get::<Session<SessionPgPool>>() {
        None => return Err(StatusCode::BAD_REQUEST),
        Some(v) => v,
    };

    // get the auth token from the request and log it
    match session.get::<String>("refresh_token") {
        None => return Ok(next.run(req).await),
        Some(token) => {
            println!("refresh_token: {}", token);
            // check to make sure the token is not empty
            if token == "" {
                return Ok(next.run(req).await)
            } else {
                // when a token is not and empty, attempt to check to see if it's a valid refresh_token
                if jwt::is_token_valid::<jwt::RefreshTokenClaims>(&token) == false {
                    return Err(StatusCode::BAD_REQUEST) 
                };
            
                match jwt::decode_token::<jwt::RefreshTokenClaims>(&token) {
                    Err(_e) => {
                        println!("failed to decode refresh token");
                        return Err(StatusCode::BAD_REQUEST) 
                    },
                    Ok(v) => session.set("refresh_token", &v.claims)
                }
            }
        }
    }

    Ok(next.run(req).await)
}