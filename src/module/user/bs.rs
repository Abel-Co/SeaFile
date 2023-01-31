use rbatis::snowflake::new_snowflake_id;

use crate::module::{samba, user};
use crate::module::user::Users;
use crate::module::utils::encryption;

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
    user.password = Some(encryption::aes(&user.password.unwrap()));
    let rows_affected = user::dao::save(&user).await;
    let (account, password) = (&user.username.unwrap(), &user.password.unwrap());
    if let Err(e) = samba::create(account, password).await {
        return Err(e.to_string());
    }
    Ok(rows_affected)
}

/**
 * 更新用户
 */
pub async fn update(user_id: i64, mut user: Users) -> u64 {
    if let Some(db_user) = user::dao::get(user_id).await {
        user.username = db_user.username;   // username => smb account 创建后不可更改
        if let Some(_password) = &user.password.clone() {
            user.password = Some(encryption::aes(_password));
            if user.password != db_user.password {
                let account = &user.username.clone().unwrap();
                let _ = samba::modify_password(account, _password).await;
            }
        }
        return user::dao::update(&user).await;
    }
    0
}