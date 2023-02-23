use actix_web::dev::HttpServiceFactory;
use actix_web::web;

pub mod utils;
pub mod ifile;
pub mod filesystem;
pub mod init;
pub mod samba;
pub mod user;
pub mod auth;
pub mod daisy;

pub fn api_routes() -> impl HttpServiceFactory {
    web::scope("")   // xxx or ""
        .service(ifile::api::index)
        .service(ifile::api::search)
        .service(ifile::api::list)
        .service(ifile::api::show)
        .service(ifile::api::visit)
        .service(ifile::api::download)
        .service(auth::api::login)
        .service(user::api::get)
        .service(user::api::put)
        .service(user::api::pwd)
        .service(user::api::list)
        .service(user::api::create)
        .service(user::api::update)
        .service(user::api::user_check)
}

pub async fn start() {
    samba::init_smb_account().await;    // 1.初始化 Smb账户
    init::daemon().await                // 2.初始化 后台守护服务
}
