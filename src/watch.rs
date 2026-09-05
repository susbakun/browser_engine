use std::path::PathBuf;
use std::sync::mpsc::{Receiver, channel};

use notify::Watcher;

pub fn watch_files(paths: &[PathBuf]) -> Receiver<()> {
    let (tx, rx) = channel();

    let mut watcher = notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
        if res.is_ok() {
            let _ = tx.send(());
        }
    })
    .unwrap();

    for p in paths {
        watcher
            .watch(p, notify::RecursiveMode::NonRecursive)
            .expect("failed to watch the provided files");
    }

    std::mem::forget(watcher);

    rx
}
