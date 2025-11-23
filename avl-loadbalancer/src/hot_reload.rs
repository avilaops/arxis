//! Configuration Hot Reload Module
//!
//! Monitors configuration files for changes and reloads them automatically.

use anyhow::Result;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{error, info};

/// Configuration reload event
#[derive(Debug, Clone)]
pub struct ReloadEvent {
    pub path: PathBuf,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Hot reload service for configuration files
pub struct HotReloadService {
    _watcher: RecommendedWatcher,
    reload_rx: mpsc::UnboundedReceiver<ReloadEvent>,
}

impl HotReloadService {
    /// Create a new hot reload service watching a configuration file
    pub fn new<P: AsRef<Path>>(config_path: P) -> Result<Self> {
        let (reload_tx, reload_rx) = mpsc::unbounded_channel();
        let config_path = config_path.as_ref().to_path_buf();

        let tx = reload_tx.clone();
        let mut watcher = RecommendedWatcher::new(
            move |res: Result<Event, notify::Error>| {
                match res {
                    Ok(event) => {
                        if event.kind.is_modify() {
                            info!("Configuration file changed: {:?}", event.paths);
                            let reload_event = ReloadEvent {
                                path: config_path.clone(),
                                timestamp: chrono::Utc::now(),
                            };
                            if let Err(e) = tx.send(reload_event) {
                                error!("Failed to send reload event: {}", e);
                            }
                        }
                    }
                    Err(e) => error!("Watch error: {}", e),
                }
            },
            Config::default(),
        )?;

        watcher.watch(config_path.as_ref(), RecursiveMode::NonRecursive)?;

        Ok(Self {
            _watcher: watcher,
            reload_rx,
        })
    }

    /// Wait for next reload event
    pub async fn next_reload(&mut self) -> Option<ReloadEvent> {
        self.reload_rx.recv().await
    }
}

/// Hot reload manager for LoadBalancer configuration
pub struct ConfigReloadManager {
    service: Arc<tokio::sync::Mutex<HotReloadService>>,
}

impl ConfigReloadManager {
    /// Create a new config reload manager
    pub fn new<P: AsRef<Path>>(config_path: P) -> Result<Self> {
        let service = HotReloadService::new(config_path)?;
        Ok(Self {
            service: Arc::new(tokio::sync::Mutex::new(service)),
        })
    }

    /// Start watching for configuration changes
    pub async fn watch<F>(&self, mut on_reload: F)
    where
        F: FnMut(ReloadEvent) + Send + 'static,
    {
        let service = self.service.clone();
        tokio::spawn(async move {
            let mut service = service.lock().await;
            while let Some(event) = service.next_reload().await {
                info!("Reloading configuration from: {:?}", event.path);
                on_reload(event);
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::Duration;

    #[tokio::test]
    #[ignore] // Requires file system operations
    async fn test_hot_reload() {
        let temp_file = "test_config_hot_reload.yaml";
        fs::write(temp_file, "test: value").unwrap();

        let mut service = HotReloadService::new(temp_file).unwrap();

        // Modify file
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(100)).await;
            fs::write(temp_file, "test: updated").unwrap();
        });

        // Wait for reload event with timeout
        let result = tokio::time::timeout(Duration::from_secs(2), service.next_reload()).await;

        fs::remove_file(temp_file).ok();

        assert!(result.is_ok());
        let event = result.unwrap();
        assert!(event.is_some());
    }
}
