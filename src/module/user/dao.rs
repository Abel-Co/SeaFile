use std::vec::Vec;

use rbatis::crud::{CRUD, Skip};
use rbson::Bson;

use crate::boot::c;
use crate::boot::c::RB;
use crate::module::user::Password;
use crate::module::user::Users;

/**
 * 用户接口: 查 列表
 */
pub async fn list() -> Vec<Users> {
    vec![Users { ..Default::default() }]
    // let pool = c::sqlx_pool();
    // sqlx::query_as::<_, Users>("select * from users")
    //     .fetch_all(pool).await.unwrap()
}

/**
 * 创建用户
 */
pub async fn create(user: &Users) -> u64 {
    RB.save(&user, &[Skip::Value(Bson::Null)]).await.unwrap().rows_affected
}

/**
 * 用户接口: 查
 */
pub async fn show(id: i64) -> Users {
    RB.fetch_by_column("id", &id).await.unwrap()
    // let pool = c::sqlx_pool();
    // sqlx::query_as::<_, Users>("select * from users where id = $1")
    //     .bind(id).fetch_one(pool).await.unwrap()
}

/**
 * 按 username 获取用户
 */
pub async fn get_by_username(username: &str) -> Option<Users> {
    RB.fetch_by_wrapper(RB.new_wrapper().eq("username", username)).await.unwrap()
}

pub async fn get_by_email(email: &str) -> Option<Users> {
    RB.fetch_by_column("email", email).await.unwrap()
}

/**
 * 用户接口: 修改用户密码
 */
pub async fn update(password: Password) -> u64 {
    1
    // let pool = c::sqlx_pool();
    // let _user = sqlx::query_as::<_, Users>("select * from users where id = $1")
    //     .bind(password.id.clone()).fetch_one(pool).await.unwrap();
    // if encryption::md5(&password.old) != _user.password.unwrap() {
    //     return 0;
    // }
    // let pg_done = sqlx::query("update users set password = $1 where id = $2")
    //     .bind(encryption::md5(&password.new))
    //     .bind(password.id.clone())
    //     .execute(pool)
    //     .await.unwrap();
    // return pg_done.rows_affected();
}

/**
 * 用户接口: 修改用户信息
 */
pub async fn update_user_info(user: Users) -> u64 {
    let pool = c::sqlx_pool();
    let pg_done = sqlx::query("update users set nickname = $1, email = $2, phone = $3 where id = $4")
        .bind(&user.nickname)
        .bind(&user.email)
        .bind(&user.phone)
        .bind(&user.id)
        .execute(pool)
        .await.unwrap();
    // println!("Hello {}! id: {}. result: {:?}", user.nickname.unwrap(), user.id.unwrap(), pg_done.rows_affected());
    return pg_done.rows_affected();
}

/**
 * 用户接口: 删
 */
pub async fn delete(id: i64) -> u64 {
    RB.remove_by_column::<Users, _>("id", &id).await.unwrap()
}