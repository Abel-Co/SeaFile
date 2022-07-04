use std::path::Path;

use rbatis::snowflake::new_snowflake_id;
use rbatis::TimestampZ;
use serde::{Deserialize, Serialize};

pub mod api;
pub mod bs;

#[derive(CRUDTable, Debug, Default, Validate, Serialize, Deserialize)]
pub struct FileInfo {
    pub id: i64,
    pub file_name: String,
    pub file_size: u64,
    pub file_type: u8,
    pub file_path: String,
    pub update_at: Option<TimestampZ>,
}

impl FileInfo {
    pub fn new(file: u8, path: &str) -> Self {
        let path = Path::new(path);
        FileInfo {
            id: new_snowflake_id(),
            file_name: format!("{}", path.file_name().unwrap().to_str().unwrap()),
            file_type: file,
            file_path: format!("{}", path.parent().unwrap().to_str().unwrap()),
            file_size: 0,
            update_at: Some(TimestampZ::now()),
        }
    }
}