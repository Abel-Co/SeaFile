use bcrypt::BcryptResult;
use rbatis::TimestampZ;
use serde::{Deserialize, Serialize};
use validator::Validate;

pub mod api;
pub mod bs;
pub mod dao;

#[derive(Debug, Default, Validate, Deserialize, Serialize)]
pub struct Login {
    #[validate(required, length(max = 50, message = "username must be less than 50 chars."))]
    pub username: Option<String>,
    #[validate(required, length(min = 6, message = "password must be more than 6 chars."))]
    pub password: Option<String>,
}

impl Login {
    /// 验证密码
    pub fn verify(&self, hash: &str) -> BcryptResult<bool> {
        bcrypt::verify(self.password.as_ref().unwrap().as_bytes(), hash)
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

#[derive(CRUDTable, Debug, Default, Validate, Serialize, Deserialize)]   // 增加了 sqlx::FromRow
pub struct Users {
    pub id: i64,
    #[validate(length(min = 6, message = "username must be more than 6 chars."))]
    pub username: Option<String>,
    #[validate(length(min = 6, message = "password must be more than 6 chars."))]
    pub password: Option<String>,
    #[validate(length(max = 320, message = "email must be less than 320 chars."))]
    pub email: Option<String>,
    #[validate(length(max = 255, message = "avatar must be less than 255 chars."))]
    pub avatar: Option<String>,
    pub phone: Option<String>,
    pub status: Option<i32>,    // 1.账号正常; 419.账号冻结（七牛扩展状态码）;
    // 类型：user、admin
    pub user_type: UserType,
    pub created_at: Option<TimestampZ>,
    pub updated_at: Option<TimestampZ>,
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]   // 增加了 sqlx::FromRow
pub enum UserType {
    Admin,
    #[default]
    User,
}

// impl Users {
//     pub fn from_login(login: Login) -> Users {
//         Users {
//             status: Some(0),
//             email: login.username.to_owned(),
//             password: login.password,
//             ..Default::default()
//         }
//     }
// }

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct Password {
    pub id: i64,
    pub new: String,
    pub old: String,
}
