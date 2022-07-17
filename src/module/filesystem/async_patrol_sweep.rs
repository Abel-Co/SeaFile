use std::path::Path;
use futures::{
    channel::mpsc::channel,
    SinkExt, StreamExt,
};
use crate::module::ifile;

use crate::module::ifile::Files;

pub async fn async_patrol(_files: Vec<Files>) {

    let (mut tx, rx) = channel(1 << 16); // 65536*2
    tx.send(_files).await.unwrap();
}

pub async fn async_watch() {
    let (mut tx, mut rx) = channel::<Vec<Files>>(1 << 16); // 65536*2
    while let Some(files) = rx.next().await {
        for _file in files {
            if !Path::new(_file.path.as_str()).exists() {
                ifile::dao::delete_by_path(_file.path.as_str()).await
            }
        }
    }
}
