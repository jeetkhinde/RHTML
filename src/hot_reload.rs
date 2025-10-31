use anyhow::Result;
use notify::{Event, EventKind, RecursiveMode, Watcher};
use std::path::PathBuf;
use tokio::sync::broadcast;
use tracing::{error, info, warn};

/// Type of file change that occurred
#[derive(Debug, Clone, PartialEq)]
pub enum ChangeType {
    Template,
    Component,
    SourceCode,
}

/// Represents a file change event
#[derive(Debug, Clone)]
pub struct FileChange {
    pub path: PathBuf,
    pub change_type: ChangeType,
}

/// Hot reload watcher that monitors file system changes
pub struct HotReloadWatcher {
    tx: broadcast::Sender<FileChange>,
    _watcher: notify::RecommendedWatcher,
}

impl HotReloadWatcher {
    /// Create a new hot reload watcher
    pub fn new(watch_paths: Vec<PathBuf>) -> Result<Self> {
        let (tx, _) = broadcast::channel(100);
        let tx_clone = tx.clone();

        // Create file watcher
        let mut watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
            match res {
                Ok(event) => {
                    // Only process modify and create events
                    if matches!(
                        event.kind,
                        EventKind::Modify(_) | EventKind::Create(_)
                    ) {
                        for path in event.paths {
                            // Determine change type based on file path
                            let change_type = if path.to_str().unwrap_or("").contains("/pages/") {
                                ChangeType::Template
                            } else if path.to_str().unwrap_or("").contains("/components/") {
                                ChangeType::Component
                            } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                                ChangeType::SourceCode
                            } else {
                                continue; // Skip other files
                            };

                            info!("📝 File changed: {:?} ({:?})", path, change_type);

                            let file_change = FileChange {
                                path: path.clone(),
                                change_type,
                            };

                            // Broadcast change event (ignore if no receivers)
                            let _ = tx_clone.send(file_change);
                        }
                    }
                }
                Err(e) => error!("Watch error: {:?}", e),
            }
        })?;

        // Watch all specified paths
        for path in watch_paths {
            if path.exists() {
                watcher.watch(&path, RecursiveMode::Recursive)?;
                info!("👀 Watching: {:?}", path);
            } else {
                warn!("⚠️  Path does not exist: {:?}", path);
            }
        }

        Ok(Self {
            tx,
            _watcher: watcher,
        })
    }

    /// Subscribe to file change events
    pub fn subscribe(&self) -> broadcast::Receiver<FileChange> {
        self.tx.subscribe()
    }

    /// Get the broadcast sender for manual notifications
    pub fn sender(&self) -> broadcast::Sender<FileChange> {
        self.tx.clone()
    }
}

/// Create a hot reload watcher for the RHTML application
pub fn create_watcher() -> Result<HotReloadWatcher> {
    let watch_paths = vec![
        PathBuf::from("pages"),
        PathBuf::from("components"),
        PathBuf::from("src"),
    ];

    HotReloadWatcher::new(watch_paths)
}

/// Generate the live reload client script
pub fn get_live_reload_script() -> String {
    r#"
<script>
(function() {
    console.log('🔄 RHTML Hot Reload enabled');

    function connect() {
        const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
        const ws = new WebSocket(protocol + '//' + window.location.host + '/__hot_reload');

        ws.onopen = function() {
            console.log('✅ Connected to hot reload server');
        };

        ws.onmessage = function(event) {
            const data = JSON.parse(event.data);
            console.log('🔄 Received reload signal:', data);

            if (data.type === 'reload') {
                console.log('🔄 Reloading page...');
                window.location.reload();
            } else if (data.type === 'css_update') {
                console.log('🎨 Updating CSS...');
                // Could implement CSS hot swapping here in future
                window.location.reload();
            }
        };

        ws.onerror = function(error) {
            console.error('❌ WebSocket error:', error);
        };

        ws.onclose = function() {
            console.log('🔌 Hot reload disconnected, attempting to reconnect...');
            setTimeout(connect, 1000);
        };
    }

    connect();
})();
</script>
"#.to_string()
}
