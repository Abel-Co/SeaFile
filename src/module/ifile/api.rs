use actix_files::NamedFile;
use actix_web::{error, get, HttpResponse, Responder, Result};
use actix_web::http::header::{ContentDisposition, DispositionParam, DispositionType};
use actix_web_lab::extract::Path;

use crate::boot::middleware::JwtToken;
use crate::module::ifile;

/**
 * 目录树浏览
 */
#[get("/list/{parent}")]
pub async fn list(Path(parent): Path<i64>, jwt: JwtToken) -> impl Responder {
    let files = ifile::bs::list(jwt.sub, parent).await;
    HttpResponse::Ok().json(files)
}

/**
 * 构建索引
 */
#[get("/index/{id}/{_name}")]
pub async fn index(Path((id, _name)): Path<(i64, String)>, jwt: JwtToken) -> impl Responder {
    ifile::bs::reindex(jwt.sub, id).await;
    HttpResponse::Ok().json("Ok")
}

/**
 * 文件搜索
 */
#[get("/search/{query}")]
pub async fn search(Path(query): Path<String>, jwt: JwtToken) -> impl Responder {
    let files = ifile::bs::search(jwt.sub, &query).await;
    HttpResponse::Ok().json(files)
}

/**
 * 读取文件
 */
#[get("/show/{id}/{_name}")]
pub async fn show(Path((id, _name)): Path<(i64, String)>) -> impl Responder {
    let content = ifile::bs::show(id).await;
    HttpResponse::Ok().json(content)
}

/**
 * 原生访问
 */
#[get("/visit/{id}/{_name}")]
pub async fn visit(Path((id, _name)): Path<(i64, String)>) -> impl Responder {
    match ifile::bs::get(id).await {
        // Some(_file) => NamedFile::open_async(_file.path).await,  // txt、html 匀可，唯 pdf 仍下载
        Some(_file) => {
            let file = NamedFile::open_async(_file.path).await?;
            Ok(file.set_content_disposition(ContentDisposition {
                disposition: DispositionType::Inline,
                parameters: vec![DispositionParam::Filename(_file.name)],
            }))
        }
        None => NamedFile::open_async("").await
    }
}

/**
 * 下载档案
 */
#[get("/download/{id}/{_name}")]
pub async fn download(Path((id, _name)): Path<(i64, String)>) -> Result<impl Responder> {
    match ifile::bs::get(id).await {
        Some(_file) => {
            let file = NamedFile::open_async(_file.path).await?;
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
