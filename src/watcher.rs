use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::time::Duration;

use log::warn;
use notify::Watcher;

fn is_same_path(a: &Path, b: &Path) -> bool {
    let a = a.canonicalize().ok();
    let b = b.canonicalize().ok();
    a.is_some() && a == b
}

pub fn watch<P: Into<PathBuf>, F: FnMut(&Path)>(path: P, mut callback: F) -> notify::Result<()> {

    let path: PathBuf = path.into();
    let parent = path.parent().unwrap_or_else(|| Path::new("."));

    callback(&path);

    loop {

        let (tx, rx) = channel();

        let mut watcher = notify::raw_watcher(tx)?;
        watcher.watch(&parent, notify::RecursiveMode::NonRecursive)?;

        loop {
            let event = match rx.recv() {
                Ok(event) => event,
                Err(err) => {
                    warn!("watcher aborted: {:?}", err);
                    std::thread::sleep(Duration::from_secs(1));
                    break;
                },
            };
            if !event.path.map(|p| is_same_path(&p, &path)).unwrap_or(false) {
                continue;
            }
            callback(&path);
        }
    }
}
