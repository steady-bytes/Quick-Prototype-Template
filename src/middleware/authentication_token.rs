use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response
};

use axum_session::{Session, SessionPgPool};
use crate::common::jwt;

// Is a wrapper around the returned extention type

pub async fn authenticity_token_protected<B>(
    mut req: Request<B>, 
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let _headers = req.headers();
    let extentions = req.extensions_mut();

    println!("auth middleware");

    // get the session extractor from the request context
    let session = match extentions.get::<Session<SessionPgPool>>() {
        None => return {
            println!("no session found");
            Err(StatusCode::BAD_REQUEST)
        },
        Some(v) => v,
    };

    // get the auth token from the request and log it
    match session.get::<String>("access_token") {
        None => {
            println!("access_token not found"); 
            return Err(StatusCode::BAD_REQUEST)
        },
        Some(access_token) => {
            println!("found access access_token: {}", access_token);

            if jwt::is_token_valid::<jwt::AccessTokenClaims>(&access_token) == false {
                println!("token is invalid");
                return Err(StatusCode::BAD_REQUEST) 
            };
        
            match jwt::decode_token::<jwt::AccessTokenClaims>(&access_token) {
                Err(_e) => {
                    println!("failed to decode access token");
                    return Err(StatusCode::BAD_REQUEST) 
                },
                Ok(v) => session.set("access_token_claims", &v.claims)
            } 

            // set the access_token back into the session
            session.set("access_token", access_token);

            // return the call to next
            Ok(next.run(req).await)
        }
    }
}