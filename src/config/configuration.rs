use std::collections::HashMap;

use super::group::ConfigurationGroup;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigurationGroups {
    #[serde(flatten)]
    groups: HashMap<String, ConfigurationGroup>
}



impl ConfigurationGroups {
    pub fn add_group(&mut self, name: String, group: ConfigurationGroup) {
        self.groups.insert(name, group);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_yaml;

    #[test]
    fn test_deserialization() {
        let yaml_content = r#"
        group1:
            INPUT_NAME: ABCD
            OUTPUT: aaaa
        group2:
            INPUT_NAME: SSSS
        "#;

        let deserialized: ConfigurationGroups = serde_yaml::from_str::<ConfigurationGroups>(&yaml_content).unwrap();

        assert_eq!(deserialized.groups.len(), 2);

        let group1 = deserialized.groups.get("group1").unwrap();
        assert_eq!(group1.entries.get("INPUT_NAME"), Some(&"ABCD".to_string()));
        assert_eq!(group1.entries.get("OUTPUT"), Some(&"aaaa".to_string()));

        let group2 = deserialized.groups.get("group2").unwrap();
        assert_eq!(group2.entries.get("INPUT_NAME"), Some(&"SSSS".to_string()));
        assert_eq!(group2.entries.get("OUTPUT"), None);
    }

    #[test]
    fn test_deserialization_empty() {
        let yaml_content = r#"{}"#;
        let deserialized: ConfigurationGroups = serde_yaml::from_str::<ConfigurationGroups>(&yaml_content).unwrap();
        assert_eq!(deserialized.groups.len(), 0);
    }
}