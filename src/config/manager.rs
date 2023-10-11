use std::{path::{Path, PathBuf}, fs, sync::Arc};

use super::configuration::ConfigurationGroups;
use thiserror::Error;
use tokio::sync::RwLock;

pub type SharedConfigManager = Arc<RwLock<ConfigurationManager>>;

#[derive(Debug, Error)]
pub enum ConfigurationManagerError {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serde Error: {0}")]
    Serde(#[from] serde_yaml::Error),
    #[error("Notify Error: {0}")]
    Notify(#[from] notify::Error),
    #[error("WatchTerminated")]
    WatchTerminated,
}


#[derive(Debug)]
enum ConfigurationManagerDbBackend {
    Yaml,
    Json,
}

impl Default for ConfigurationManagerDbBackend {
    fn default() -> Self { ConfigurationManagerDbBackend::Yaml }
}


#[derive(Default, Debug)]
pub struct ConfigurationManager {
    db_file_path: PathBuf,
    pub configuration_groups: ConfigurationGroups,
    configuration_db_backend: ConfigurationManagerDbBackend,
}

impl ConfigurationManager {
    pub fn from_yaml_file<P: AsRef<Path>>(config_path: P) -> Result<Self, ConfigurationManagerError> {
        log::info!("Loading configuration from file: {}", config_path.as_ref().to_str().unwrap());

        let yaml_file_contents = fs::read_to_string(&config_path)?;
        let serde_deserialized = serde_yaml::from_str::<ConfigurationGroups>(&yaml_file_contents)?;

        Ok(ConfigurationManager {
            db_file_path: config_path.as_ref().with_extension("db"),
            configuration_groups: serde_deserialized,
            configuration_db_backend: ConfigurationManagerDbBackend::Yaml,
        })
    }

    pub fn get_db_file_path(&self) -> &PathBuf {
        &self.db_file_path
    }

    pub fn save(&self) -> Result<(), ConfigurationManagerError> {
        match self.configuration_db_backend {
            ConfigurationManagerDbBackend::Yaml => {
                log::info!("Saving configuration to file: {}", self.db_file_path.to_str().unwrap());
                let yaml_file_contents = serde_yaml::to_string(&self.configuration_groups)?;
                std::fs::write(&self.db_file_path, yaml_file_contents)?;
            },
            ConfigurationManagerDbBackend::Json => {
                unimplemented!();
            }
        }
        Ok(())
    }

    pub fn reload_config(&mut self) -> Result<(), ConfigurationManagerError> {
        let yaml_file_contents = fs::read_to_string(&self.db_file_path)?;
        let serde_deserialized = serde_yaml::from_str::<ConfigurationGroups>(&yaml_file_contents)?;
        self.configuration_groups = serde_deserialized;
        log::info!("Configuration reloaded");
        Ok(())
    }
}