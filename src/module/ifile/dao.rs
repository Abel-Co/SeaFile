use std::path;

use rbatis::crud::{CRUD, Skip};
use rbson::Bson;

use crate::boot::c::RB;
use crate::module::ifile::{Depth, Files};
use crate::module::utils::crc_utils::crc_i64;

pub async fn get(id: i64) -> Option<Files> {
    RB.fetch_by_column("id", id).await.unwrap()
}

pub async fn get_by_path(path: &str) -> Option<Files> {
    RB.fetch_by_wrapper(
        RB.new_wrapper().eq("crc", crc_i64(path)).eq("path", path)
    ).await.unwrap()
}

/**
 * 档案搜索
 */
pub async fn search(path: &str, query: &str) -> Vec<Files> {
    let (path, query) = ([path, "/%"].concat(), ["%", query, "%"].concat());
    RB.fetch("select * from files where path like $1 and name ilike $2 order by kind desc;",
             vec![Bson::String(path), Bson::String(query)]).await.unwrap()
}

// e.g.: rbatis -> py_sql:
// name 需在传入前拼装好 % + name + %
// #[py_sql(RB, "select * from files where name ilike #{name} order by kind desc, path")]
// pub async fn search_py_sql(name: &str) -> Vec<Files> {}

/**
 * 下钻浏览
 */
pub async fn list(path_prefix: &str, parent: i64) -> Vec<Files> {
    RB.fetch_list_by_wrapper(
        RB.new_wrapper().eq("parent", parent).like_right("path", path_prefix)
            .order_bys(&[("kind", false), ("path", true)])  // 虽代码中已包含对 13-1<2.1 的排序，
        // 但经实际观察，此处继续保留对path的排序，有助于提升web接口性能，40ms 提升到 33ms.
    ).await.unwrap()
}

/**
 * 增-档案
 */
pub async fn save(files: Files) -> u64 {
    let rb_resp = RB.save(&files, &[Skip::Value(Bson::Null)]).await;
    rb_resp.unwrap().rows_affected
}

/**
 * 改-档案
 */
pub async fn update(id: i64, i_file: Files) -> u64 {
    RB.update_by_wrapper(&i_file, RB.new_wrapper()
        .eq("id", id), &[Skip::Column("id"), Skip::Value(Bson::Null)],
    ).await.unwrap()
}

/**
 * 删-档案
 */
pub async fn delete(ids: Vec<i64>) -> u64 {
    RB.remove_by_wrapper::<Files>(
        RB.new_wrapper().r#in("id", &ids)
    ).await.unwrap()
}

/**
 * 删-档案
 */
pub async fn delete_by_path(path: &str) -> u64 {
    RB.remove_by_wrapper::<Files>(
        RB.new_wrapper().eq("crc", crc_i64(path)).eq("path", path)
    ).await.unwrap()
}

// // macOS Finder 下 “解压/删除”，“增/删” 不干净而添加，结果仍 “增/删” 不彻底。（sh下没问题）（待验 Linux smb）
// pub async fn delete_children(path: &str) -> u64 {
//     RB.remove_by_wrapper::<Files>(
//         RB.new_wrapper().like("path", format!("{}{}{}", path, path::MAIN_SEPARATOR, "%"))
//     ).await.unwrap()
// }

/// 查找后代
pub async fn find_posterity(path: &str) -> Vec<Files> {
    RB.fetch_list_by_wrapper(
        RB.new_wrapper().like_right("path", format!("{}{}", path, path::MAIN_SEPARATOR))
    ).await.unwrap()
}

/**
 * 路径回溯
 */
pub async fn backtrace(trace: Vec<String>) -> Option<Files> {
   RB.fetch_by_wrapper(
       RB.new_wrapper().r#in("path", &trace).order_by(false, &["length(path)"]).limit(1)
   ).await.unwrap()
}

/// 查找path后代目录, 按长度降续
pub async fn folder_depth_desc(path: &str) -> Vec<Depth> {
    let sql = r#"
        select length(path) len, count(*) cnt, string_agg(cast(id as varchar), ',') ids from files
            where kind = 'Folder' and path like $1 group by length(path) order by length(path) desc;
    "#;
    RB.fetch(sql, vec![Bson::String(format!("{}/%", path))]).await.unwrap()
}

/// 计算目录大小(子代 Folder + File)
pub async fn update_folder_size(ids: String) -> u64 {
    let sql = format!(r#"
        update files p set size =
            coalesce((select sum(size) from files where parent = p.id), p.size)
        where id in ({});"#, ids);
    RB.exec(sql.as_str(), vec![]).await.unwrap().rows_affected
}