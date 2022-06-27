use futures::executor::block_on;
use reqwest::StatusCode;

use crate::module::{auth, email, signup, user};
use crate::module::auth::Login;
use crate::module::user::{Activate, Users};

/**
 * 用户登录
 */
pub async fn login(login: Login) -> Result<Users, u16> {
    match user::dao::get_by_email(login.username.as_ref().unwrap()).await {
        Some(user) => {
            match user.status {
                Some(1) => {
                    if login.verify(user.password.as_ref().unwrap()).unwrap() { // duration: debug 2589, release 280 !!
                        Ok(user)
                    } else {
                        Err(StatusCode::UNAUTHORIZED.as_u16())  // 密码验证，不通过 ！！
                    }
                }
                Some(0) => {
                    do_signup(user).await;
                    Err(StatusCode::LOCKED.as_u16())    // 待激活 http 423 （同资源锁定）
                }
                Some(-1) => Err(419),                // 账号冻结 http 419 （七牛扩展状态码）
                _ => {
                    log::error!("[用户登录]-状态未知: {:?}", user);
                    Err(StatusCode::UNAUTHORIZED.as_u16())  // 其他匀按登录失败处理
                }
            }
        }
        None => {
            do_signup(Users::from_login(login)).await;
            Err(StatusCode::LOCKED.as_u16())    // 待激活 http 423 （同资源锁定）
        }
    }
}

/**
 * 用户注册
 */
pub async fn do_signup(mut user: Users) -> Result<(), String> {
    assert_eq!(user.status, Some(0));
    let email_addr = user.email.clone().unwrap();
    let user_id = user.id.unwrap_or_else(|| block_on({
        user.password = Some(auth::passhash(&user.password.unwrap()));
        user::bs::create(user)
    }).id.unwrap());
    let activate = signup::bs::save_activate(user_id, &email_addr).await;
    log::debug!("{:?}", activate);
    let email = email::Email {
        to: email_addr,
        title: "鸭个梨验证码".to_string(),
        body: activate.content.unwrap().to_string(),
    };
    email::send(&email)
}

/**
 * 账号激活
 */
pub async fn activate(act_info: Activate) -> Result<Users, String> {
    match signup::bs::get_activate(act_info.code.as_ref().unwrap()).await {
        Some(act) => {
            Ok(signup::bs::activate(act).await)
        }
        None => {
            log::warn!("用户激活失败，需要关注，激活码：{:?} ！！", act_info.code);
            // Err("激活失败，请稍后再试 ！！".to_string())
            Err("验证码错误 ！！".to_string())
        }
    }
}


// -------------------------------------------------------------------------------------------------
#[cfg(test)]
mod auth_bs_test {
    use rbatis::TimestampZ;
    use serde::{Deserialize, Serialize};
    use sqlx::FromRow;

    use crate::boot;

    #[tokio::test]
    pub async fn test_signup() {
        use crate::module::auth;
        use crate::module::auth::Login;
        use crate::module::user::{Users};

        boot::start().await;
        let login = Login {
            username: Some(String::from("aurainc@126.com")),
            password: Some(String::from("123456")),
        };
        let mut user = Users::from_login(login);
        user.id = Some(318079580470644736);
        auth::bs::do_signup(user).await;
    }

    // #[crud_table(formats_pg:"created_at:{}::timestamptz,updated_at:{}::timestamptz")]
    #[derive(CRUDTable, Debug, Default, Validate, Serialize, Deserialize, FromRow)]   // 增加了 sqlx::FromRow
    pub struct UserInfo {
        pub username: String,
        pub updated_at: Option<TimestampZ>,
        pub created_at: Option<TimestampZ>,
        pub deleted_at: Option<TimestampZ>,
    }

    #[tokio::test]
    pub async fn test_update_table() {
        #![allow(unused_imports)]
        use rbatis::crud::{CRUD, Skip};
        use rbson::{Bson};
        use crate::boot::c;

        boot::start().await;
        use crate::boot::c::RB;
        let user_info = UserInfo {
            username: String::from("XiaoMing"),
            created_at: Some(TimestampZ::now()),
            updated_at: Some(TimestampZ::now_local()),
            ..Default::default()
        };
        // let tim = chrono::DateTime::<Utc>::from_str("2022-01-05 03:18:49 UTC");
        // log::debug!("TimestampZ: {:?}", tim.unwrap());
        // let rows_affected = RB.save(&user_info, &[Skip::Value(Bson::Null)]).await.unwrap().rows_affected;
        let user_info: Option<UserInfo> = RB.fetch_by_column("username", "XiaoMing").await.unwrap();
        log::info!("{:?}", user_info);
        // let user_info = sqlx::query_as::<_, UserInfo>("select * from user_info").fetch_one(c::sqlx_pool()).await.unwrap();
    }
}