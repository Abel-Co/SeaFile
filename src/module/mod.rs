pub mod utils;
pub mod file;

pub mod handler {
    use actix_files::Files;
    use actix_web::dev::HttpServiceFactory;
    use actix_web::web;

    use crate::module::{file};

    pub fn api_routes() -> impl HttpServiceFactory {
        web::scope("")
            // .service(auth::api::login)
            // .service(auth::api::activate)
            // .service(user::api::list)
            // .service(user::api::show)
            // .service(user::api::update)
            // .service(user::api::delete)
            // .service(user::api::update_user_info)
            .service(Files::new("/", "dist/").index_file("index.html"))
    }
}
