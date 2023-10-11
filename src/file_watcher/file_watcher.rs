use std::path::Path;

use notify::{RecommendedWatcher, Event, Watcher, Config};
use tokio::sync::mpsc::{Receiver, channel};

use crate::config::manager::ConfigurationManagerError;


#[derive(Debug)]
pub struct FileWatcher {
    pub watcher: RecommendedWatcher,
    pub rx: Receiver<notify::Result<Event>>,
}

impl FileWatcher {
    pub async fn new<P: AsRef<Path>>(path: P) -> Result<Self, ConfigurationManagerError> {
        let (tx, rx) = channel(1);
        let mut watcher = RecommendedWatcher::new(move |res| {
            let _ = tx.blocking_send(res);
        }, Config::default())?;
        watcher.watch(&path.as_ref(), notify::RecursiveMode::NonRecursive)?;
        Ok(Self { watcher, rx })
    }

    pub async fn watch(&mut self) -> Result<notify::Result<Event>, ConfigurationManagerError> {
        Ok(self.rx.recv().await.ok_or(ConfigurationManagerError::WatchTerminated)?)
    }
}