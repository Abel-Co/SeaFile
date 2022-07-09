pub mod utils;
pub mod ifile;
pub mod filesystem;

pub mod handler {
    use actix_web::dev::HttpServiceFactory;
    use actix_web::web;

    use crate::module::ifile;

    pub fn api_routes() -> impl HttpServiceFactory {
        web::scope("")
            .service(ifile::api::search)
            .service(ifile::api::list)
            .service(ifile::api::show)
            .service(ifile::api::download)
    }
}
