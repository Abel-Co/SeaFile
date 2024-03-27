use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use notify::event::{CreateKind, RemoveKind};
use rayon::prelude::*;
use walkdir::WalkDir;

use crate::boot::app_env;
use crate::boot::c::HOME;
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

pub async fn get_by_path(path: &str) -> Option<Files> {
    dao::get_by_path(path).await
}

/**
 * 档案搜索
 */
pub async fn search(user_id: i64, query: &str) -> Vec<Files> {
    match auth::bs::get_subject(user_id).await {
        Some(subject) => {
            let path = &get_user_home_path(&subject.username.unwrap());
            let files = ifile::dao::search(path.as_str(), query).await;
            update_folders_size(&files).await;
            desensitize_and_sort(files, path.as_str()).await
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
            // let mut path = &*format!("{}/{}", HOME.as_str(), subject.username.unwrap());
            let path = &get_user_home_path(&subject.username.unwrap());
            let files = if parent == 0 {
                match dao::get_by_path(path).await {
                    Some(file) => dao::list(path, file.id).await, // path也需限制, 防水平越权
                    None => vec![]
                }
            } else {
                dao::list(path, parent).await   // path也需限制, 防水平越权
            };
            update_folders_size(&files).await;
            desensitize_and_sort(files, path).await
        }
        None => vec![]
    }
}

fn get_user_home_path(username: &str) -> String {
    let mut path = String::from(HOME.as_str());
    if app_env().as_str() != "local" {
        path.push_str(&format!("/{}", username))
    }
    path
}

/**
 * 巡检、脱敏、排序
 */
async fn desensitize_and_sort(mut files: Vec<Files>, path: &str) -> Vec<Files> {
    filesystem::async_patrol(&files).await; // 巡检, push to queue only
    let idx = path.len();
    files.par_iter_mut().for_each(|x| x.path = x.path[idx..].to_string()); // 脱敏
    files.par_sort_by(|a, b| a.cmp(&b));    // 排序
    files
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

pub async fn create(kind: CreateKind, path: &str) {
    let i_file = Files::new(format!("{:?}", kind), path).await;
    match dao::get_by_path(path).await {
        Some(file) => dao::update(file.id, i_file).await,
        None => dao::save(i_file).await
    };
}

pub async fn update(path: &str) {
    let _file = Path::new(path);
    if _file.exists() {
        if let Some(kind) = match _file {
            _file if _file.is_file() => { Some(CreateKind::File) }
            _file if _file.is_symlink() => { Some(CreateKind::File) }
            _file if _file.is_dir() => { Some(CreateKind::Folder) }
            _ => None
        } {
            create(kind, path).await;
        }
    } else {
        ifile::dao::delete_by_path(path).await;
    }
}

pub async fn delete(_kind: RemoveKind, path: &str) {
    // ifile::dao::delete_children(path).await; // macOS Finder “解压/删除”，“增/删” 不干净而添加，结果仍 “增/删” 不彻底。（sh下没问题）（待验 Linux smb）
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
        for entry in WalkDir::new(path.as_str()) {
            if let Ok(entry) = entry {
                if let Some(kind) = match entry.file_type() {
                    file_type if file_type.is_file() => { Some(CreateKind::File) }
                    file_type if file_type.is_symlink() => { Some(CreateKind::File) }
                    file_type if file_type.is_dir() => { Some(CreateKind::Folder) }
                    _ => None
                } {
                    create(kind, entry.path().to_str().unwrap()).await;
                }
            }
        }
        // 2.清理索引
        for x in ifile::dao::find_posterity(path.as_str()).await {
            if !Path::new(x.path.as_str()).exists() {
                let _ = ifile::dao::delete(vec![x.id]).await;
            }
        }
        // 3.度量目录
        dao::update_folder_size_all_in_sql(path.as_str()).await;
    });
}

/**
 * C端触发-刷新索引
 */
pub async fn reindex(user_id: i64, id: i64) {
    if let Some(_file) = ifile::dao::get(id).await {
        let account = auth::bs::get_subject(user_id).await.unwrap().username.unwrap();
        let path = &get_user_home_path(&account);
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
 * 校正目录size
 * 建议在：进入 登录、个人中心 后，预热触发
 */
pub async fn calc_folder(account: &str) {
    let path = get_user_home_path(account);
    tokio::spawn(async move {
        // method-1.分步统计
        // 1.1.查找path后代目录, 按长度: 分组 & 降续
        for depth in dao::fetch_folder_group_by_depth_desc(path.as_str()).await {
            // 1.2.从底层向上, 逐层级（不一定在同一棵树上）更新目录体积.
            dao::update_folder_size(depth.ids).await;
        }
        // method-2.一条sql搞定（巨型存储，可能卡住DB）
        // dao::update_folder_size_all_in_sql(path.as_str()).await
    });
}

/// **同一层级**目录 更新体积
pub async fn update_folders_size(folders: &Vec<Files>) {
    let folders = folders.par_iter().filter(|x| x.kind == "Folder").map(|x| x.to_owned()).collect();
    tokio::spawn(async move {
        let folders: Vec<Files> = folders;
        // folders.sort_by(|a, b| b.path.len().cmp(&a.path.len())); // 倒序、长的在前.(都是同棵树，同层级，没必要排序了)
        let ids = folders.iter().map(|x| x.id.to_string()).collect::<Vec<_>>();
        for x in ids.chunks(100) {  // 分组，100条一批
            let _ = dao::update_folder_size(x.join(",")).await;
        }
    });
}
