use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::error::GuardStackError;

pub fn check_missing_keys(
    map: &HashMap<String, Option<String>>,
    example_path: &str,
) -> Vec<GuardStackError> {
    let mut vec_errors: Vec<GuardStackError> = Vec::new();
    if !Path::new(example_path).exists() {
        return vec![];
    }

    let file = match File::open(example_path) {
        Ok(f) => f,
        Err(_) => return vec_errors,
    };
    let reader = BufReader::new(file);

    for line in reader.lines().flatten() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        if let Some((key, _)) = trimmed.split_once('=') {
            let key = key.trim();
            if !map.contains_key(key) {
                vec_errors.push(GuardStackError::new(
                    key.to_string(),
                    "missing".to_string(),
                    "missing required variable".to_string(),
                ));
            }
        }
    }

    vec_errors
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_check_missing_keys() -> anyhow::Result<()> {
        let mut map = HashMap::new();
        map.insert("KEY1".to_string(), Some("VAL1".to_string()));

        let mut file = NamedTempFile::new()?;
        writeln!(file, "KEY1=VALUE1")?;
        writeln!(file, "KEY2=VALUE2")?;

        let missing = check_missing_keys(&map, file.path().to_str().unwrap());

        assert_eq!(missing.len(), 1);
        assert_eq!(missing[0].key, "KEY2");
        Ok(())
    }
}
