use rbatis::crud::{CRUD, Skip};
use rbatis::snowflake::new_snowflake_id;
use rbson::Bson;

use crate::boot::c::RB;
use crate::module::user;
use crate::module::user::{Users, UserType};

/**
 * 按 username 获取用户
 */
pub async fn get_by_username(username: &str) -> Option<Users> {
    RB.fetch_by_wrapper(RB.new_wrapper().eq("username", username)).await.unwrap()

    // Some(Users { id: 123456, status: Some(1), username: Some(String::from("abel")), password: Some("123456".to_string()), ..Default::default() })
}


/// 模拟创建账号
#[allow(unused)]
async fn test_create() -> Option<Users> {
    let user = Users {
        id: new_snowflake_id(),
        username: Some("abel".to_string()),
        password: Some(user::passhash("123456")),
        user_type: UserType::Admin,
        status: Some(1),
        ..Default::default()
    };
    let rb_resp = RB.save(&user, &[Skip::Value(Bson::Null)]).await;
    let _ = rb_resp.unwrap().rows_affected;

    let user = Users {
        id: new_snowflake_id(),
        username: Some("xugy".to_string()),
        password: Some("123456".to_string()),
        user_type: UserType::User,
        status: Some(1),
        ..Default::default()
    };
    let rb_resp = RB.save(&user, &[Skip::Value(Bson::Null)]).await;
    let _ = rb_resp.unwrap().rows_affected;

    Some(user)
}