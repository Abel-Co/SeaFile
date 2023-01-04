use std::{thread};
use std::ops::Index;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use futures::executor::block_on;
use notify::event::CreateKind;
use rbatis::crud::CRUD;

use crate::boot::c::RB;
use crate::boot::global;
use crate::module::{filesystem, ifile, samba};
use crate::module::ifile::Files;
use crate::module::user::{Users, UserType};

pub async fn if_init_db() {
    let tables: i64 = RB.fetch("select count(*) from pg_tables where tablename = 'files';", vec![]).await.unwrap();
    if tables == 0 {
        // 1.建表
        let sql = std::fs::read_to_string("scripts/create.sql").unwrap();
        let _ = RB.exec(sql.as_str(), vec![]).await;
        // 2.记录管理路径
        let watch_path = global().watch_path.as_str();
        ifile::bs::save_or_update(CreateKind::Folder, watch_path).await;
    }
    // let files: i64 = RB.fetch("select count(*) from files;", vec![]).await.unwrap();
    // if files < 2 {
    let root_path = RB.fetch_by_wrapper::<Option<Files>>(RB.new_wrapper().eq("path", global().watch_path.as_str())).await.unwrap();
    if let None = root_path {
        // 3.初始建立索引
        let watch_path = global().watch_path.as_str();
        ifile::bs::index(watch_path).await;
    }
}

pub async fn init_smb_account() {
    if cfg!(target_os = "linux") {
        let output = Command::new("sh").arg("-c").arg("cat /etc/os-release").output();
        if !String::from_utf8_lossy(&output.unwrap().stdout).contains("Alpine Linux") {return}
        for user in RB.fetch_list::<Users>().await.unwrap() {
            let account = user.username.unwrap();
            let _ = Command::new("adduser").arg("-D").arg(&account).output();
            if UserType::User == user.user_type {
                let password = user.password.unwrap();
                let double_passwd = format!("{}\n{}\n", password, password);
                let output = Command::new("echo -e").arg(double_passwd).arg(" | smbpasswd -a -s").arg(&account).output();
                let output = match output {
                    Ok(output) => String::from_utf8_lossy(&output.stdout).to_string(),
                    Err(err) => err.to_string()
                };
                log::info!("{}", output);
            }
        }
    }
}

pub async fn daemon() {
    // 1.文件系统 监视
    thread::spawn(|| {
        block_on(async {
            let _ = filesystem::async_watch(global().watch_path.as_str()).await;
        })
    });

    // 2.索引合法性巡视
    filesystem::async_patrol_watch().await;

    // 3.samba服务保活
    thread::spawn(|| {
        loop {
            block_on(async {
                samba::daemon_smb().await;
            });
            sleep(Duration::from_secs(56))
        }
    });
}