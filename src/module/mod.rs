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
        .service(auth::api::login_check)
        .service(user::api::get)
        .service(user::api::put)
        .service(user::api::pwd)
        .service(user::api::list)
        .service(user::api::create)
        .service(user::api::update)
        .service(user::api::user_check)
        .service(user::api::delete)
        .service(daisy::api::user)
        .service(daisy::api::system)
}

pub async fn start() {
    auth::init_crypto_conf().await;     // 1.初始化密码学组件
    auth::init_naive_account().await;   // 2.创建初始账号（依赖 1）
    samba::init_smb_account().await;    // 3.初始化 Smb账户
    init::daemon().await;               // 4.初始化 后台守护服务
}
