use std::path::Path;

use event::EventKind;
use futures::{
    channel::mpsc::{channel, Receiver},
    SinkExt, StreamExt,
};
use notify::{Event, event, RecommendedWatcher, RecursiveMode, Watcher};

use crate::module::file;

fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (mut tx, rx) = channel(1);
    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let watcher = RecommendedWatcher::new(move |res| {
        futures::executor::block_on(async {
            tx.send(res).await.unwrap();
        })
    })?;
    Ok((watcher, rx))
}

pub async fn async_watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    let (mut watcher, mut rx) = async_watcher()?;
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;
    while let Some(res) = rx.next().await {
        match res {
            Ok(event) => {
                match event.kind {
                    EventKind::Create(file) => file::bs::create(1, event.paths[0].to_str().unwrap()).await,
                    EventKind::Remove(file) => file::bs::delete(1, event.paths[0].to_str().unwrap()).await,
                    EventKind::Modify(file) => file::bs::update(1, event.paths[0].to_str().unwrap()).await,
                    _ => ()
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
    Ok(())
}

fn main() {
    let path = "/var/lib/grafana";
    println!("watching {}", path);

    futures::executor::block_on(async {
        if let Err(e) = async_watch(path).await {
            println!("error: {:?}", e)
        }
    });
}
