use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use notify::event::{CreateKind, ModifyKind, RemoveKind};
use walkdir::WalkDir;

use crate::module::ifile;
use crate::module::ifile::Files;

pub async fn create(kind: CreateKind, path: &str) {
    ifile::dao::save(Files::new(format!("{:?}", kind), path)).await;
}

pub async fn update(kind: ModifyKind, path: &str) {
    let ifile = Files::new(format!("{:?}", kind), path);
    match kind {
        ModifyKind::Name(any) => {
            if Path::new(path).exists() {
                ifile::dao::save(Files::new(format!("{:?}", kind), path)).await;
            } else {
                ifile::dao::delete(format!("{:?}", kind).as_str(), path);
            }
        }
        _ => ()
    }
}

pub async fn delete(kind: RemoveKind, path: &str) {
    ifile::dao::delete(format!("{:?}", kind).as_str(), path).await;
}

pub async fn read(path: &str) -> String {
    let mut buffer = String::new();
    File::open(path).unwrap().read_to_string(&mut buffer);
    buffer
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
                delete(RemoveKind::File, entry.path().to_str().unwrap());
                create(CreateKind::File, entry.path().to_str().unwrap());
            } else if file_type.is_dir() {
                delete(RemoveKind::Folder, entry.path().to_str().unwrap());
                create(CreateKind::Folder, entry.path().to_str().unwrap());
            } else if file_type.is_symlink() {
                delete(RemoveKind::File, entry.path().to_str().unwrap());
                create(CreateKind::File, entry.path().to_str().unwrap());
            }
        }
    });
}