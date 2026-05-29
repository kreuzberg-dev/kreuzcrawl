//! File-system watcher for atomic hot-reload of WAF fingerprint rules.
//!
//! [`WatchHandle`] is returned by [`super::TomlClassifier::watch`] and stops
//! the watcher when dropped.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use thiserror::Error;
use tokio::sync::{mpsc, oneshot};

use super::TomlClassifier;
use crate::waf::rules::load_from_path;

/// Drop-on-shutdown handle returned by [`TomlClassifier::watch`].
///
/// Holds the watcher and the debounce task; dropping stops both.
#[derive(Debug)]
pub struct WatchHandle {
    // Keep the watcher alive. Dropping it de-registers the OS watch.
    pub(super) _watcher: RecommendedWatcher,
    // Signals the debounce task to exit cleanly. Held in Option so Drop can
    // send the shutdown signal before aborting the task as a backstop.
    pub(super) shutdown: Option<oneshot::Sender<()>>,
    // Debounce task handle; aborted by Drop after the shutdown signal fires,
    // so a panicked task cannot silently leak.
    pub(super) task: tokio::task::JoinHandle<()>,
}

impl Drop for WatchHandle {
    fn drop(&mut self) {
        // Best-effort cooperative shutdown first, then abort as backstop.
        if let Some(tx) = self.shutdown.take() {
            let _ = tx.send(());
        }
        self.task.abort();
    }
}

/// Error returned when setting up a [`WatchHandle`].
#[derive(Debug, Error)]
pub enum WatchError {
    /// The underlying `notify` watcher could not be created or configured.
    #[error("watch setup: {0}")]
    Setup(#[from] notify::Error),
    /// The supplied path has no parent directory to watch.
    #[error("path has no parent: {0}")]
    NoParent(PathBuf),
}

/// Spawns the debounce task and configures a `RecommendedWatcher` on the
/// parent directory of `watch_path`.
///
/// The caller receives a [`WatchHandle`]; dropping it stops the watcher and
/// the debounce task.
pub(super) fn start_watch(classifier: Arc<TomlClassifier>, watch_path: &Path) -> Result<WatchHandle, WatchError> {
    // Resolve the parent directory eagerly because the OS watcher needs it to
    // exist NOW. Do NOT canonicalize watch_path itself: on Linux/inotify the
    // file may not exist yet (Kubernetes ConfigMap atomic projection, freshly
    // created temp file in tests), and an eager canonicalize would silently
    // fall back to the un-resolved path. The event closure then compares the
    // un-resolved path against inotify's resolved path and never matches.
    // Canonicalize lazily inside the closure when both sides exist, and fall
    // back to file-name match inside the watched parent directory.
    let watch_path = watch_path.to_owned();
    let parent = watch_path
        .parent()
        .ok_or_else(|| WatchError::NoParent(watch_path.clone()))?
        .to_owned();
    let parent = parent.canonicalize().unwrap_or(parent);
    let file_name = watch_path.file_name().map(|n| n.to_owned());

    // Bounded channel: if the debounce task is busy the sender simply fills
    // up, which is fine — we only care that at least one tick gets through.
    let (event_tx, event_rx) = mpsc::channel::<()>(16);
    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

    // Spawn the async debounce loop before creating the watcher so the
    // receiver is live before any events can arrive.
    let task = spawn_debounce_task(classifier, watch_path.clone(), event_rx, shutdown_rx);

    // Build and arm the watcher. Errors here are propagated to the caller.
    let path_for_closure = watch_path.clone();
    let canonical_target = path_for_closure.canonicalize().ok();
    let mut watcher = notify::recommended_watcher(move |result: notify::Result<Event>| {
        let event = match result {
            Ok(e) => e,
            Err(err) => {
                #[cfg(feature = "tracing")]
                tracing::warn!(
                    target = "kreuzcrawl::waf::watch",
                    error = %err,
                    "notify error; skipping event"
                );
                let _ = err;
                return;
            }
        };

        // Only react to events that touch the exact watch target. Match on:
        //   1. exact path equality against the original watch_path
        //   2. canonical equality against the canonical target captured at
        //      setup. Handles static symlinks pointing at a stable inode.
        //   3. file-name equality inside the watched parent directory.
        //      Carries the load for inotify-delivered Create events for files
        //      that did not exist at setup, AND for the Kubernetes ConfigMap
        //      atomic-projection symlink swap (each swap rotates the inode
        //      that canonical_target captured, so arm 2 stops matching after
        //      the first swap — only arm 3 remains).
        let is_relevant = matches!(
            event.kind,
            EventKind::Modify(_) | EventKind::Create(_) | EventKind::Remove(_)
        ) && event.paths.iter().any(|p| {
            if p == &path_for_closure {
                return true;
            }
            if let (Ok(p_canon), Some(target)) = (p.canonicalize(), canonical_target.as_ref())
                && &p_canon == target
            {
                return true;
            }
            file_name.as_deref().is_some_and(|name| p.file_name() == Some(name))
        });

        if is_relevant {
            // Ignore a full channel — a tick is already pending.
            let _ = event_tx.try_send(());
        }
    })?;

    watcher.watch(&parent, RecursiveMode::NonRecursive)?;

    Ok(WatchHandle {
        _watcher: watcher,
        shutdown: Some(shutdown_tx),
        task,
    })
}

/// Runs the debounce loop: waits for the first tick, sleeps 500 ms, drains
/// any additional ticks, then reloads the rules file and swaps atomically.
fn spawn_debounce_task(
    classifier: Arc<TomlClassifier>,
    path: PathBuf,
    mut event_rx: mpsc::Receiver<()>,
    mut shutdown_rx: oneshot::Receiver<()>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        loop {
            // Wait for the first tick or shutdown.
            tokio::select! {
                biased;
                _ = &mut shutdown_rx => {
                    break;
                }
                tick = event_rx.recv() => {
                    if tick.is_none() {
                        // Sender dropped — watcher was torn down.
                        break;
                    }
                }
            }

            // Debounce: wait 500 ms so that rapid sequences of events
            // (tmpfile + rename produces two notifications) are collapsed.
            tokio::time::sleep(Duration::from_millis(500)).await;

            // Drain any ticks that arrived during the sleep.
            while event_rx.try_recv().is_ok() {}

            // Reload and swap.
            match load_from_path(&path) {
                Ok(new_rules) => {
                    classifier.swap(new_rules);
                    #[cfg(feature = "tracing")]
                    tracing::info!(
                        target = "kreuzcrawl::waf::watch",
                        path = %path.display(),
                        "waf rules reloaded"
                    );
                }
                Err(err) => {
                    #[cfg(feature = "tracing")]
                    tracing::warn!(
                        target = "kreuzcrawl::waf::watch",
                        error = %err,
                        "reload failed; keeping previous rules"
                    );
                    let _ = err;
                }
            }
        }
    })
}
