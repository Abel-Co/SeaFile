use actix_web::http::StatusCode;
use crate::module::user;
use crate::module::user::{Login, Users};

/**
 * 用户登录
 */
pub async fn login(login: Login) -> Result<Users, u16> {
    if let Some(user) = user::dao::get_by_username(&login.username.as_ref().unwrap()).await {
        if user.status != Some(1) {
            return Err(user.status.unwrap_or(401) as u16);
        }
        if login.verify_aes(&user.password.as_ref().unwrap()) {
            return Ok(user)
        }
    }
    return Err(StatusCode::UNAUTHORIZED.as_u16());  // 其他匀按登录失败处理
}

/**
 * 登录用户的 subject 信息
 */
pub async fn get_subject(id: i64) -> Option<Users> {
    user::dao::get(id).await
}