use actix_web::{HttpResponse, Responder};
use actix_web::get;
use actix_web_lab::extract::Path;

use crate::boot::c::HOME;
use crate::boot::middleware::JwtToken;
use crate::module::{auth, daisy, ifile};
use crate::module::ifile::Files;
use crate::module::user::UserType;

/**
 * DaisyDisk视图 - 用户级
 */
#[get("/daisy/user/{id}")]
pub async fn user(id: Path<i64>, jwt: JwtToken) -> impl Responder {
    if let Some(subject) = auth::bs::get_subject(jwt.sub).await {
        let path = format!("{}/{}", HOME.as_str(), subject.username.as_ref().unwrap());
        let (file, level) = daisy_base(id.0, &path).await;
        let list = daisy::bs::daisy_user(file.id, level).await;
        return HttpResponse::Ok().json(list);
    }
    return HttpResponse::BadRequest().json("未知用户");
}

// 用户、or 管理员 权限的'根'
pub async fn daisy_base(id: i64, home: &str) -> (Files, isize) {
    let file = ifile::bs::get(id).await;
    let file = if file.as_ref().is_some() && file.as_ref().unwrap().path.starts_with(home) {
        file.unwrap()
    } else {
        ifile::bs::check_path(home).await.unwrap()
    };
    let level = (file.path.replace(home, "").split("/").collect::<Vec<_>>().len() - 1) as isize;
    (file, level)
}

/**
 * DaisyDisk视图 - 系统级
 */
#[get("/daisy/system/{id}")]
pub async fn system(id: Path<i64>, jwt: JwtToken) -> impl Responder {
    if let Some(subject) = auth::bs::get_subject(jwt.sub).await {
        if subject.user_type == UserType::Admin {   // 验证管理员权限
            let path = format!("{}", HOME.as_str());
            let (file, level) = daisy_base(id.0, &path).await;
            let list = daisy::bs::daisy_user(file.id, level).await;
            return HttpResponse::Ok().json(list);
        }
    }
    return HttpResponse::BadRequest().json("未知用户");
}