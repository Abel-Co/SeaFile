use futures::{
    channel::mpsc::channel,
    SinkExt, StreamExt,
};

use crate::module::ifile::Files;

pub async fn async_patrol(_files: Vec<Files>) {
    tokio::sync::mpsc::channel()
    let (mut tx, rx) = channel(1 << 16); // 65536*2
    tx.send(_files).await.unwrap();
}

pub async fn async_watch() {
    let (mut tx, mut rx) = channel::<Files>(1 << 16); // 65536*2
    while let Some(event) = rx.next().await {
        log::info!("{:?}", event)
    }
}
