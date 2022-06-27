use rbatis::TimestampZ;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::module::auth::Login;
use crate::module::utils;

pub mod api;
pub mod bs;
pub mod dao;

#[derive(CRUDTable, Debug, Default, Validate, Serialize, Deserialize, FromRow)]   // 增加了 sqlx::FromRow
pub struct Users {
    pub id: Option<i64>,
    pub nickname: Option<String>,
    #[validate(length(min = 6, message = "password must be more than 6 chars."))]
    pub password: Option<String>,
    #[validate(length(max = 320, message = "username must be less than 320 chars."))]
    pub email: Option<String>,
    #[validate(length(max = 255, message = "username must be less than 255 chars."))]
    pub avatar: Option<String>,
    pub unionid: Option<String>,
    pub phone: Option<String>,
    pub status: Option<i32>,
    // 状态: 0 待激活，1 已激活/已启用，-1 已冻结
    pub origin: Option<String>,
    pub created_at: Option<TimestampZ>,
    pub updated_at: Option<TimestampZ>,
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
}

#[derive(Debug, Validate, Serialize, Deserialize, FromRow)]
pub struct Password {
    pub id: i64,
    pub new: String,
    pub old: String,
}

#[derive(CRUDTable, Debug, Default, Validate, Serialize, Deserialize)]
pub struct Activate {
    // #[validate(required)]
    pub id: Option<i64>,
    pub user_id: Option<i64>,
    pub contact: Option<String>,
    #[validate(required)]
    pub code: Option<String>,
    pub content: Option<String>,
    pub created_at: Option<TimestampZ>,
    pub updated_at: Option<TimestampZ>,
}

// pub enum ActivateType {
//     PHONE { id: i64, phone: String, content: String },
//     EMAIL { id: i64, email: String, content: String },
// }

impl Activate {
    pub fn new(user_id: i64, contact: &str) -> Activate {
        let regex_email: Regex = Regex::new(r"\w+@.+").unwrap();
        let rand_code = utils::random::get_rand_code(6);    // 6 位验证码
        let mut content = String::from("");
        if regex_email.is_match(contact) {
            content.push_str("您的验证码是：\n\t");
            content.push_str(rand_code.as_str());
        }
        Activate {
            user_id: Some(user_id),
            contact: Some(contact.to_string()),
            code: Some(rand_code),
            content: Some(content),
            ..Default::default()
        }
    }

    /**
     * 生成新的验证码，更新时间戳
     */
    pub fn update(&mut self) -> &Activate {
        let rand_code = utils::random::get_rand_code(6);    // 6 位验证码
        let mut content = String::from("");
        content.push_str("您的验证码是：\n\t");
        content.push_str(rand_code.as_str());
        self.code = Some(rand_code);
        self.content = Some(content);
        self
    }
}

#[cfg(test)]
mod activate_test {
    #[test]
    pub fn test_activate_new() {
        use crate::module::user::Activate;
        let activate = Activate::new(123, "xugy@126.com");
        println!("signup: {:?}", activate);
    }
}