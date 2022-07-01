use actix_web::{HttpRequest, HttpResponse, Responder, Result};
use actix_web::{delete, get, post, put};
use actix_web::web::Path;
use actix_files::NamedFile;

// /**
//  * 用户接口: 查
//  */
// #[get("/file/{id}")]
// pub async fn show(req: HttpRequest) ->
//                                     // impl Responder {
//                                        Result<NamedFile> {
//     // let user_info = user::bs::show(id).await;
//     // let info = id;
//     let info = 0;
//     // HttpResponse::Ok().json(info)
//     // fs::StaticFiles::new("public").unwrap()
//     Ok(NamedFile::open("/Users/Abel/Nutstore\\ Files/我的坚果云/坚果云/资料/房地产选址思路.txt")?)
// }
