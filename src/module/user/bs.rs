use rbatis::plugin::snowflake::new_snowflake_id;

use crate::module::user;
use crate::module::user::{Password, Users};

/**
 * 用户接口: 查 列表
 */
pub async fn list() -> Vec<Users> {
    let user = user::dao::list().await;
    return user;
}

/**
 * 用户接口: 增
 */
pub async fn create(mut user: Users) -> Users {
    user.id = Some(new_snowflake_id());
    if let Some(email) = &user.email {
        let email = user.email.as_ref().unwrap();
        let nickname = email.split("@").collect::<Vec<_>>().get(0).map(|s| s.trim());
        user.nickname = Some(nickname.unwrap().to_string());
    }
    user::dao::create(&user).await;
    user
}

/**
 * 用户接口: 查
 */
pub async fn show(id: i64) -> Users {
    user::dao::show(id).await
}

/**
 * 用户接口: 改
 */
pub async fn update(password: Password) -> u64 {
    user::dao::update(password).await
}

/**
 * 用户接口: 改
 */
pub async fn update_user_info(user: Users) -> u64 {
    user::dao::update_user_info(user).await
}

/**
 * 用户接口: 删
 */
pub async fn delete(id: i64) -> u64 {
    user::dao::delete(id).await
}


// -------------------------------------------------------------------------------------------------
#[cfg(test)]
pub mod user_bs_test {
    use rbatis::crud::{CRUD, Skip};
    use rbatis::TimestampZ;
    use rbson::Bson;

    use crate::boot;
    use crate::boot::c::RB;
    use crate::module::utils;

    /**
     * 测试更新 Activate
     */
    #[tokio::test]
    pub async fn test_update_activate() {
        boot::start().await;
        let activate = &crate::module::user::Activate {
            id: Some(318079580667777024),
            user_id: Some(318079580470644736),
            contact: Some("aurainc@126.com".to_string()),
            code: Some(utils::random::get_rand_code(6)),
            content: Some("您的验证码是：\n\t\tOagd18".to_string()),
            created_at: Some(TimestampZ::now()),
            updated_at: Some(TimestampZ::now()),
            ..Default::default()
        };
        RB.save(activate, &[Skip::Value(Bson::Null)]).await;
        RB.update_by_column("id", activate).await;
    }
}