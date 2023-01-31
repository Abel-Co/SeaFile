use actix_files::Files;
use actix_web::{App, HttpServer, middleware};

use seafile::{boot, module};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    boot::start().await;
    HttpServer::new(move || App::new()
        .wrap(middleware::Compress::default())
        .wrap(middleware::Logger::default())
        .wrap(boot::middleware::Auth)
        .service(module::router::api_routes())
        .default_service(Files::new("/", "dist/").index_file("index.html"))
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
