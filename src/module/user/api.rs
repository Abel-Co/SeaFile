use actix_web::{HttpResponse, Responder};
#[allow(unused)]
use actix_web::{delete, get, post, put};
use actix_web::web::Json;
use actix_web_lab::extract::Path;

use crate::boot::middleware::JwtToken;
use crate::module::{auth, user};
use crate::module::user::{Users, UserType};

/**
 * 用户列表
 */
#[get("/user")]
pub async fn list(jwt: JwtToken) -> impl Responder {
    if let Some(subject) = auth::bs::get_subject(jwt.sub).await {
        if subject.user_type == UserType::Admin {   // 验证管理员权限
            let users = user::bs::list().await;
            return HttpResponse::Ok().json(users);
        }
    }
    HttpResponse::BadRequest().json("权限不足")
}

/**
 * 创建用户
 */
#[post("/user")]
pub async fn create(user: Json<Users>, jwt: JwtToken) -> impl Responder {
    // log::info!(">>> {:?} => {:?}", jwt.sub, user);
    if let Some(subject) = auth::bs::get_subject(jwt.sub).await {
        if subject.user_type == UserType::Admin {   // 验证管理员权限
            let _ = user::bs::create(user.0).await;
        }
    }
    HttpResponse::Ok()
}

/**
 * 更新用户
 */
#[put("/user/{id}")]
pub async fn update(id: Path<i64>, user: Json<Users>, jwt: JwtToken) -> impl Responder {
    // log::info!(">>> {:?} => {:?}: {:?}", jwt.sub, id.0, user.0);
    if Some(id.0) == user.id {
        if let Some(subject) = auth::bs::get_subject(jwt.sub).await {
            if subject.user_type == UserType::Admin {   // 验证管理员权限
                let _ = user::bs::update(id.0, user.0).await;
            }
        }
    }
    HttpResponse::Ok()
}

