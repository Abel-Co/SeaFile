use std::path;
use rbatis::crud::{CRUD, Skip};
use rbson::Bson;
use crate::boot::c::RB;
use crate::module::ifile::Files;
use crate::module::utils::crc_utils::crc_i64;

pub async fn get(path: &str) -> Option<Files> {
    RB.fetch_by_wrapper(
        RB.new_wrapper().eq("crc", crc_i64(path)).eq("path", path)
    ).await.unwrap()
}

pub async fn search(name: &str) -> Vec<Files> {
    RB.fetch_list_by_wrapper(
        RB.new_wrapper().like("name", name)
            .order_bys(&[("kind", false), ("path", true)])
    ).await.unwrap()
}

pub async fn list(parent: i64) -> Vec<Files> {
    RB.fetch_list_by_wrapper(
        RB.new_wrapper().eq("parent", parent)
            .order_bys(&[("kind", false), ("path", true)])
    ).await.unwrap()
}

pub async fn save(files: Files) -> u64 {
    let rb_resp = RB.save(&files, &[Skip::Value(Bson::Null)]).await;
    rb_resp.unwrap().rows_affected
}

pub async fn update(id: i64, ifile: Files) -> u64 {
    RB.update_by_wrapper(&ifile, RB.new_wrapper()
        .eq("id", id), &[Skip::Column("id"), Skip::Value(Bson::Null)]
    ).await.unwrap()
}

pub async fn delete(path: &str) -> u64 {
    RB.remove_by_wrapper::<Files>(
        RB.new_wrapper().eq("crc", crc_i64(path)).eq("path", path)
    ).await.unwrap()
}

pub async fn delete_all(path: &str) -> u64 {
    RB.remove_by_wrapper::<Files>(
        RB.new_wrapper().like("path", format!("{}{}{}", path, path::MAIN_SEPARATOR, "%"))
    ).await.unwrap()
}