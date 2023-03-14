use std::path::Path;
use std::thread;

use async_channel::{Receiver, Sender};
use futures::executor::block_on;
use once_cell::sync::OnceCell;

#[allow(unused)]
use crate::module::ifile;
use crate::module::ifile::Files;

static ONCE: OnceCell<(Sender<Vec<Files>>, Receiver<Vec<Files>>)> = OnceCell::new();

/**
 * 索引巡检-入队列
 */
pub async fn async_patrol(_files: &Vec<Files>) {
    let _ = ONCE.get().unwrap().0.send(_files.to_vec()).await;
}

/**
 * 索引巡检-队列消费
 */
pub async fn async_patrol_watch() {
    let _ = ONCE.set(async_channel::bounded(1 << 16));  // 65536*2
    thread::spawn(move || {
        block_on(async {
            while let Ok(files) = ONCE.get().unwrap().1.recv().await {
                for _file in files {
                    if !Path::new(_file.path.as_str()).exists() {
                        // ifile::dao::delete_by_path(_file.path.as_str()).await;   // 临时关闭巡视
                    }
                }
            }
        })
    });
}
