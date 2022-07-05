use std::{fs};
use std::any::Any;
use std::fs::{File, FileType};
use std::io::Read;
use std::path::Path;
use actix_web::delete;
use notify::event::{CreateKind, ModifyKind, RemoveKind};
use walkdir::WalkDir;

pub async fn create(kind: CreateKind, path: &str) {
    match kind {
        // CreateKind::File => ,
        // CreateKind::Folder => ,
        _ => ()
    }
}

pub async fn update(kind: ModifyKind, path: &str) {
    match kind {
        ModifyKind::Name(any) => {
            if Path::new(path).exists() {
                // create db record
            } else {
                // delete db record
            }
        },
        _ => ()
    }
}

pub async fn delete(kind: RemoveKind, path: &str) {
    match kind {
        // RemoveKind::File => ,
        // RemoveKind::Folder => ,
        _ => ()
    }
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