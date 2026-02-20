use crate::journal::Journal;
use crate::schema2::JournalEvent;
use crate::state::GameState;
use crossbeam::channel::{bounded, unbounded, Sender};
use crossbeam::select;
use dashmap::DashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread::JoinHandle;
use std::time::Duration;
use notify_debouncer_full::{notify::*, new_debouncer};

enum Action {
    Remove,
    Add,
}

#[derive(Debug)]
pub struct JournalWatcher {
    journals: Arc<DashMap<PathBuf, Journal>>,
    pending: Option<Sender<(Action, PathBuf)>>,
    shutdown: Option<Sender<()>>,
    sender: Sender<anyhow::Result<(GameState, JournalEvent)>>,
    thread: Option<JoinHandle<()>>,
}

impl JournalWatcher {
    pub fn new(sender: Sender<anyhow::Result<(GameState, JournalEvent)>>) -> Self {
        Self {
            journals: Arc::new(DashMap::new()),
            pending: None,
            shutdown: None,
            sender,
            thread: None,
        }
    }

    pub fn start(&mut self) {
        let journals = self.journals.clone();
        let (pending_tx, pending_rx) = unbounded();
        let (shutdown_tx, shutdown_rx) = bounded(1);
        self.pending = Some(pending_tx);
        self.shutdown = Some(shutdown_tx);
        self.thread = Some(std::thread::spawn(move || {
            let (tx, rx) = unbounded();
            let mut watcher = new_debouncer(Duration::from_millis(200), None, tx).unwrap();
            for mut ev in journals.iter_mut() {
                ev.init();
                let _ = watcher.watch(ev.key(), RecursiveMode::NonRecursive);
            }

            loop {
                select! {
                    recv(shutdown_rx) -> _ => break,
                    recv(pending_rx) -> msg => {
                        if let Ok((action, v)) = msg {
                            let _ = match action {
                                Action::Add => watcher.watch(&v, RecursiveMode::NonRecursive),
                                Action::Remove => watcher.unwatch(&v),
                            };
                        }
                    }
                    recv(rx) -> msg => {
                        match msg {
                            Ok(Ok(events)) => {
                                for ev in events {
                                    if let Some(Some(path)) = ev.paths.get(0).map(|p| Path::parent(p)) {
                                        if let Some(mut journal) = journals.get_mut(path) {
                                            journal.handle_event(ev.event);
                                        }
                                    }
                                }
                            }
                            Ok(Err(errors)) => {
                                for err in errors {
                                    log::warn!("notify error: {}", err)
                                }
                            },
                            Err(_) => {} // sender dropped
                        }
                    }
                }
            }
        }));
    }

    pub fn stop(&mut self, wait: bool) {
        if let Some(shutdown) = &self.shutdown {
            let _ = shutdown.try_send(());
        }
        if wait && let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }

    pub fn add<P: AsRef<Path>>(&mut self, journal_dir: P) {
        self.add_impl(journal_dir.as_ref())
    }

    fn add_impl(&mut self, journal_dir: &Path) {
        let journal = Journal::new(journal_dir.to_path_buf(), chrono::Duration::seconds(5), self.sender.clone());
        self.journals.insert(journal_dir.to_path_buf(), journal);
        if let Some(pending) = &self.pending {
            let _ = pending.send((Action::Add, journal_dir.to_path_buf()));
        }
    }

    pub fn remove<P: AsRef<Path>>(& mut self, journal_dir: P) {
        self.remove_impl(journal_dir.as_ref());
    }

    fn remove_impl(&mut self, journal_dir: &Path) {
        self.journals.remove(journal_dir);
        if let Some(pending) = &self.pending {
            let _ = pending.send((Action::Remove, journal_dir.to_path_buf()));
        }
    }
}