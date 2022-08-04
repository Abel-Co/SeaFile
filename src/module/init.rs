use notify::event::CreateKind;
use rbatis::crud::CRUD;

use crate::boot::c::RB;
use crate::boot::global;
use crate::module::ifile;
use crate::module::ifile::Files;

pub async fn decide_to_init() {
    let tables: i64 = RB.fetch("select count(*) from pg_tables where tablename = 'files';", vec![]).await.unwrap();
    if tables == 0 {
        // 1.建表
        let sql = std::fs::read_to_string("scripts/create.sql").unwrap();
        let _ = RB.exec(sql.as_str(), vec![]).await;
        // 2.记录管理路径
        let watch_path = global().watch_path.as_ref().unwrap();
        ifile::bs::save_or_update(CreateKind::Folder, watch_path).await;
    }
    // let files: i64 = RB.fetch("select count(*) from files where path = ?;", vec![global().watch_path.as_ref().unwrap()]).await.unwrap();
    // if files < 2 {
    let root_path = RB.fetch_by_wrapper::<Option<Files>>(RB.new_wrapper().eq("path", global().watch_path.as_ref().unwrap())).await.unwrap();
    if let None = root_path {
        // 3.初始建立索引
        let watch_path = global().watch_path.as_ref().unwrap();
        ifile::bs::index(watch_path).await;
    }
}