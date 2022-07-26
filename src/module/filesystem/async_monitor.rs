use std::path::Path;

use futures::{
    channel::mpsc::{channel, Receiver},
    SinkExt, StreamExt,
};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use notify::EventKind::{Create, Modify, Remove};

use crate::module::ifile;

fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (mut tx, rx) = channel(1<<16); // 65536*2
    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let watcher = RecommendedWatcher::new(move |res| {
        futures::executor::block_on(async {
            tx.send(res).await.unwrap();
        })
    }, Config::default())?;
    Ok((watcher, rx))
}

pub async fn async_watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    let (mut watcher, mut rx) = async_watcher()?;
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;
    while let Some(res) = rx.next().await {
        match res {
            Ok(event) => {
                log::info!("{:?}", event);
                match event.kind {
                    Create(kind) => ifile::bs::save_or_update(kind, event.paths[0].to_str().unwrap()).await,
                    Modify(kind) => ifile::bs::update(kind, event.paths[0].to_str().unwrap()).await,
                    Remove(kind) => ifile::bs::delete(kind, event.paths[0].to_str().unwrap()).await,
                    _ => ()
                }
            }
            Err(e) => log::error!("watch error: {:?}", e),
        }
    }
    Ok(())
}

