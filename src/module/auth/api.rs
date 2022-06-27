use actix_web::{HttpResponse, Responder};
use actix_web::{get, post};
use actix_web::http::StatusCode;
use actix_web::web::Json;
use validator::Validate;

use crate::boot::middleware::JwtToken;
use crate::module::auth;
use crate::module::auth::Login;
use crate::module::user::Activate;

/**
 * 登录接口
 * Jwt { claims: Claims { sub: user.id, exp: 30days } }
 */
#[post("/login")]
pub async fn login(login: Json<Login>) -> impl Responder {
    if let Err(e) = login.validate() {
        return HttpResponse::BadRequest().json(e);
    }
    match auth::bs::login(login.into_inner()).await {
        Ok(user) => {
            let jwt_token = JwtToken::from_user(user).create();
            HttpResponse::Ok().json(jwt_token.unwrap())
        }
        Err(code) => match code {
            419 => HttpResponse::build(StatusCode::from_u16(419).unwrap()).json("账号已冻结！"),
            423 => HttpResponse::build(StatusCode::LOCKED).json("账号待激活！"),
            401 => HttpResponse::Unauthorized().json("账号或密码错误！"),
            _ => HttpResponse::Unauthorized().json("账号或密码错误！")
        }
    }
}

/**
 * 注册账号
 */
#[get("/signup")]
pub async fn signup(act_info: Json<Activate>) -> impl Responder {
    HttpResponse::Ok()
}

/**
 * 账号激活
 */
#[get("/activate")]
pub async fn activate(act_info: Json<Activate>) -> impl Responder {
    log::debug!("signup: {:?}", act_info.0);
    if let Err(e) = act_info.validate() {
        return HttpResponse::BadRequest().json(e)
    }
    match auth::bs::activate(act_info.0).await {
        Ok(user) => {
            let jwt_token = JwtToken::from_user(user).create();
            HttpResponse::Ok().json(jwt_token.unwrap())
        }
        Err(e) => {
            HttpResponse::Unauthorized().json(e)
        }
    }
}
