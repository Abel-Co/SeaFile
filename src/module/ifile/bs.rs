use std::{fs};
use std::fs::File;
use std::io::Read;
use std::path::Path;

use notify::event::{CreateKind, ModifyKind, RemoveKind};
use walkdir::WalkDir;

use crate::module::{filesystem, user};
use crate::module::ifile;
use crate::module::ifile::Files;

pub async fn get(id: i64) -> Option<Files> {
    match ifile::dao::get(id).await {
        Some(_file) => {
            filesystem::async_patrol(&vec![_file.clone()]).await;
            Some(_file)
        }
        None => None
    }
}

pub async fn search(user_id: i64, query: &str) -> Vec<Files> {
    let account = &user::dao::get(user_id).await.username.unwrap();
    let mut _files = ifile::dao::search(&format!("/home/{}", account), query).await;
    filesystem::async_patrol(&_files).await;
    _files.sort_by(|a, b| a.cmp(&b));
    _files
}

pub async fn list(user_id: i64, parent: i64) -> Vec<Files> {
    let account = &user::dao::get(user_id).await.username.unwrap();
    let path = &format!("/home/{}", account);
    let mut _files = if parent == 0 {
        let _file = ifile::dao::get_by_path(path).await.unwrap();
        ifile::dao::list(path, _file.id).await
    } else {
        ifile::dao::list(path, parent).await
    };
    filesystem::async_patrol(&_files).await;
    _files.sort_by(|a, b| a.cmp(&b));
    _files
}

pub async fn show(id: i64) -> String {
    match ifile::dao::get(id).await {
        Some(_file) => {
            let mut buffer = String::new();
            let _ = File::open(_file.path).unwrap().read_to_string(&mut buffer);
            buffer
        }
        None => "".to_string()
    }
}

pub async fn save_or_update(kind: CreateKind, path: &str) {
    match ifile::dao::check(path).await {
        Some(_file) => ifile::dao::update(_file.id, Files::new(format!("{:?}", kind), path).await).await,
        None => ifile::dao::save(Files::new(format!("{:?}", kind), path).await).await
    };
}

pub async fn update(_kind: ModifyKind, path: &str) {
    let _file = Path::new(path);
    if _file.exists() {
        let kind = if _file.is_file() { CreateKind::File } else if _file.is_dir() { CreateKind::Folder } else { CreateKind::Other };
        save_or_update(kind, path).await;
    } else {
        ifile::dao::delete_by_path(path).await;
    }
}

pub async fn delete(_kind: RemoveKind, path: &str) {
    // ifile::dao::delete_children(path).await; // macOS Finder 下 “解压/删除”，“增/删” 不干净而添加，结果仍 “增/删” 不彻底。（sh下没问题）（待验 Linux smb）
    ifile::dao::delete_by_path(path).await;
}

pub async fn delete_file(path: &str) {
    fs::remove_dir_all(path).expect("删除失败");
}

pub async fn index(path: &str) {
    let path = String::from(path);
    tokio::spawn(async move {
        // // 不能采用删老记录的方案，因为离线保存的，基于url的资源地址将失效。
        // let _ = ifile::dao::delete_children(path.as_str()).await;
        // 1.发现数据
        for entry in WalkDir::new(path.clone()) {
            if let Ok(entry) = entry {
                match entry.file_type() {
                    file_type if file_type.is_file() => {
                        save_or_update(CreateKind::File, entry.path().to_str().unwrap()).await;
                    }
                    file_type if file_type.is_dir() => {
                        save_or_update(CreateKind::Folder, entry.path().to_str().unwrap()).await;
                    }
                    file_type if file_type.is_symlink() => {
                        save_or_update(CreateKind::File, entry.path().to_str().unwrap()).await;
                    }
                    _ => {}
                }
            }
        }
        // 2.清理索引
        for x in ifile::dao::find_posterity(path.as_str()).await {
            if !Path::new(x.path.as_str()).exists() {
                let _ = ifile::dao::delete(vec![x.id]).await;
            }
        }
    });
}

pub async fn reindex(user_id: i64, id: i64) {
    if let Some(_file) = ifile::dao::get(id).await {
        let account = user::dao::get(user_id).await.username.unwrap();
        let path = &format!("/home/{}", account);
        if _file.path.starts_with(path) {
            index(_file.path.as_str()).await
        }
    }
}