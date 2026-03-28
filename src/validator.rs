use std::collections::HashMap;

use crate::{config::DevGuardConfig, error::DevGuardError};

// RULE TRAIT
pub trait Rule {
    fn pattern(&self) -> &str;
    fn check(&self, key: &str, value: &str) -> Option<DevGuardError>;
}

pub struct SecretRule;
pub struct PortRule;
pub struct UrlRule;
pub struct IdRule;
pub struct HostRule;
pub struct NodeRule;
pub struct DynamicRule {
    pub pattern: String,
    pub rule_type: String,
    pub value: String,
    pub message: String,
}

// Impl
impl Rule for SecretRule {
    fn pattern(&self) -> &str {
        "SECRET"
    }

    fn check(&self, key: &str, value: &str) -> Option<DevGuardError> {
        if !key.contains("SECRET") && !key.contains("KEY") && !key.contains("API") {
            return None;
        }

        if value.is_empty() {
            return Some(DevGuardError::new(
                key.to_string(),
                "empty".to_string(),
                "must not be empty".to_string(),
            ));
        }

        if value.chars().count() < 32 {
            return Some(DevGuardError::new(
                key.to_string(),
                "min_length".to_string(),
                "must be greater than or equal to 32".to_string(),
            ));
        }

        None
    }
}

impl Rule for PortRule {
    fn pattern(&self) -> &str {
        "PORT"
    }

    fn check(&self, key: &str, value: &str) -> Option<DevGuardError> {
        if !key.contains("PORT") {
            return None;
        }
        if value.is_empty() {
            return Some(DevGuardError::new(
                key.to_string(),
                "empty".to_string(),
                "must not be empty".to_string(),
            ));
        }
        if !value.parse::<u16>().is_ok() {
            return Some(DevGuardError::new(
                key.to_string(),
                "format".to_string(),
                "must be a number".to_string(),
            ));
        }

        None
    }
}

impl Rule for UrlRule {
    fn pattern(&self) -> &str {
        "URL"
    }

    fn check(&self, key: &str, value: &str) -> Option<DevGuardError> {
        if !key.contains("URL") {
            return None;
        }
        if value.is_empty() {
            return Some(DevGuardError::new(
                key.to_string(),
                "empty".to_string(),
                "must not be empty".to_string(),
            ));
        }
        if !VALID_URL_PREFIXES
            .iter()
            .any(|prefix| value.starts_with(prefix))
        {
            return Some(DevGuardError::new(
                key.to_string(),
                "format".to_string(),
                String::from(
                    "must start with http://, https://, postgres://, postgresql://, mysql://, redis://, rediss://, mongodb://, mongodb+srv://, amqp://, amqps://, sqlite://",
                ),
            ));
        }

        None
    }
}

impl Rule for IdRule {
    fn pattern(&self) -> &str {
        "ID"
    }

    fn check(&self, key: &str, value: &str) -> Option<DevGuardError> {
        if !key.contains("ID") {
            return None;
        }
        if value.is_empty() {
            return Some(DevGuardError::new(
                key.to_string(),
                "empty".to_string(),
                "must not be empty".to_string(),
            ));
        }

        None
    }
}

impl Rule for HostRule {
    fn pattern(&self) -> &str {
        "HOST"
    }

    fn check(&self, key: &str, value: &str) -> Option<DevGuardError> {
        if !key.contains("HOST") {
            return None;
        }
        if value.is_empty() {
            return Some(DevGuardError::new(
                key.to_string(),
                "empty".to_string(),
                "must not be empty".to_string(),
            ));
        }

        None
    }
}

impl Rule for NodeRule {
    fn pattern(&self) -> &str {
        "NODE_ENV"
    }

    fn check(&self, key: &str, value: &str) -> Option<DevGuardError> {
        if key != "NODE_ENV" {
            return None;
        }
        if value.is_empty() {
            return Some(DevGuardError::new(
                key.to_string(),
                "empty".to_string(),
                "must not be empty".to_string(),
            ));
        }

        if value != "development" && value != "production" && value != "test" {
            return Some(DevGuardError::new(
                key.to_string(),
                "format".to_string(),
                "must be \"development\" or \"production\" or \"test\"".to_string(),
            ));
        }

        None
    }
}

impl Rule for DynamicRule {
    fn pattern(&self) -> &str {
        &self.pattern
    }

    fn check(&self, key: &str, value: &str) -> Option<DevGuardError> {
        if !key.contains(&self.pattern) {
            return None;
        }

        match self.rule_type.as_str() {
            "min_length" => {
                let min: usize = self.value.parse().unwrap_or(32);
                if value.len() < min {
                    return Some(DevGuardError::new(
                        key.to_string(),
                        "min_length".to_string(),
                        self.message.clone(),
                    ));
                }
            }
            "one_of" => {
                let options: Vec<&str> = self.value.split(",").collect();
                if !options.contains(&value) {
                    return Some(DevGuardError::new(
                        key.to_string(),
                        "one_of".to_string(),
                        self.message.clone(),
                    ));
                }
            }
            _ => {}
        }
        None
    }
}

// outside the loop - created once!!
const VALID_URL_PREFIXES: &[&str] = &[
    "http://",
    "https://",
    "postgres://",
    "postgresql://",
    "mysql://",
    "redis://",
    "rediss://",
    "mongodb://",
    "mongodb+srv://",
    "amqp://",
    "amqps://",
    "sqlite://",
];

pub fn validate_env(
    map: &HashMap<String, Option<String>>,
    config: &Option<DevGuardConfig>,
) -> Vec<DevGuardError> {
    let mut rules: Vec<Box<dyn Rule>> = vec![
        Box::new(NodeRule),
        Box::new(SecretRule),
        Box::new(UrlRule),
        Box::new(PortRule),
        Box::new(HostRule),
        Box::new(IdRule),
    ];

    // merge custom rules from config
    if let Some(cfg) = config {
        if let Some(custom_rules) = &cfg.rules {
            for custom in custom_rules {
                let exists = rules.iter().any(|r| r.pattern() == custom.pattern);
                if exists {
                    // removing existing rule with same pattern
                    rules.retain(|r| r.pattern() != custom.pattern);
                }

                // add custom rule
                rules.push(Box::new(DynamicRule {
                    pattern: custom.pattern.clone(),
                    rule_type: custom.rule.clone(),
                    value: custom.value.clone(),
                    message: custom.message.clone(),
                }));
            }
        }
    }

    let mut vec_errors: Vec<DevGuardError> = Vec::new();

    for (key, value) in map {
        let val_str = match value {
            None => "",
            Some(v) => v.as_str(),
        };

        for rule in &rules {
            if let Some(error) = rule.check(key, val_str) {
                vec_errors.push(error);
                break;
            }
        }
    }

    vec_errors
}
