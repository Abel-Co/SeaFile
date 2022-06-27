use rbatis::snowflake::new_snowflake_id;
use rbatis::TimestampZ;

use crate::module::{signup, user};
use crate::module::user::{Activate, Users};

/**
 * 创建待激活信息
 */
pub async fn save_activate(user_id: i64, contact: &String) -> Activate {
    let mut activate = signup::dao::find_activate(user_id, contact).await.unwrap_or(Activate::new(user_id, contact));
    if let Some(id) = activate.id {
        let now = TimestampZ::now().timestamp_millis(); // mac 下时间戳、时区，可能错误，Linux下就好了。
        let updated_at = activate.updated_at.unwrap().timestamp_millis();
        if now - updated_at > 10 * 60 * 1000 {  // 大于 10分钟
            let update_cnt = signup::dao::update_activate(id, &activate.update()).await;
            assert_eq!(update_cnt, 1, "更新用户验证码失败！{:?}", activate)
        }
    } else {
        activate.id = Some(new_snowflake_id());
        signup::dao::create_activate(&activate).await;
    }
    activate
}

/**
 * get激活信息
 */
pub async fn get_activate(code: &str) -> Option<Activate> {
    signup::dao::get_activate(code).await
}

/**
 * 账号激活
 */
pub async fn activate(act_info: Activate) -> Users {
    log::debug!(">>> {:?}", act_info);
    signup::dao::activate(act_info.user_id.unwrap()).await;
    signup::dao::delete_activate(act_info.id.unwrap()).await;
    user::dao::show(act_info.user_id.unwrap()).await
}

pub mod signup_bs_test {
    #![allow(unused_imports)]
    use chrono::{FixedOffset, Local};
    use rbatis::TimestampZ;

    #[test]
    pub fn test_timestamp() {
        let now = TimestampZ::now_local();
        let now2 = Local::now();
        // let now2 = now2.with_timezone(&FixedOffset::east(8*3600));
        println!("now: {}, now2: {}", now, now2);
        println!("now: {}, now2: {}", now.timezone(), now2);
        println!("now: {}, now2: {}", now.timestamp_millis(), now2.timestamp_millis());
    }
}