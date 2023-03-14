use rbatis::TimestampZ;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::module::auth::{Login, RE_USERNAME};

pub mod api;
pub mod bs;
pub mod dao;

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
    pub quota: Option<u64>,
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
    pub id: Option<i64>,
    pub new: String,
    pub old: String,
}
