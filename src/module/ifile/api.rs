use actix_files::NamedFile;
use actix_web::{HttpRequest, HttpResponse, Responder, Result};
use actix_web::{delete, get, post, put};
use actix_web::http::header::{ContentDisposition, DispositionParam, DispositionType};

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
