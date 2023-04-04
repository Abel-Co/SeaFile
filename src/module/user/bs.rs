use rbatis::snowflake::new_snowflake_id;

use crate::module::{auth, samba, user};
use crate::module::user::{dao, Users, UserType};

/**
 * 特定用户
 */
pub async fn get(id: i64) -> Option<Users> {
    user::dao::get(id).await
}

/**
 * 用户列表
 */
pub async fn list() -> Vec<Users> {
    let users = user::dao::list().await;
    users.into_iter().map(|x| x.set_password(None)).collect()
}

/**
 * 创建用户
 */
pub async fn create(mut user: Users) -> Result<u64, String> {
    user.id = Some(new_snowflake_id());
    let password = user.password.clone();
    if let Some(_password) = &password {
        user.password = Some(auth::passaes(&_password));
    }
    let rows_affected = user::dao::save(&user).await;
    let account = user.username.as_ref().unwrap();
    if let Err(e) = samba::create(account, password).await {
        return Err(e.to_string());
    }
    Ok(rows_affected)
}

pub async fn create_root() -> u64 {
    let user = Users {
        id: Some(new_snowflake_id()),
        username: Some("root".to_string()),
        password: Some(auth::passaes("123456")),
        user_type: UserType::Admin,
        status: Some(1),
        ..Default::default()
    };
    user::dao::save(&user).await
}

/**
 * 更新用户
 */
pub async fn update(user_id: i64, mut user: Users) -> u64 {
    if let Some(db_user) = user::dao::get(user_id).await {
        user.username = db_user.username;   // username => smb account 创建后不可更改
        user.quota = db_user.quota;
        if let Some(_password) = user.password.clone().as_ref() {
            user.password = Some(auth::passaes(_password.as_str()));
            if user.password != db_user.password {
                let account = &user.username.as_ref().unwrap();
                let _ = samba::modify_password(account, _password).await;
            }
        }
        return user::dao::update(&user).await;
    }
    0
}

/**
 * 修改密码
 */
pub async fn update_pwd(user_id: i64, old: &str, new: &str) -> u64 {
    if let Some(mut db_user) = user::dao::get(user_id).await {
        if db_user.password == Some(auth::passaes(old)) {
            db_user.password = Some(auth::passaes(new));
            let account = db_user.username.as_ref().unwrap();
            let _ = samba::modify_password(account, new).await;
            return user::dao::update(&db_user).await;
        }
    }
    0
}

/**
 * 按 username 获取用户
 */
pub async fn get_by_username(username: &str) -> Option<Users> {
    user::dao::get_by_username(username).await
}

/**
 * 按 username 获取用户 忽略大小写
 */
pub async fn get_by_username_ignore_case(username: &str) -> Vec<Users> {
    user::dao::get_by_username_ignore_case(username).await
}

/**
 * 删除用户-务必谨慎操作-校验好权限
 */
pub async fn delete(id: i64) -> u64 {
    dao::delete(id).await
}
