use rbatis::snowflake::new_snowflake_id;
use rbatis::TimestampZ;
use serde::{Deserialize, Serialize};
use crate::module::utils::random::gen_str64;

pub mod crc_utils;
pub mod encryption;
pub mod random;
pub mod sort;

#[derive(CRUDTable, Clone, Debug, Default, Serialize, Deserialize)]
pub struct Base {
    id: i64,
    name: BaseLabel,
    key: String,
    iv: String,
    created_at: Option<TimestampZ>,
    updated_at: Option<TimestampZ>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub enum BaseLabel {
    #[default]
    GLOBAL,
}

impl Base {
    pub fn new() -> Self {
        Base {
            id: new_snowflake_id(),
            key: gen_str64(),
            iv: gen_str64(),
            ..Default::default()
        }
    }
}
