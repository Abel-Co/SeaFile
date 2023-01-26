use rbatis::snowflake::new_snowflake_id;
use crate::module::user;
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
pub async fn create(mut user: Users) -> u64 {
    user.id = Some(new_snowflake_id());
    user::dao::save(user).await
}

/**
 * 更新用户
 */
pub async fn update(user_id: i64, mut user: Users) -> u64 {
    if let Some(db_user) = user::dao::get(user_id).await {
        user.username = db_user.username;
        return user::dao::save(user).await
    }
    0
}