use std::path::Path;

use super::configuration::ConfigurationGroups;
use notify::{Config, Event, RecommendedWatcher, Watcher};

use tokio::sync::mpsc::{channel, Receiver};

#[derive(Debug)]
enum ConfigurationManagerDbBackend {
    Yaml,
    Json,
}


#[derive(Debug)]
pub struct ConfigurationManager {
    db_file_path: String,
    configuration_groups: ConfigurationGroups,
    configuration_db_backend: ConfigurationManagerDbBackend,
}

impl ConfigurationManager {
    pub fn from_yaml_file(config_path: String) -> Self {
        let yaml_file_contents = std::fs::read_to_string(&config_path).expect("KONFIG");

        let serde_deserialized = serde_yaml::from_str::<ConfigurationGroups>(&yaml_file_contents).unwrap();
        
        ConfigurationManager {
            db_file_path: format!("{}.db", &config_path),
            configuration_groups: serde_deserialized,
            configuration_db_backend: ConfigurationManagerDbBackend::Yaml,
        }
    }

    pub fn save(&self) {
        match self.configuration_db_backend {
            ConfigurationManagerDbBackend::Yaml => {
                let yaml_file_contents = serde_yaml::to_string(&self.configuration_groups).unwrap();
                std::fs::write(&self.db_file_path, yaml_file_contents).expect("KONFIG");
            },
            ConfigurationManagerDbBackend::Json => {
                unimplemented!();
            }
        }
    }

    pub fn print_yaml(&self) {
        println!("Db file path: {}\n{}", self.db_file_path, serde_yaml::to_string(&self.configuration_groups).unwrap());
    }

    pub async fn watch_config_changes(&mut self) -> notify::Result<()> {
        let (mut watcher, mut rx) = async_watcher().await?;

        watcher.watch(Path::new(&self.db_file_path), notify::RecursiveMode::NonRecursive);

        while let Some(res) = rx.recv().await {
            match res {
                Ok(event) => println!("changed: {:?}", event),
                Err(e) => println!("watch error: {:?}", e),
            }
        }

        Ok(())
    }

    fn reload_config(&mut self) {
        let yaml_file_contents = std::fs::read_to_string(&self.db_file_path).expect("KONFIG");
        let serde_deserialized = serde_yaml::from_str::<ConfigurationGroups>(&yaml_file_contents).unwrap();
        self.configuration_groups = serde_deserialized;
    }
}

async fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (tx, rx) = channel(1);

    let watcher = RecommendedWatcher::new(
        move |res| {
            let tx = tx.clone();
            
            tokio::task::spawn(async move {
                tx.send(res).await.unwrap();
            });
        },
        Config::default(),
    )?;

    Ok((watcher, rx))
}