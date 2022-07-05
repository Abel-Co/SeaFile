use std::path::Path;

use rbatis::snowflake::new_snowflake_id;
use rbatis::TimestampZ;
use serde::{Deserialize, Serialize};
use crate::module::utils::crc_utils;

pub mod api;
pub mod bs;
pub mod dao;

#[derive(CRUDTable, Debug, Default, Validate, Serialize, Deserialize)]
pub struct Files {
    pub id: i64,
    pub crc : i64,
    pub size: u64,
    pub name: String,
    pub path: String,
    pub kind: String,
    pub update_at: Option<TimestampZ>,
}

impl Files {
    pub fn new(kind: String, path: &str) -> Self {
        let file = Path::new(path);
        Files {
            id: new_snowflake_id(),
            name: format!("{}", file.file_name().unwrap().to_str().unwrap()),
            path: format!("{}", file.parent().unwrap().to_str().unwrap()),
            kind,
            crc: crc_utils::crc_i64(path),
            update_at: Some(TimestampZ::now()),
            ..Default::default()
        }
    }
}