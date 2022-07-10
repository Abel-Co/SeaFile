use std::fs;
use std::path::Path;

use rbatis::snowflake::new_snowflake_id;
use rbatis::TimestampZ;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::module::utils::crc_utils;

pub mod api;
pub mod bs;
pub mod dao;

#[derive(CRUDTable, Debug, Default, Validate, Serialize, Deserialize)]
pub struct Files {
    pub id: i64,
    pub crc: i64,
    pub size: u64,
    pub name: String,
    pub path: String,
    pub kind: String,
    pub parent: i64,
    pub updated_at: Option<TimestampZ>,
}

impl Files {
    pub fn new(kind: String, path: &str) -> Self {
        let _file = Path::new(path);
        let parent_path = _file.parent().unwrap().to_str().unwrap();
        let parent_id = futures::executor::block_on(async {
            match dao::check(parent_path).await {
                Some(p_file) => p_file.id,
                None => 0
            }
        });
        Files {
            id: new_snowflake_id(),
            path: path.to_string(),
            name: format!("{}", _file.file_name().unwrap().to_str().unwrap()),
            parent: parent_id,
            size: match fs::metadata(path) { Ok(meta) => meta.len(), Err(_) => 0 },
            kind,
            crc: crc_utils::crc_i64(path),
            updated_at: Some(TimestampZ::now()),
            ..Default::default()
        }
    }
}