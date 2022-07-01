pub mod utils;
pub mod file;

pub mod handler {
    use actix_files::Files;
    use actix_web::dev::HttpServiceFactory;
    use actix_web::web;

    use crate::module::{file};

    pub fn api_routes() -> impl HttpServiceFactory {
        web::scope("")
            // .service(user::api::update_user_info)
    }
}
