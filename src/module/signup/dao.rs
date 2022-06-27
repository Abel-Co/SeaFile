use rbatis::crud::{CRUD, Skip};
use rbson::Bson;

use crate::boot::c::RB;
use crate::module::user::{Activate, Users};

/**
 * 创建用户待激活
 */
pub async fn create_activate(activate: &Activate) -> Option<i64> {
    RB.save(activate, &[Skip::Value(Bson::Null)]).await.unwrap().last_insert_id
}

/**
 * 用户接口: 查
 */
pub async fn find_activate(user_id: i64, contact: &String) -> Option<Activate> {
    RB.fetch_by_wrapper(RB.new_wrapper().eq("user_id", user_id).eq("contact", contact)).await.unwrap()
}

/**
 * get用户待激活信息
 */
pub async fn get_activate(code: &str) -> Option<Activate> {
    RB.fetch_by_column("code", code).await.unwrap()
}

/**
 * 更新用户待激活
 */
pub async fn update_activate(id: i64, activate: &Activate) -> u64 {
    RB.update_by_column("id", activate).await.unwrap()
}

/**
 * 用户激活
 */
pub async fn activate(user_id: i64) -> u64 {
    let users = Users { id: Some(user_id), status: Some(1), ..Default::default() };
    RB.update_by_wrapper(&users, RB.new_wrapper().eq("id", &user_id), &[Skip::Value(Bson::Null)]).await.unwrap()
}

/**
 * 用户激活
 */
pub async fn delete_activate(id: i64) -> u64 {
    RB.remove_by_column::<Activate, _>("id", &id).await.unwrap()
}
