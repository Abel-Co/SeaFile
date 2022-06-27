// #![allow(warnings)]

// // #[macro_use] extern crate diesel;
// // #[macro_use] extern crate serde_derive;
// extern crate serde;
// extern crate serde_json;
// extern crate futures;
// // extern crate num_cpus;
// extern crate actix_web;
// // extern crate env_logger;
// // extern crate dotenv;
// extern crate chrono;
// extern crate bcrypt;
// extern crate regex;
// // extern crate http;
// // extern crate postgres;
// // extern crate timeago;
// // extern crate pulldown_cmark;
// // extern crate openssl;
// extern crate jsonwebtoken as jwt;
// // extern crate md5;
// // extern crate ring;

// use actix_web::{server,actix::System};
// use crate::model::db::init;
// // use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

// // mod api;
// // mod handler;
// // mod model;
// // mod share;
// // mod utils;
// mod router;

use actix_web::{App, http, HttpServer, middleware};

use sea::{boot, module};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    boot::start().await;
    HttpServer::new(move || App::new()
        .wrap(middleware::Logger::default())
        .wrap(middleware::Compress::new(http::ContentEncoding::Br))
        .wrap(boot::middleware::Auth)
        .service(module::handler::api_routes())
    ).bind(boot::global().addr())?.run().await
}

// 0. 以下修改，以 rust-demo 进行                        OK
// 1. 集成日志 log4rs                                   OK
// 2. HttpServer::new().await 后加 ?，下一行，打日志      Fail
// 3. module::handler::api_routes() 迁入 boot          Fail -> 日志初始化，要在 HttpServer::new() 之前
// 4. 增加 header 操作中间件                             OK
// 5. 增加 入口日志 中间件                                OK
// 6. 私有内容，整体封装入 boot                           Over
// 7. 描述 config.yaml 的位置，易于修改                   Over

// fn main() {
//     ::std::env::set_var("RUST_LOG", "rustlang-cn=info");
//     ::std::env::set_var("RUST_BACKTRACE", "1");
//     env_logger::init();
//     // let sys = System::new("rustlang-cn");
//     let addr = init();

//     // let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
//     // builder.set_private_key_file("privkey.pem", SslFiletype::PEM).unwrap();
//     // builder.set_certificate_chain_file("fullchain.pem").unwrap();

//     server::new( move || router::app_state(addr.clone()))
//          .bind("localhost:8000").unwrap()
//          .shutdown_timeout(2)
//          .start();

//     sys.run();
// }

