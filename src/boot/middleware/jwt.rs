use actix_web::{Error, FromRequest, HttpMessage, HttpRequest};
use actix_web::dev::Payload;
use chrono::{Duration, Utc};
use futures::future::{ok, Ready};
use jsonwebtoken::{Algorithm, decode, DecodingKey, Validation};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::boot::global;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct JwtToken {
    pub sub: i64,
    pub exp: usize,
}

impl FromRequest for JwtToken {
    type Error = Error;
    type Future = Ready<Result<Self, Error>>;

    #[inline]
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        match req.extensions_mut().remove::<JwtToken>() {
            Some(jwt_token) => ok(jwt_token),
            None => ok(JwtToken::new())
        }
    }
}

impl JwtToken {
    pub fn new() -> Self {
        JwtToken { sub: 0, exp: 0 }
    }

    /// create jwt_token struct
    pub fn from_id(subject_id: i64) -> Self {
        JwtToken {
            sub: subject_id,
            exp: (Utc::now() + Duration::days(30)).timestamp() as usize,
        }
    }
    /*pub fn from_user(user: &Users) -> JwtToken {
        JwtToken {
            sub: user.id.unwrap(),
            exp: (Utc::now() + Duration::days(30)).timestamp() as usize,  // Duration::hours(1)
        }
    }*/

    /// create token
    /// secret: your secret string
    pub fn create(&self) -> Result<String, &'static str> {
        return match encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(global().jwt.key.as_ref()),
        ) {
            Ok(t) => Ok(t),
            Err(_) => Err("JWTToken encode fail !"), // in practice you would return the error
        };
    }
    /// verify token
    /// secret: your secret string
    pub fn verify(token: &str) -> Result<JwtToken, &'static str> {
        return match decode::<JwtToken>(
            &token,
            &DecodingKey::from_secret(global().jwt.key.as_ref()),
            &Validation::new(Algorithm::HS256),
        ) {
            Ok(c) => Ok(c.claims),
            Err(_) => Err("JWTToken verified fail !")
        };
    }
}