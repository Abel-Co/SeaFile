use rbatis::crud::{CRUD, Skip};
use rbson::Bson;
use crate::boot::c::RB;
use crate::module::ifile::Files;
use crate::module::utils::crc_utils::crc_i64;

pub async fn save(files: Files) -> u64 {
    let rb_resp = RB.save(&files, &[Skip::Value(Bson::Null)]).await;
    rb_resp.unwrap().rows_affected
}

pub async fn update(id: i64, mut ifile: Files) -> u64 {
    ifile.id = id;
    0
}

pub async fn delete(kind: &str, path: &str) -> u64 {
    RB.remove_by_wrapper::<Files>(
        RB.new_wrapper().eq("kind", &kind).eq("crc", crc_i64(path)).eq("path", path)
    ).await.unwrap()
}