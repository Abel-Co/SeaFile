use bcrypt::BcryptResult;
use rbatis::TimestampZ;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::module::utils::encryption;

pub mod api;
pub mod bs;
pub mod dao;

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

#[derive(CRUDTable, Debug, Default, Clone, Validate, Serialize, Deserialize)]   // 增加了 sqlx::FromRow
pub struct Users {
    pub id: Option<i64>,
    #[validate(regex(path = "RE_USERNAME", message = "The username is invalid !"))]
    pub username: Option<String>,
    #[validate(length(min = 6, message = "password must be more than 6 chars."))]
    pub password: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    #[validate(phone)]
    pub phone: Option<String>,
    #[validate(url)]
    pub avatar: Option<String>,
    pub status: Option<i32>,
    // 1.账号正常; 419.账号冻结（七牛扩展状态码）;
    // 类型：User、Admin
    pub user_type: UserType,
    pub usage: Option<u64>,
    pub created_at: Option<TimestampZ>,
    pub updated_at: Option<TimestampZ>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]   // 增加了 sqlx::FromRow
pub enum UserType {
    Admin,
    #[default]
    User,
}

impl Users {
    pub fn from_login(login: Login) -> Users {
        Users {
            status: Some(0),
            email: login.username.to_owned(),
            password: login.password,
            ..Default::default()
        }
    }
    pub fn set_password(mut self, password: Option<String>) -> Self {
        self.password = password;
        self
    }
}

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct Password {
    pub id: i64,
    pub new: String,
    pub old: String,
}
