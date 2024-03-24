use rbatis::crud::{CRUD, Skip};
use rbatis::db::DBExecResult;
use rbson::Bson;

use crate::boot::c::RB;
use crate::module::user::{Users};

/**
 * 用户列表
 */
pub async fn list() -> Vec<Users> {
    RB.fetch_list_by_wrapper(
        RB.new_wrapper().order_bys(&[("user_type", true), ("username", true)])
    ).await.unwrap()
}

/**
 * Get用户
 */
pub async fn get(id: i64) -> Option<Users> {
    RB.fetch_by_column("id", &id).await.unwrap()
}

/**
 * 增加用户
 */
pub async fn save(users: &Users) -> rbatis::Result<DBExecResult> {
    RB.save(users, &[Skip::Value(Bson::Null)]).await
}

/**
 * 更新用户
 */
pub async fn update(users: &Users) -> u64 {
    RB.update_by_column("id", users).await.unwrap()
}

/**
 * 按 username 获取用户
 */
pub async fn get_by_username(username: &str) -> Option<Users> {
    RB.fetch_by_wrapper(RB.new_wrapper().eq("username", username)).await.unwrap()
}

/**
 * 按 username 获取用户 忽略大小写
 */
pub async fn get_by_username_ignore_case(username: &str) -> Vec<Users> {
    RB.fetch("select * from users where username ilike $1 ;", vec![Bson::String(username.to_string())]).await.unwrap()
}

/**
 * 删除用户
 */
pub async fn delete(id: i64) -> u64 {
    RB.remove_by_column::<Users, i64>("id", id).await.unwrap()
}

