use std::collections::HashMap;

use crate::error::DevGuardError;

// RULE TRAIT
pub trait Rule {
    fn check(&self, key: &str, value: &str) -> Option<DevGuardError>;
}

pub struct SecretRule;
pub struct PortRule;
pub struct UrlRule;
pub struct IdRule;
pub struct HostRule;
pub struct NodeRule;

// Impl
impl Rule for SecretRule {
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

pub fn validate_env(map: &HashMap<String, Option<String>>) -> Vec<DevGuardError> {
    let rules: Vec<Box<dyn Rule>> = vec![
        Box::new(NodeRule),
        Box::new(SecretRule),
        Box::new(UrlRule),
        Box::new(PortRule),
        Box::new(HostRule),
        Box::new(IdRule),
    ];

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
