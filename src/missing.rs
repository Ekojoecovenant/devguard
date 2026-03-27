use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::error::DevGuardError;

pub fn check_missing_keys(
    map: &HashMap<String, Option<String>>,
    example_path: &str,
) -> Vec<DevGuardError> {
    let mut vec_errors: Vec<DevGuardError> = Vec::new();
    if !Path::new(example_path).exists() {
        return vec![];
    }

    let file = File::open(example_path).expect("Could not find .env.example file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        if line.chars().count() == 0 {
            continue;
        }
        if line.starts_with("#") {
            continue;
        }

        if let Some((key, _)) = line.trim().split_once("=") {
            if !map.contains_key(key) {
                vec_errors.push(DevGuardError::new(
                    key.to_string(),
                    "missing".to_string(),
                    "missing required variable".to_string(),
                ));
            }
        }
    }

    vec_errors
}
