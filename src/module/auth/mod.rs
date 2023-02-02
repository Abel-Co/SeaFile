use bcrypt::BcryptResult;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::boot::middleware::JwtToken;

use crate::module::utils::encryption;

pub mod api;
pub mod bs;

#[derive(Debug, Default, Serialize)]
pub struct Subject {
    pub account: String,
    pub token: String,
    pub expire: usize,
}

impl Subject {
    //! auth包作为框架层.
    //! 直接接收account, 减少对user层依赖.
    pub fn new(username: &str, jwt_token: JwtToken) -> Self {
        Subject {
            account: username.to_string(),
            token: jwt_token.create().unwrap(),
            expire: jwt_token.exp,
        }
    }
}

lazy_static! {
    pub static ref RE_USERNAME: Regex = Regex::new(r"^[a-zA-Z0-9_]{3,16}$").unwrap();
}

#[derive(Debug, Default, Validate, Deserialize, Serialize)]
pub struct Login {
    #[validate(regex(path = "RE_USERNAME", message = "The username is invalid !"))]
    pub username: Option<String>,
    #[validate(required, length(min = 6, message = "password must be more than 6 chars."))]
    pub password: Option<String>,
}

impl Login {
    /// 加盐hash验证, // duration: debug 2589, release 280 !!
    pub fn verify(&self, hash: &str) -> BcryptResult<bool> {
        bcrypt::verify(self.password.as_ref().unwrap().as_bytes(), hash)
    }
    pub fn verify_aes(&self, cipher: &str) -> bool {
        encryption::aes(&self.password.as_ref().unwrap()) == cipher
    }
}

/**
 * 生成加密密码
 * PBKDF2 < bcrypt < scrypt
 */
pub fn passhash(pass: &str) -> String {
    // let namedpass = format!("{}{}", name, pass);
    bcrypt::hash(pass.as_bytes(), bcrypt::DEFAULT_COST).unwrap()
}

pub fn passaes(pass: &str) -> String {
    encryption::aes(pass)
}