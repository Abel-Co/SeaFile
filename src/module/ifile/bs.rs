use std::{fs, thread};
use std::fs::File;
use std::io::Read;
use std::path::Path;

use futures::executor::block_on;
use notify::event::{CreateKind, ModifyKind, RemoveKind};
use rayon::prelude::*;
use walkdir::WalkDir;

use crate::boot::c::HOME;
use crate::boot::global;
use crate::module::{auth, filesystem};
use crate::module::ifile;
use crate::module::ifile::{dao, Files};

pub async fn get(id: i64) -> Option<Files> {
    match ifile::dao::get(id).await {
        Some(_file) => {
            filesystem::async_patrol(&vec![_file.clone()]).await;
            Some(_file)
        }
        None => None
    }
}

pub async fn check_path(path: &str) -> Option<Files> {
    dao::get_by_path(path).await
}

/**
 * 档案搜索
 */
pub async fn search(user_id: i64, query: &str) -> Vec<Files> {
    match auth::bs::get_subject(user_id).await {
        Some(subject) => {
            let (account, home_path) = (subject.username.unwrap(), global().watch_path.as_str());
            let files = ifile::dao::search(&format!("{}/{}", home_path, account), query).await;
            calc_folders(&files).await;
            ifile::desensitize_sort(files).await
        }
        None => vec![]
    }
}

/**
 * 目录树浏览
 */
pub async fn list(user_id: i64, parent: i64) -> Vec<Files> {
    match auth::bs::get_subject(user_id).await {
        Some(subject) => {
            let path = &*format!("{}/{}", HOME.as_str(), subject.username.unwrap());
            let files = if parent == 0 {
                match dao::get_by_path(path).await {
                    Some(file) => dao::list(path, file.id).await, // path也需限制, 防水平越权
                    None => vec![]
                }
            } else {
                dao::list(path, parent).await   // path也需限制, 防水平越权
            };
            calc_folders(&files).await;
            ifile::desensitize_sort(files).await
        }
        None => vec![]
    }
}

/**
 * 读取文件content
 */
pub async fn show(id: i64) -> String {
    match ifile::dao::get(id).await {
        Some(_file) => {
            let mut buffer = String::new();
            if let Ok(mut file) = File::open(_file.path) {
                let _ = file.read_to_string(&mut buffer);
            }
            buffer
        }
        None => "".to_string()
    }
}

pub async fn save_or_update(kind: CreateKind, path: &str) {
    match dao::get_by_path(path).await {
        Some(file) => dao::update(file.id, Files::new(format!("{:?}", kind), path).await).await,
        None => dao::save(Files::new(format!("{:?}", kind), path).await).await
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

/**
 * 建立索引
 */
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

/**
 * C端触发-刷新索引
 */
pub async fn reindex(user_id: i64, id: i64) {
    if let Some(_file) = ifile::dao::get(id).await {
        let account = auth::bs::get_subject(user_id).await.unwrap().username.unwrap();
        let home_path = global().watch_path.as_str();
        let path = &format!("{}/{}", home_path, account);
        if _file.path.starts_with(path) {
            index(_file.path.as_str()).await
        }
    }
}

/**
 * 路径回溯
 */
pub async fn backtrace(username: &str, trace: Vec<String>) -> Option<Files> {
    let home = format!("{}/{}", HOME.as_str(), username);
    let paths = trace.par_iter().map(|path| format!("{}{}", home, path)).collect();
    match dao::backtrace(paths).await {
        Some(mut file) => {
            file.path = file.path[6..].to_string();
            Some(file)
        }
        None => None
    }
}

/**
 * 校正目录(size)
 * 建议在：进入 登录、个人中心 后，预热触发
 */
pub async fn calc_folder(account: &str) {
    let path = format!("{}/{}", HOME.as_str(), account);
    thread::spawn(move || {
        block_on(async {
            let path: &str = path.as_str();
            for depth in dao::folder_depth_desc(path).await {
                let _ = dao::write_folder_size(depth.ids).await;
            }
        });
    });
}

pub async fn calc_folders(folders: &Vec<Files>) {
    let folders = folders.par_iter().filter(|x| x.kind == "Folder").map(|x| x.clone()).collect();
    thread::spawn(move || {
        block_on(async {
            let mut folders: Vec<Files> = folders;
            folders.sort_by(|a, b| b.path.len().cmp(&a.path.len()));
            for folder in folders {
                let _ = dao::write_folder_size(folder.id.to_string()).await;
            }
        });
    });
}
