use std::thread;
use std::thread::sleep;
use std::time::Duration;

use futures::executor::block_on;

use crate::boot::global;
use crate::module::{filesystem, samba};

pub async fn daemon() {
    // 1.文件系统 监视
    thread::spawn(|| {
        block_on(async {
            _ = filesystem::async_watch(global().watch_path.as_str()).await;
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