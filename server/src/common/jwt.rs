use std::fmt::Debug;
use chrono::{Utc, Duration};
use serde::{Serialize, Deserialize};
use jsonwebtoken::{
    decode, 
    encode, 
    Algorithm, 
    DecodingKey, 
    EncodingKey, 
    Header, 
    Validation, 
    TokenData,
    errors::ErrorKind
};

#[derive(Debug)]
pub struct TokenError(String);

pub type JwtResult<T> = Result<T, TokenError>;

#[derive(Debug)]
pub struct Tokens {
    pub id_token: Option<String>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub expires_in: i64,
}

pub struct AccessToken {
    pub access_token: Option<String>,
    pub expires_in: i64,
}

pub struct ForgeOptions {
    pub offline_mode: bool,
}

impl ForgeOptions {
    pub fn new() -> Self {
        ForgeOptions { offline_mode: false }
    }
}

pub fn forge_tokens(email: &str, options: Option<ForgeOptions>) -> JwtResult<Tokens> {
    let opt = options.unwrap_or(ForgeOptions{ offline_mode: false});
    
    let key = b"secret";
    let header = Header { kid: Some("signing_key".to_owned()), alg: Algorithm::HS512, ..Default::default() };
    let now = Utc::now();
    let access_token_expiry = now + Duration::minutes(60);
    let id_token_expiry = now + Duration::days(365);
    let refresh_token_expiry = now + Duration::days(30);

    let mut tokens = Tokens {
        access_token: None,
        id_token: None,
        refresh_token: None,
        expires_in: access_token_expiry.timestamp(),
    };

    match encode(&header, &AccessTokenClaims{
        iss: String::from("https://domain.auth.com/"),
        sub: email.to_owned(),
        aud: vec![String::from("audience")],
        azp: String::from("my_client_id"),
        exp: access_token_expiry.timestamp(),
        iat: access_token_expiry.timestamp(),
        scope: String::from("default"),
    }, &EncodingKey::from_secret(key)) {
        Ok(t) => {
            println!("access_token minted");
            tokens.access_token = Some(t);
        },
        Err(e) => {
            println!("failed to mint access token");
            return Err(TokenError(e.to_string()))
        },
    };

    match encode(&header, &IdTokenClaims{
        iss: String::from("https://domain.auth.com/"),
        sub: email.to_owned(),
        aud: vec![String::from("audience")],
        exp: id_token_expiry.timestamp(),
        iat: id_token_expiry.timestamp(), 
        email: email.to_owned(),
    }, &EncodingKey::from_secret(key)) {
        Ok(t) => {
            println!("id_token minted");
            tokens.id_token = Some(t)
        },
        Err(e) => {
            println!("failed to mint id token");
            return Err(TokenError(e.to_string()))
        }
    }

    if opt.offline_mode == true {
        println!("offline_mode is true");

        match encode(&header, &RefreshTokenClaims{
            iss: String::from("https://domain.auth.com/"),
            sub: email.to_owned(),
            aud: vec![String::from("audience")],
            exp: refresh_token_expiry.timestamp(),
            iat: refresh_token_expiry.timestamp(),
            scope: String::from("retail"), 
            client_id: String::from("uuid-for-client"),
        }, &EncodingKey::from_secret(key)) {
            Ok(t) => {
                println!("refresh_token minted");
                tokens.refresh_token = Some(t)
            },
            Err(e) => {
                println!("failed to min refresh token");
                return Err(TokenError(e.to_string()))
            }
        }
    }
   
    Ok(tokens)
}

pub fn forge_access_token(email: &str) -> JwtResult<AccessToken> {
    let key = b"secret";
    let header = Header { kid: Some("signing_key".to_owned()), alg: Algorithm::HS512, ..Default::default() };
    let now = Utc::now();
    let access_token_expiry = now + Duration::minutes(60);

    match encode(&header, &AccessTokenClaims{
        iss: String::from("https://domain.auth.com/"),
        sub: email.to_owned(),
        aud: vec![String::from("audience")],
        azp: String::from("my_client_id"),
        exp: access_token_expiry.timestamp(),
        iat: access_token_expiry.timestamp(),
        scope: String::from("retail"),
    }, &EncodingKey::from_secret(key)) {
        Ok(t) => {
            println!("access_token minted");
            return Ok(AccessToken{
                access_token: Some(t),
                expires_in: access_token_expiry.timestamp()
            })
        },
        Err(e) => {
            println!("failed to mint access token");
            return Err(TokenError(e.to_string()))
        },
    };
}

