use actix_web::{HttpResponse, Responder};
#[allow(unused)]
use actix_web::{delete, get, post, put};
use actix_web::http::StatusCode;
use actix_web::web::Json;
use validator::Validate;

use crate::boot::middleware::JwtToken;
use crate::module::user;
use crate::module::user::Login;

#[post("/login")]
pub async fn login(login: Json<Login>) -> impl Responder {
    if let Err(e) = login.validate() {
        return HttpResponse::BadRequest().json(e);
    }
    match user::bs::login(login.into_inner()).await {
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