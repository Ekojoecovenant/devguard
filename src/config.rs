use std::path::Path;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct DevGuardConfig {
    pub rules: Option<Vec<CustomRule>>,
}

#[derive(Deserialize)]
pub struct CustomRule {
    pub pattern: String,
    pub rule: String,
    pub value: String,
    pub message: String,
}

pub fn load_config(path: &str) -> Option<DevGuardConfig> {
    if !Path::new(path).exists() {
        return None;
    }

    let content = std::fs::read_to_string(path).ok()?;
    toml::from_str(&content).ok()
}
