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
    pub id_token: String,
    pub access_token: String,
    pub refresh_token: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AccessToken {
    pub access_token: Option<String>,
    pub expires_in: i64,
}

#[derive(Default, Clone)]
pub struct ForgeOptions {
    pub offline_mode: bool,
    // The subject of the token. This is a way to identify the user, currently we are using
    // the users email.
    // todo -> Use a `Pairwise Pseudonymous Identifier` for the subject id
    //         https://curity.io/resources/learn/jwt-best-practices/#12-pairwise-pseudonymous-identifiers 
    pub subject: String,
    // A string that identifies the principal that issued the JWT
    pub issuer: String,
    // Audience, the recipient for which the JWT is intended.
    // So in the case of an admin user, that is logging into the admin dashboard
    // something that notes the product/client that will be using token
    pub audience: Vec<String>,
    // authorized_parties, is the same as the client_id. So it's the client id that the token is issued for
    pub authorized_parties: String,
    // scope, is a mapping of what claims the token is for. It's uses a way of telling the server
    // what access the token is for. For, example a scope might have `users` in it. That could 
    // be used as a mapping to the acl for which a user would have some type of access to the `users` resource.
    pub scope: Vec<String>,
}

impl ForgeOptions {
    pub fn new() -> Self {
        Self {
            offline_mode: false,
            subject: String::new(),
            issuer: String::new(),
            audience: vec![],
            authorized_parties: String::new(),
            scope: vec![],
        }
    }

    pub fn offline(mut self, m: Option<bool>) -> Self {
        match m {
            Some(v) => self.offline_mode = v,
            None => self.offline_mode = false
        }

        self 
    }

    pub fn subject(mut self, sub: String) -> Self {
        self.subject = sub;
        self
    }

    pub fn issuer(mut self, iss: String) -> Self {
        self.issuer = iss;
        self
    }

    pub fn audience(mut self, aud: Vec<String>) -> Self {
        self.audience = aud;
        self
    }

    pub fn authorized_parties(mut self, az: String) -> Self {
        self.authorized_parties = az;
        self
    }

    pub fn scopes(mut self, scopes: Vec<String>) -> Self {
        self.scope = scopes;
        self
    }

    // forge executes the builder returning the tokens
    pub fn forge(self) -> JwtResult<Tokens> {
        forge_tokens(self)
    }
}

pub fn forge_tokens(options: ForgeOptions) -> JwtResult<Tokens> { 
    let key = b"secret";
    let header = Header { kid: Some("signing_key".to_owned()), alg: Algorithm::HS512, ..Default::default() };
    let now = Utc::now();
    let access_token_expiry = now + Duration::minutes(60);
    let id_token_expiry = now + Duration::days(365);
    let refresh_token_expiry = now + Duration::days(30);

    let mut tokens = Tokens {
        access_token: String::new(),
        id_token: String::new(),
        refresh_token: None,
    };

    match encode(&header, &AccessTokenClaims{
        iss: options.issuer.clone(),
        sub: options.subject.clone(),
        aud: options.audience.clone(),
        azp: options.authorized_parties.clone(),
        exp: access_token_expiry.timestamp(),
        iat: access_token_expiry.timestamp(),
        scope: options.scope.clone(),
    }, &EncodingKey::from_secret(key)) {
        Ok(t) => {
            println!("access_token minted");
            tokens.access_token = t.clone();
        },
        Err(e) => {
            println!("failed to mint access token");
            return Err(TokenError(e.to_string()))
        },
    };

    match encode(&header, &IdTokenClaims{
        iss: options.issuer.clone(),
        sub: options.subject.clone(),
        aud: options.audience.clone(),
        exp: id_token_expiry.timestamp(),
        iat: id_token_expiry.timestamp(), 
        email: options.subject.clone(),
    }, &EncodingKey::from_secret(key)) {
        Ok(t) => {
            println!("id_token minted");
            tokens.id_token = t.clone()
        },
        Err(e) => {
            println!("failed to mint id token");
            return Err(TokenError(e.to_string()))
        }
    }

    if options.offline_mode == true {
        println!("offline_mode is true");

        match encode(&header, &RefreshTokenClaims{
            iss: options.issuer.clone(),
            sub: options.subject.clone(),
            aud: options.audience.clone(),
            exp: refresh_token_expiry.timestamp(),
            iat: refresh_token_expiry.timestamp(),
            scope: options.scope.clone(), 
            client_id: options.authorized_parties.clone(),
        }, &EncodingKey::from_secret(key)) {
            Ok(t) => {
                println!("refresh_token minted");
                tokens.refresh_token = Some(t.clone())
            },
            Err(e) => {
                println!("failed to min refresh token");
                return Err(TokenError(e.to_string()))
            }
        }
    }
   
    Ok(tokens)
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
    scope: Vec<String>,
}

// pub fn forge_access_token(email: &str) -> JwtResult<AccessToken> {
//     let key = b"secret";
//     let header = Header { kid: Some("signing_key".to_owned()), alg: Algorithm::HS512, ..Default::default() };
//     let now = Utc::now();
//     let access_token_expiry = now + Duration::minutes(60);

//     match encode(&header, &AccessTokenClaims{
//         iss: "https://domain.auth.com/",
//         sub: email.to_owned(),
//         aud: vec![String::from("audience")],
//         azp: String::from("my_client_id"),
//         exp: access_token_expiry.timestamp(),
//         iat: access_token_expiry.timestamp(),
//         scope: vec![String::from("retail")],
//     }, &EncodingKey::from_secret(key)) {
//         Ok(t) => {
//             println!("access_token minted");
//             return Ok(AccessToken{
//                 access_token: Some(t),
//                 expires_in: access_token_expiry.timestamp()
//             })
//         },
//         Err(e) => {
//             println!("failed to mint access token");
//             return Err(TokenError(e.to_string()))
//         },
//     };
// }

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
    scope: Vec<String>,
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