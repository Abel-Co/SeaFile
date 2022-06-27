// use actix_web::{App, http::{header, Method}, fs,error,middleware,middleware::cors::Cors,actix::Addr};
//
// use api::{home::index, auth::{signup, signin}};
// use api::info::ruster_info;
// use api::admin::{admin_users,admin_themes};
// use api::theme::{theme_page_list, theme_and_comments, theme_new, theme_add_comment,blog_save,blog_like,best_person};
// use api::category::{categorys, category_new, category_theme_page_list};
// use api::user::{user_info, user_delete, user_id,user_update,user_update_img,user_themes,user_comments,user_saves,user_messages,user_messages_readall};
// use model::db::{ ConnDsl, init };
// use share::common::AppState;
//
// pub fn app_state(addr: Addr<ConnDsl>) -> App<AppState> {
//
//     App::with_state(AppState{ db: addr.clone()})
//         .middleware(middleware::Logger::default())
//         .resource("/", |r| r.f(index))
//         .resource(r"/a/{tail:.*}", |r| r.f(index))
//         .configure(|app| Cors::for_app(app)
//             .allowed_methods(vec!["GET", "POST"])
//             .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
//             .allowed_header(header::CONTENT_TYPE)
//             .max_age(3600)
//             .resource("/user/signup", |r| { r.method(Method::POST).with(signup); })
//             .resource("/user/signin", |r| { r.method(Method::POST).with(signin); })
//             .resource("/api/user_info", |r| { r.method(Method::GET).with(user_info); })
//             .resource("/api/user_id", |r| { r.method(Method::POST).with(user_id); })
//             .resource("/api/user_delete", |r| { r.method(Method::GET).with(user_delete); })
//             .resource("/api/user_update", |r| { r.method(Method::POST).with(user_update); })
//             .resource("/api/user_update_img", |r| { r.method(Method::POST).with(user_update_img); })
//             .resource("/api/user/id/themes", |r| { r.method(Method::POST).with(user_themes); })
//             .resource("/api/user/id/comments", |r| { r.method(Method::POST).with(user_comments); })
//             .resource("/api/user/id/saves", |r| { r.method(Method::POST).with(user_saves); })
//             .resource("/api/user/id/messages", |r| { r.method(Method::POST).with(user_messages); })
//             .resource("/api/user/id/messages/readall", |r| { r.method(Method::POST).with(user_messages_readall); })
//             .resource("/api/theme_list/page_id", |r| { r.method(Method::POST).with(theme_page_list); })
//             .resource("/api/theme_new", |r| { r.method(Method::POST).with(theme_new); })
//             .resource("/api/home/bestperson", |r| { r.method(Method::GET).with(best_person); })
//             .resource("/api/ruster/info", |r| { r.method(Method::GET).with(ruster_info); })
//             .resource("/api/home/category_list/page_id", |r| { r.method(Method::POST).with(category_theme_page_list); })
//             .resource("/api/categorys", |r| { r.method(Method::GET).with(categorys); })
//             .resource("/api/category_new", |r| { r.method(Method::POST).with(category_new); })
//             .resource("/api/blog/save", |r| { r.method(Method::POST).with(blog_save); })
//             .resource("/api/blog/like", |r| { r.method(Method::POST).with(blog_like); })
//             .resource("/api/theme/{theme_id}", |r| {
//                 r.method(Method::GET).with(theme_and_comments);
//                 r.method(Method::POST).with(theme_add_comment);
//             })
//             .resource("/api/admin/users", |r| { r.method(Method::GET).with(admin_users); })
//             .resource("/api/admin/themes", |r| { r.method(Method::GET).with(admin_themes); })
//             .register())
//         .handler("/", fs::StaticFiles::new("public").unwrap())
// }
