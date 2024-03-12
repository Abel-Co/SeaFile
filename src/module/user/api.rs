use actix_web::{delete, get, HttpResponse, post, put, Responder};
use actix_web::web::Json;
use actix_web_lab::extract::Path;

use crate::boot::middleware::JwtToken;
use crate::module::{auth, user};
use crate::module::auth::Login;
use crate::module::user::{Password, Users, UserType};

/// 用户接口
/**
 * 当前用户
 */
#[get("/user")]
pub async fn get(jwt: JwtToken) -> impl Responder {
    if let Some(mut user) = user::bs::get(jwt.sub).await {
        user.password = None;
        return HttpResponse::Ok().json(user);
    }
    HttpResponse::Ok().json("none")
}

/**
 * 更新用户
 */
#[put("/user/self")]
pub async fn put(mut user: Json<Users>, jwt: JwtToken) -> impl Responder {
    // log::info!(">>> {:?} => {:?}: {:?}", jwt.sub, id.0, user.0);
    user.password = None;
    let _ = user::bs::update(jwt.sub, user.0).await;
    HttpResponse::Ok().json("Ok")
}

#[put("/user/pwd")]
pub async fn pwd(pwd: Json<Password>, jwt: JwtToken) -> impl Responder {
    let cnt = user::bs::update_pwd(jwt.sub, &pwd.old, &pwd.new).await;
    HttpResponse::Ok().json(cnt)
}

/// 管理员接口
/**
 * 用户列表
 */
#[get("/user/list")]
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
    HttpResponse::Ok().json(200)
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
    HttpResponse::Ok().json(200)
}

/**
 * 检查用户
 */
#[get("/user/check/{username}")]
pub async fn user_check(username: Path<String>, jwt: JwtToken) -> impl Responder {
    if let Some(subject) = auth::bs::get_subject(jwt.sub).await {
        if subject.user_type == UserType::Admin {   // 验证管理员权限
            let data = user::bs::get_by_username_ignore_case(&username.0).await;
            return HttpResponse::Ok().json(data.len());
        }
    }
    HttpResponse::BadRequest().json("权限不足")
}

/**
 * 管理员删除用户
 */
#[delete("/user/{id}")]
pub async fn delete(id: Path<i64>, login: Json<Login>, jwt: JwtToken) -> impl Responder {
    if let Some(subject) = auth::bs::get_subject(jwt.sub).await {
        if subject.user_type == UserType::Admin     // 验证管理员权限
            && login.verify_aes(subject.password.as_ref().unwrap()) {
            let cnt = user::bs::delete(id.0).await;
            return HttpResponse::Ok().json(cnt);
        }
    }
    HttpResponse::BadRequest().json("权限不足")
}