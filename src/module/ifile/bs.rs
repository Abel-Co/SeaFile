use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use notify::event::{CreateKind, ModifyKind, RemoveKind};
use walkdir::WalkDir;

use crate::module::ifile;
use crate::module::ifile::Files;

pub async fn get(id: i64) -> Option<Files> {
    ifile::dao::get(id).await
}

pub async fn search(name: &str) -> Vec<Files> {
    ifile::dao::search(name).await
}

pub async fn list(parent: i64) -> Vec<Files> {
    ifile::dao::list(parent).await
}

pub async fn show(id: i64) -> String {
    match ifile::dao::get(id).await {
        Some(_file) => {
            let mut buffer = String::new();
            File::open(_file.path).unwrap().read_to_string(&mut buffer);
            buffer
        },
        None => "".to_string()
    }
}

pub async fn save_or_update(kind: CreateKind, path: &str) {
    match ifile::dao::check(path).await {
        Some(_file) => ifile::dao::update(_file.id, Files::new(format!("{:?}", kind), path)).await,
        None => ifile::dao::save(Files::new(format!("{:?}", kind), path)).await
    };
}

pub async fn update(kind: ModifyKind, path: &str) {
    let _file = Path::new(path);
    if _file.exists() {
        let kind = if _file.is_file() { CreateKind::File } else if _file.is_dir() { CreateKind::Folder } else { CreateKind::Other };
        save_or_update(kind, path).await;
    } else {
        ifile::dao::delete(path).await;
    }
}

pub async fn delete(kind: RemoveKind, path: &str) {
    ifile::dao::delete_all(path).await;
    ifile::dao::delete(path).await;
}

pub async fn delete_file(path: &str) {
    fs::remove_dir_all(path);
}

pub async fn index(path: &'static str) {
    tokio::spawn(async move {
        for entry in WalkDir::new(path) {
            let entry = entry.unwrap();
            let file_type = entry.file_type();
            log::info!("{:?}", entry.path().display());
            if file_type.is_file() {
                save_or_update(CreateKind::File, entry.path().to_str().unwrap());
            } else if file_type.is_dir() {
                save_or_update(CreateKind::Folder, entry.path().to_str().unwrap());
            } else if file_type.is_symlink() {
                save_or_update(CreateKind::File, entry.path().to_str().unwrap());
            }
        }
    });
}