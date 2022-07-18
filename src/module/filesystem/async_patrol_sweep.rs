use once_cell::sync::Lazy;
use std::path::Path;
use std::sync::{Mutex};
use futures::{
    channel::mpsc::channel,
};
use futures::channel::mpsc::Sender;
use crate::module::ifile;

use crate::module::ifile::Files;

// static SENDER: OnceCell<Sender<Vec<Files>>> = OnceCell::new();
static TX: Lazy<Mutex<Vec<Sender<Vec<Files>>>>> = Lazy::new(|| Mutex::new(vec![]));

pub async fn async_patrol(_files: &Vec<Files>) {
    // let sender = SENDER.get().unwrap();
    // let mut tx = TX.get_mut().unwrap().get_mut(0).unwrap();

    // let (mut tx, rx) = channel(1 << 16); // 65536*2
    // tx.send(_files).await.unwrap();

    // fixme
    for _file in _files {
        if !Path::new(_file.path.as_str()).exists() {
            ifile::dao::delete_by_path(_file.path.as_str()).await;
        }
    }
}

pub async fn async_watch(files: Vec<Files>) {
    // let (mut tx, mut rx) = channel::<Vec<Files>>(1 << 16); // 65536*2
    // SENDER.set(tx);
    // while let Some(files) = rx.next().await {
    //     for _file in files {
    //         if !Path::new(_file.path.as_str()).exists() {
    //             ifile::dao::delete_by_path(_file.path.as_str()).await;
    //         }
    //     }
    // }
}
