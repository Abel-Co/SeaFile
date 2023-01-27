use rbatis::snowflake::new_snowflake_id;

use crate::module::{samba, user};
use crate::module::user::Users;

/**
 * 用户列表
 */
pub async fn list() -> Vec<Users> {
    user::dao::list().await
}

/**
 * 创建用户
 */
pub async fn create(mut user: Users) -> Result<u64, String> {
    user.id = Some(new_snowflake_id());
    let rows_affected = user::dao::save(&user).await;
    let (account, password) = (&user.username.unwrap(), &user.password.unwrap());
    if let Err(e) = samba::create(account, password).await {
        return Err(e.to_string())
    }
    Ok(rows_affected)
}

/**
 * 更新用户
 */
pub async fn update(user_id: i64, mut user: Users) -> u64 {
    if let Some(db_user) = user::dao::get(user_id).await {
        user.username = db_user.username;
        return user::dao::update(&user).await;
    }
    0
}