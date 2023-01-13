use std::path;
use rbatis::crud::{CRUD, Skip};
use rbson::Bson;
use crate::boot::c::RB;
use crate::module::ifile::Files;
use crate::module::utils::crc_utils::crc_i64;

pub async fn get(id: i64) -> Option<Files> {
    RB.fetch_by_column("id", id).await.unwrap()
}

pub async fn get_by_path(path: &str) -> Option<Files> {
    RB.fetch_by_column("path", path).await.unwrap()
}

pub async fn check(path: &str) -> Option<Files> {
    RB.fetch_by_wrapper(
        RB.new_wrapper().eq("crc", crc_i64(path)).eq("path", path)
    ).await.unwrap()
}

pub async fn search(path_prefix: &str, query: &str) -> Vec<Files> {
    // ilike
    let query = format!("{}{}{}", "%", query, "%").to_string();
    let path_prefix = format!("{}/%", path_prefix).to_string();
    RB.fetch("select * from files where path like $1 and name ilike $2 order by kind desc, path;",
    // 虽代码中已包含对 13-1<2.1 的排序，但经实际观察，此处继续保留对path的排序，有助于提升web接口性能，40ms 提升到 33ms.
             vec![Bson::String(path_prefix), Bson::String(query)]).await.unwrap()
    // like
    /*RB.fetch_list_by_wrapper(
        RB.new_wrapper().like("name", name)
            .order_bys(&[("kind", false), ("path", true)])
    ).await.unwrap()*/
}
/// name 需在传入前拼装好 % + name + %
// #[py_sql(RB, "select * from files where name ilike #{name} order by kind desc, path")]
// pub async fn search_py_sql(name: &str) -> Vec<Files> {}

pub async fn list(path_prefix: &str, parent: i64) -> Vec<Files> {
    /*let mut wrapper = RB.new_wrapper().like_right("path", path_prefix);
    if parent != 0 {
        wrapper = wrapper.eq("parent", parent)
    }
    // 虽代码中已包含对 13-1<2.1 的排序，但经实际观察，此处继续保留对path的排序，有助于提升web接口性能，40ms 提升到 33ms.
    wrapper = wrapper.order_bys(&[("kind", false), ("path", true)]);
    RB.fetch_list_by_wrapper(wrapper).await.unwrap()*/

    RB.fetch_list_by_wrapper(
        RB.new_wrapper().eq("parent", parent).like_right("path", path_prefix)
            .order_bys(&[("kind", false), ("path", true)])  // 虽代码中已包含对 13-1<2.1 的排序，
        // 但经实际观察，此处继续保留对path的排序，有助于提升web接口性能，40ms 提升到 33ms.
    ).await.unwrap()
}

pub async fn save(files: Files) -> u64 {
    let rb_resp = RB.save(&files, &[Skip::Value(Bson::Null)]).await;
    rb_resp.unwrap().rows_affected
}

pub async fn update(id: i64, ifile: Files) -> u64 {
    RB.update_by_wrapper(&ifile, RB.new_wrapper()
        .eq("id", id), &[Skip::Column("id"), Skip::Value(Bson::Null)],
    ).await.unwrap()
}

pub async fn delete(ids: Vec<i64>) -> u64 {
    RB.remove_by_wrapper::<Files>(
        RB.new_wrapper().r#in("id", &ids)
    ).await.unwrap()
}

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