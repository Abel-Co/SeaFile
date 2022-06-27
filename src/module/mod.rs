pub mod auth;
pub mod signup;
pub mod user;
pub mod email;
pub mod utils;

pub mod handler {
    use actix_web::dev::HttpServiceFactory;
    use actix_web::web;

    use crate::module::{auth, user};

    pub fn api_routes() -> impl HttpServiceFactory {
        web::scope("")
            .service(auth::api::login)
            .service(auth::api::activate)
            .service(user::api::list)
            .service(user::api::show)
            .service(user::api::update)
            .service(user::api::delete)
            .service(user::api::update_user_info)
    }
}