#[derive(Debug)]
pub enum Error { 
    DecodeError,
}

/// Check to tokens validity, and decode into specified claims
pub fn decode_token<T: for<'de> Deserialize<'de> + Debug>(token: &str) -> Result<TokenData<T>, Error> {
    let validation = Validation::new(Algorithm::HS512);

    match decode::<T>(&token, &DecodingKey::from_secret(b"secret"), &validation) {
        Ok(c) => { 
            Ok(c)
        },
        Err(err) => match *err.kind() {
            ErrorKind::InvalidToken => {
                println!("Token is invalid"); // Example on how to handle a specific error
                Err(Error::DecodeError)
            }
            ErrorKind::InvalidIssuer => {
                println!("Issuer is invalid"); // Example on how to handle a specific error
                Err(Error::DecodeError)
            }
            ErrorKind::ExpiredSignature => {
                println!{"expired token signature"};
                Err(Error::DecodeError)
            }
            _ => {
                println!("Some other errors");
                println!{"error: {:?}", *err.kind()}
                Err(Error::DecodeError)
            }
        },
    }
}

pub fn is_token_valid<T: for<'de> Deserialize<'de> + Debug>(token: &str) -> bool {
    let validation = Validation::new(Algorithm::HS512);

    match decode::<T>(&token, &DecodingKey::from_secret(b"secret"), &validation) {
        Ok(c) => {
            println!("{:?}", c);
            return true 
        },
        Err(err) => match *err.kind() {
            ErrorKind::InvalidToken => {
                println!("Token is invalid"); // Example on how to handle a specific error
                return false
            }
            ErrorKind::InvalidIssuer => {
                println!("Issuer is invalid"); // Example on how to handle a specific error
                return false
            }
            ErrorKind::ExpiredSignature => {
                println!{"expired token signature"};

                return false
            }
            _ => {
                println!("Some other errors");
                println!{"error: {:?}", *err.kind()}
                return false
            }
        },
    }
} 

/// ref: https://auth0.com/docs/secure/tokens/access-tokens
/// {
///     "iss": "https://my-domain.auth0.com/",
///     "sub": "auth0|123456",
///     "aud": [
///       "https://example.com/health-api",
///       "https://my-domain.auth0.com/userinfo"
///     ],
///     "azp": "my_client_id",
///     "exp": 1311281970,
///     "iat": 1311280970,
///     "scope": "openid profile read:patients read:admin"
///   }
#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenClaims {
    iss: String,
    sub: String,
    aud: Vec<String>,
    azp: String,
    exp: i64,
    iat: i64,
    scope: String,
}

/// ref: https://auth0.com/docs/secure/tokens/refresh-tokens
/// {
///     "iss": "https://my-domain.auth0.com/",
///     "sub": "auth0|123456",
///     "aud": [
///       "https://example.com/health-api",
///       "https://my-domain.auth0.com/userinfo"
///     ],
///     "exp": 1311281970,
///     "iat": 1311280970,
///     "scope": "openid profile read:patients read:admin"
///     "client_id": "my_client_id",
///   }
#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    iss: String,
    sub: String,
    aud: Vec<String>,
    exp: i64,
    iat: i64,
    scope: String,
    client_id: String,
}

// ref: https://auth0.com/docs/secure/tokens/id-tokens/id-token-structure
// {
//     "iss": "http://my-domain.com",
//     "sub": "123456",
//     "aud": "my_client_id",
//     "exp": 1311281970,
//     "iat": 1311280970,
//     "name": "Jane Doe",
//     "email": "janedoe@example.com",
//     "picture": "http://example.com/janedoe/me.jpg"
//   }
#[derive(Debug, Serialize, Deserialize)]
pub struct IdTokenClaims {
    iss: String,
    sub: String,
    aud: Vec<String>,
    exp: i64,
    iat: i64,
    pub email: String,
}