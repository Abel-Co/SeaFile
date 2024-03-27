use std::path::Path;

use futures::{
    channel::mpsc::{channel, Receiver},
    SinkExt, StreamExt,
};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use notify::EventKind::{Create, Modify, Remove};

use crate::module::ifile;

/// 创建监听器
fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (mut tx, rx) = channel(1 << 16); // 65536*2
    let watcher = RecommendedWatcher::new(move |res| {
        futures::executor::block_on(async {
            tx.send(res).await.unwrap();
        })
    }, Config::default())?;
    Ok((watcher, rx))
}

/**
 * 文件系统监听
 */
pub async fn async_watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    let (mut watcher, mut rx) = async_watcher()?;
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;
    _ = subscribe().await;
    while let Some(res) = rx.next().await {
        match res {
            Ok(event) => {
                log::info!("{:?}", event);
                match event.kind {
                    Create(kind) => ifile::bs::create(kind, event.paths[0].to_str().unwrap()).await,
                    Remove(kind) => ifile::bs::delete(kind, event.paths[0].to_str().unwrap()).await,
                    Modify(_) => publish(event.paths[0].to_str().unwrap()).await,
                    _ => ()
                }
            }
            Err(e) => log::error!("watch error: {:?}", e),
        }
    }
    Ok(())
}

// ---------------------------
use std::collections::HashMap;
use std::thread;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

use futures::executor::block_on;
use once_cell::sync::Lazy;
use parking_lot::Mutex;

pub static MAP: Lazy<Mutex<HashMap<String, SystemTime>>> = Lazy::new(|| Default::default());

pub async fn publish(path: &str) {
    let mut modify_map = MAP.lock();
    modify_map.insert(path.to_owned(), SystemTime::now());
}

pub async fn subscribe() {
    thread::spawn(move || {
        block_on(async {
            loop {
                {
                    let mut modify_map = MAP.lock();
                    let modify_map_copy = modify_map.clone();
                    let keys = modify_map_copy.keys();
                    modify_map.retain(|_, val| val.elapsed().unwrap().as_millis() < 1200);
                    for key in keys {
                        if !modify_map.contains_key(key) {
                            ifile::bs::update(key).await;
                        }
                    }
                }
                sleep(Duration::from_millis(1200));
            }
        })
    });
}


/*
#[derive(Debug)]
pub struct ModifyCache {
    pub cache: Mutex<HashMap<String, SystemTime>>,
}

impl Default for ModifyCache {
    fn default() -> Self {
        Self { cache: Default::default() }
    }
}

pub static CONTEXT: Lazy<ModifyCache> = Lazy::new(|| ModifyCache::default());

#[derive(Debug, Clone)]
pub struct Element {
    path: String,
    system_time: SystemTime,
}

impl Element {
    pub fn new(path: &str) -> Self {
        Element { path: path.to_owned(), system_time: SystemTime::now() }
    }
}
*/