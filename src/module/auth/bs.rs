use actix_web::http::StatusCode;
use crate::module::user;
use crate::module::user::{Login, Users, UserType};

/**
 * 用户登录
 */
pub async fn login(login: Login) -> Result<Users, u16> {
    match user::dao::get_by_username(login.username.as_ref().unwrap()).await {
        Some(user) => {
            match user.status {
                Some(1) => {
                    match user.user_type {
                        UserType::Admin => admin_login(login, user).await,
                        UserType::User => user_login(login, user).await
                    }
                }
                _ => Err(user.status.unwrap_or(401) as u16)
            }
        }
        None => Err(StatusCode::UNAUTHORIZED.as_u16())  // 其他匀按登录失败处理
    }
}

async fn admin_login(login: Login, user: Users) -> Result<Users, u16> {
    if login.verify(user.password.as_ref().unwrap()).unwrap() { // duration: debug 2589, release 280 !!
        Ok(user)
    } else {
        Err(StatusCode::UNAUTHORIZED.as_u16())  // 密码验证，不通过 ！！
    }
}

async fn user_login(login: Login, user: Users) -> Result<Users, u16> {
    if login.password == user.password {
        Ok(user)
    } else {
        Err(StatusCode::UNAUTHORIZED.as_u16())  // 其他匀按登录失败处理
    }
}

/**
 * 登录用户的 subject 信息
 */
pub async fn get_subject(id: i64) -> Option<Users> {
    user::dao::get(id).await
}