use actix_files::NamedFile;
use actix_web::{HttpResponse, Responder, Result};
#[allow(unused)]
use actix_web::{delete, get, post, put};
use actix_web::http::header::{ContentDisposition, DispositionParam, DispositionType};
use actix_web_lab::extract::Path;

use crate::module::ifile;

#[get("/search/{name}")]
pub async fn search(Path(name): Path<String>) -> impl Responder {
    let files = ifile::dao::search(&name).await;
    HttpResponse::Ok().json(files)
}

#[get("/list/{parent}")]
pub async fn list(Path(parent): Path<i64>) -> impl Responder {
    let files = ifile::dao::list(parent).await;
    HttpResponse::Ok().json(files)
}

#[get("/show/{id}/{name}")]
pub async fn show(Path((id, name)): Path<(i64, String)>) -> Result<impl Responder> {
    log::info!("{}, {}", id, name);
    Ok(HttpResponse::Ok())
}

#[get("/download")]
pub async fn download() -> Result<impl Responder> {
    let file = NamedFile::open("Cargo.toml")?;
    // let ifile = NamedFile::open("/Users/Abel/Downloads/知否知否应是绿肥红瘦.The.Story.of.Ming.Lan.2018.E15.1080p.WEB-DL.x264.AAC-HotWEB.mp4")?;
    Ok(file
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![DispositionParam::Filename("Cargo.mp4".to_string())],
        }))
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
