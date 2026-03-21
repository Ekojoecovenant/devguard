use std::collections::HashMap;

pub struct ValidationError {
    pub key: String,
    pub rule: String,
    pub message: String,
}

impl ValidationError {
    fn new(key: String, rule: String, message: String) -> Self {
        ValidationError { key, rule, message }
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

pub fn validate_env(map: HashMap<String, Option<String>>) -> Vec<ValidationError> {
    let mut vec_errors: Vec<ValidationError> = Vec::new();

    for (key, value) in map {
        if key.contains("SECRET") {
            match value {
                None => {
                    vec_errors.push(ValidationError::new(
                        key,
                        "empty".to_string(),
                        "must not be empty".to_string(),
                    ));
                }
                Some(val) => {
                    let is_valid = val.chars().count() >= 32;
                    if !is_valid {
                        vec_errors.push(ValidationError::new(
                            key,
                            "min_length".to_string(),
                            "must be greater than or equal to 32".to_string(),
                        ));
                    }
                }
            }
        } else if key.contains("PORT") {
            match value {
                None => {
                    vec_errors.push(ValidationError::new(
                        key,
                        "empty".to_string(),
                        "must not be empty".to_string(),
                    ));
                }
                Some(val) => {
                    let is_num = val.parse::<u16>().is_ok();
                    if !is_num {
                        vec_errors.push(ValidationError::new(
                            key,
                            "format".to_string(),
                            "must be a number".to_string(),
                        ));
                    }
                }
            }
        } else if key.contains("URL") {
            match value {
                None => {
                    vec_errors.push(ValidationError::new(
                        key,
                        "empty".to_string(),
                        "must not be empty".to_string(),
                    ));
                }
                Some(val) => {
                    let is_valid = VALID_URL_PREFIXES
                        .iter()
                        .any(|prefix| val.starts_with(prefix));
                    if !is_valid {
                        vec_errors.push(ValidationError::new(
                            key,
                            "format".to_string(),
                            "must start with http://, https://, postgres://, mysql://".to_string(),
                        ));
                    }
                }
            }
        }
    }

    vec_errors
}
