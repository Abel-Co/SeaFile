use std::{cmp, fs};
use std::cmp::Ordering;
use std::path::Path;

use rayon::prelude::*;
use rbatis::snowflake::new_snowflake_id;
use rbatis::TimestampZ;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::module::filesystem;
use crate::module::utils::crc_utils;

pub mod api;
pub mod bs;
pub mod dao;

#[derive(CRUDTable, Clone, Debug, Default, Validate, Eq, Serialize, Deserialize)]
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
    pub async fn new(kind: String, path: &str) -> Self {
        let _file = Path::new(path);
        let parent_path = if let Some(_path)
            = _file.parent() { _path.to_str().unwrap() } else { "" };
        let _file_name = if let Some(_name) = _file.file_name() {
            _name.to_str().unwrap() } else { "/" };
        Files {
            id: new_snowflake_id(),
            path: path.to_string(),
            name: _file_name.to_string(),
            parent: dao::get_by_path(parent_path).await.map_or(0, |p_file| p_file.id),
            size: fs::metadata(path).map_or(0, |meta| meta.len()),
            kind,
            crc: crc_utils::crc_i64(path),
            updated_at: Some(TimestampZ::now()),
            ..Default::default()
        }
    }
}

lazy_static! {
    static ref RE_S: Vec<(Regex, char)> = vec!(
        (Regex::new(r"\d{1,3}-\d{1,3}").unwrap(), '-'),(Regex::new(r"\d{1,3}\.\d{1,3}").unwrap(), '.')
    );
}

/// 为免乱序，简单做个排序
impl Ord for Files {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.kind == "Folder" && other.kind != "Folder" {
            Ordering::Less
        } else if other.kind == "Folder" && self.kind != "Folder" {
            Ordering::Greater
        } else if self.kind == other.kind {
            let mut ord = Ordering::Equal;
            for re in RE_S.iter() {
                if ord == Ordering::Equal {
                    if let Some(ma) = re.0.find(&self.path) {
                        if let Some(mb) = re.0.find(&other.path) {
                            let (va, vb) = (
                                ma.as_str().split(re.1).collect::<Vec<&str>>(), mb.as_str().split(re.1).collect::<Vec<&str>>());
                            for i in 0..cmp::min(va.len(), vb.len()) {
                                if ord == Ordering::Equal {
                                    ord = va[i].parse::<i32>().unwrap().cmp(&vb[i].parse::<i32>().unwrap());
                                }
                            }
                        }
                    }
                }
            }
            if ord != Ordering::Equal {
                return ord;
            }
            return self.path.cmp(&other.path);
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd<Self> for Files {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.path.cmp(&other.path))
    }
}

impl PartialEq<Self> for Files {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

/**
 * 巡检、脱敏、排序
 */
async fn desensitize_sort(mut files: Vec<Files>) -> Vec<Files> {
    filesystem::async_patrol(&files).await; // 巡检, push to queue only
    files.par_iter_mut().for_each(|x| x.path = x.path[6..].to_string()); // 脱敏
    files.par_sort_by(|a, b| a.cmp(&b));    // 排序
    files
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Depth {
    pub len: u64,
    pub cnt: u64,
    pub ids: String,
}