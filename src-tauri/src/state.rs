use crate::database::Database;
use crate::device_detection::DetectedDevice;
use crate::serial::PortLockManager;
use crate::serial_monitor::SerialSessionHub;
use parking_lot::RwLock;
use std::path::PathBuf;

pub struct AppState {
    pub db: Database,
    pub data_dir: PathBuf,
    pub ports: PortLockManager,
    pub devices: RwLock<Vec<DetectedDevice>>,
    pub last_errors: RwLock<Vec<String>>,
    pub serial_hub: SerialSessionHub,
}

impl AppState {
    pub fn new(data_dir: PathBuf) -> crate::error::AppResult<Self> {
        std::fs::create_dir_all(&data_dir)?;
        let db = Database::open(data_dir.join("aether_flash_studio.db"))?;
        Ok(Self {
            db,
            data_dir,
            ports: PortLockManager::new(),
            devices: RwLock::new(Vec::new()),
            last_errors: RwLock::new(Vec::new()),
            serial_hub: SerialSessionHub::default(),
        })
    }

    pub fn mock_mode_enabled(&self) -> bool {
        self.db
            .get_setting("mock_device_mode")
            .ok()
            .flatten()
            .map(|v| v == "true" || v == "1")
            .unwrap_or(false)
    }

    pub fn push_error(&self, message: impl Into<String>) {
        let mut errors = self.last_errors.write();
        errors.insert(0, message.into());
        errors.truncate(20);
    }
}
