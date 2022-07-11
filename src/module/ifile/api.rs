use actix_files::NamedFile;
use actix_web::{error, HttpResponse, Responder, Result};
#[allow(unused)]
use actix_web::{delete, get, post, put};
use actix_web::http::header::{ContentDisposition, DispositionParam, DispositionType};
use actix_web_lab::extract::Path;

use crate::module::ifile;

#[get("/search/{name}")]
pub async fn search(Path(name): Path<String>) -> impl Responder {
    let files = ifile::bs::search(&name).await;
    HttpResponse::Ok().json(files)
}

#[get("/list/{parent}")]
pub async fn list(Path(parent): Path<i64>) -> impl Responder {
    let files = ifile::bs::list(parent).await;
    HttpResponse::Ok().json(files)
}

#[get("/show/{id}/{name}")]
pub async fn show(Path((id, name)): Path<(i64, String)>) -> impl Responder {
    let content = ifile::bs::show(id).await;
    HttpResponse::Ok().json(content)
}

#[get("/visit/{id}/{name}")]
pub async fn visit(Path((id, name)): Path<(i64, String)>) -> impl Responder {
    match ifile::bs::get(id).await {
        Some(_file) => NamedFile::open_async(_file.path).await,
        None => NamedFile::open_async("").await
    }
}

#[get("/download/{id}/{name}")]
pub async fn download(Path((id, name)): Path<(i64, String)>) -> Result<impl Responder> {
    match ifile::bs::get(id).await {
        Some(_file) => {
            let file = NamedFile::open(_file.path)?;
            Ok(file
                .use_last_modified(true)
                .set_content_disposition(ContentDisposition {
                    disposition: DispositionType::Attachment,
                    parameters: vec![DispositionParam::Filename(_file.name)],
                }))
        }
        None => Err(error::ErrorBadRequest("资源不存在！"))
    }
}


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
