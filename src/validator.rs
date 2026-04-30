use std::collections::HashMap;

use crate::{config::GuardStackConfig, error::GuardStackError};

// RULE TRAIT
pub trait Rule {
    fn pattern(&self) -> &str;
    fn check(&self, key: &str, value: &str) -> Option<GuardStackError>;
}

pub struct SecretRule;
pub struct PortRule;
pub struct UrlRule;
pub struct IdRule;
pub struct HostRule;
pub struct NodeRule;
pub struct DummyValueRule;
pub struct InsecureConfigRule;
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

    fn check(&self, key: &str, value: &str) -> Option<GuardStackError> {
        if !key.contains("SECRET") && !key.contains("KEY") && !key.contains("API") {
            return None;
        }

        if value.is_empty() {
            return Some(GuardStackError::new(
                key.to_string(),
                "empty".to_string(),
                "must not be empty".to_string(),
            ));
        }

        if value.chars().count() < 32 {
            return Some(GuardStackError::new(
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

    fn check(&self, key: &str, value: &str) -> Option<GuardStackError> {
        if !key.contains("PORT") {
            return None;
        }
        if value.is_empty() {
            return Some(GuardStackError::new(
                key.to_string(),
                "empty".to_string(),
                "must not be empty".to_string(),
            ));
        }
        if !value.parse::<u16>().is_ok() {
            return Some(GuardStackError::new(
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

    fn check(&self, key: &str, value: &str) -> Option<GuardStackError> {
        if !key.contains("URL") {
            return None;
        }
        if value.is_empty() {
            return Some(GuardStackError::new(
                key.to_string(),
                "empty".to_string(),
                "must not be empty".to_string(),
            ));
        }
        if !VALID_URL_PREFIXES
            .iter()
            .any(|prefix| value.starts_with(prefix))
        {
            return Some(GuardStackError::new(
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

    fn check(&self, key: &str, value: &str) -> Option<GuardStackError> {
        if !key.contains("ID") {
            return None;
        }
        if value.is_empty() {
            return Some(GuardStackError::new(
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

    fn check(&self, key: &str, value: &str) -> Option<GuardStackError> {
        if !key.contains("HOST") {
            return None;
        }
        if value.is_empty() {
            return Some(GuardStackError::new(
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

    fn check(&self, key: &str, value: &str) -> Option<GuardStackError> {
        if key != "NODE_ENV" {
            return None;
        }
        if value.is_empty() {
            return Some(GuardStackError::new(
                key.to_string(),
                "empty".to_string(),
                "must not be empty".to_string(),
            ));
        }

        if value != "development" && value != "production" && value != "test" && value != "staging" {
            return Some(GuardStackError::new(
                key.to_string(),
                "format".to_string(),
                "must be one of: development, production, test, staging".to_string(),
            ));
        }

        None
    }
}

const DUMMY_VALUES: &[&str] = &[
    "123456",
    "password",
    "password123",
    "change_me",
    "changeme",
    "REPLACE_ME",
    "dummy",
    "test",
];

impl Rule for DummyValueRule {
    fn pattern(&self) -> &str {
        "DUMMY_VALUE"
    }

    fn check(&self, key: &str, value: &str) -> Option<GuardStackError> {
        if DUMMY_VALUES.iter().any(|v| value.to_lowercase() == v.to_lowercase()) {
            return Some(GuardStackError::new(
                key.to_string(),
                "insecure_value".to_string(),
                format!("contains a common dummy value: '{}'", value),
            ));
        }
        None
    }
}

impl Rule for InsecureConfigRule {
    fn pattern(&self) -> &str {
        "INSECURE_CONFIG"
    }

    fn check(&self, key: &str, value: &str) -> Option<GuardStackError> {
        if key.contains("CORS_ORIGIN") && value == "*" {
            return Some(GuardStackError::new(
                key.to_string(),
                "insecure_cors".to_string(),
                "CORS_ORIGIN set to '*' is insecure in production".to_string(),
            ));
        }

        if key == "NODE_TLS_REJECT_UNAUTHORIZED" && value == "0" {
            return Some(GuardStackError::new(
                key.to_string(),
                "insecure_ssl".to_string(),
                "SSL certificate validation is disabled (NODE_TLS_REJECT_UNAUTHORIZED=0)".to_string(),
            ));
        }

        if (key.contains("DEBUG") || key.contains("LOG_LEVEL")) && (value == "true" || value.to_lowercase() == "debug") {
            return Some(GuardStackError::new(
                key.to_string(),
                "insecure_debug".to_string(),
                "debug logging enabled - may leak sensitive information".to_string(),
            ));
        }

        None
    }
}

impl Rule for DynamicRule {
    fn pattern(&self) -> &str {
        &self.pattern
    }

    fn check(&self, key: &str, value: &str) -> Option<GuardStackError> {
        if !key.contains(&self.pattern) {
            return None;
        }

        match self.rule_type.as_str() {
            "min_length" => {
                let min: usize = self.value.parse().unwrap_or(32);
                if value.len() < min {
                    return Some(GuardStackError::new(
                        key.to_string(),
                        "min_length".to_string(),
                        self.message.clone(),
                    ));
                }
            }
            "one_of" => {
                let options: Vec<&str> = self.value.split(",").collect();
                if !options.contains(&value) {
                    return Some(GuardStackError::new(
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
    config: &Option<GuardStackConfig>,
) -> Vec<GuardStackError> {
    let mut rules: Vec<Box<dyn Rule>> = vec![
        Box::new(NodeRule),
        Box::new(SecretRule),
        Box::new(UrlRule),
        Box::new(PortRule),
        Box::new(HostRule),
        Box::new(IdRule),
        Box::new(DummyValueRule),
        Box::new(InsecureConfigRule),
    ];

    // merge custom rules from config
    if let Some(cfg) = config {
        if let Some(custom_rules) = &cfg.rules {
            for custom in custom_rules {
                // custom rules override built-in rules with matching patterns
                rules.retain(|r| r.pattern() != custom.pattern);

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

    let mut vec_errors: Vec<GuardStackError> = Vec::new();

    for (key, value) in map {
        let val_str = value.as_deref().unwrap_or("");

        for rule in &rules {
            if let Some(error) = rule.check(key, val_str) {
                vec_errors.push(error);
                break;
            }
        }
    }

    vec_errors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_rule() {
        let rule = NodeRule;
        assert!(rule.check("NODE_ENV", "production").is_none());
        assert!(rule.check("NODE_ENV", "invalid").is_some());
        assert!(rule.check("OTHER", "invalid").is_none());
    }

    #[test]
    fn test_secret_rule() {
        let rule = SecretRule;
        let long_secret = "a".repeat(32);
        let short_secret = "a".repeat(31);
        assert!(rule.check("APP_SECRET", &long_secret).is_none());
        assert!(rule.check("APP_SECRET", &short_secret).is_some());
    }

    #[test]
    fn test_port_rule() {
        let rule = PortRule;
        assert!(rule.check("PORT", "3000").is_none());
        assert!(rule.check("PORT", "abc").is_some());
        assert!(rule.check("PORT", "70000").is_some());
    }
}
