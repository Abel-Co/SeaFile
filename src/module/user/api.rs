use actix_web::{HttpResponse, Responder};
use actix_web::{delete, get, post, put};
use actix_web::web::{Json, Path};

use crate::module::user;
use crate::module::user::{Password, Users};

// -------------------------------------------------------------------------------------------------
/**
 * 用户接口: 查
 */
#[get("/user/list")]
pub async fn list() -> impl Responder {
    let users = user::bs::list().await;
    HttpResponse::Ok().json(users)
}

/**
 * 用户接口: 增
 */
#[post("/user/create")]
pub async fn create(user: Json<Users>) -> impl Responder {
    let user = user::bs::create(user.0).await;
    HttpResponse::Ok().json(user)
}

/**
 * 用户接口: 查
 */
#[get("/user/{id}")]
pub async fn show(Path(id): Path<i64>) -> impl Responder {
    let user_info = user::bs::show(id).await;
    HttpResponse::Ok().json(user_info)
}

/**
 * 用户接口: 修改用户密码
 */
#[put("/user/modify/password")]
pub async fn update(password: Json<Password>) -> impl Responder {
    let rows = user::bs::update(password.into_inner()).await;
    HttpResponse::Ok().json(rows)
}

/**
 * 用户接口: 修改用户信息
 */
#[put("/user/update/info")]
pub async fn update_user_info(user: Json<Users>) -> impl Responder {
    let rows = user::bs::update_user_info(user.into_inner()).await;
    HttpResponse::Ok().json(rows)
}

/**
 * 用户接口: 删
 */
#[delete("/user/{id}")]
pub async fn delete(Path(id): Path<i64>) -> impl Responder {
    let rows = user::bs::delete(id).await;
    HttpResponse::Ok().json(rows)
}
