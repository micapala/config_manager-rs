use super::configuration::ConfigurationGroups;

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
}