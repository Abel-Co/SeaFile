use actix_web::{HttpResponse, post, Responder};
use actix_web::http::StatusCode;
use actix_web::web::Json;
use validator::Validate;

use crate::boot::middleware::JwtToken;
use crate::module::{auth, ifile};
use crate::module::auth::{Login, Subject};

/**
 * 登录
 */
#[post("/login")]
pub async fn login(login: Json<Login>) -> impl Responder {
    if let Err(e) = login.validate() {
        return HttpResponse::BadRequest().json(e);
    }
    match auth::bs::login(login.into_inner()).await {
        Ok(user) => {
            ifile::bs::calc_folder(user.username.as_ref().unwrap()).await; // 校正/预热'档案',仅db,无 disk io
            let jwt_token = JwtToken::from_id(user.id.unwrap());
            HttpResponse::Ok().json(Subject::new(&user.username.unwrap(), jwt_token))
        }
        Err(code) => match code {
            419 => HttpResponse::build(StatusCode::from_u16(419).unwrap()).json("账号已冻结！"),
            423 => HttpResponse::build(StatusCode::LOCKED).json("账号待激活！"),
            401 => HttpResponse::Unauthorized().json("账号或密码错误！"),
            _ => HttpResponse::Unauthorized().json("账号或密码错误！")
        }
    }
}
