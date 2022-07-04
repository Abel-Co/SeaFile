pub mod utils;
pub mod file;
pub mod filesystem;

pub mod handler {
    use actix_web::dev::HttpServiceFactory;
    use actix_web::web;

    use crate::module::file;

    pub fn api_routes() -> impl HttpServiceFactory {
        web::scope("")
            .service(file::api::download)
    }
}
