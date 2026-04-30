use anyhow::{Context, Result};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn parser_env(path: &str) -> Result<(HashMap<String, Option<String>>, Vec<String>)> {
    let file = File::open(path).with_context(|| format!("Could not find .env file at '{}'", path))?;
    let reader = BufReader::new(file);

    let mut lines_map: HashMap<String, Option<String>> = HashMap::new();
    let mut warnings: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line.context("Failed to read line from .env file")?;
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        match trimmed.split_once('=') {
            None => {
                warnings.push(format!("⚠️  '{}' is malformed - missing '='", trimmed));
                continue;
            }
            Some((key, value)) => {
                let key = key.trim();
                let mut value = value.trim();

                if value.is_empty() {
                    lines_map.insert(key.to_string(), None);
                    continue;
                }

                if (value.starts_with('"') && value.ends_with('"'))
                    || (value.starts_with('\'') && value.ends_with('\''))
                {
                    value = &value[1..value.len() - 1];
                }

                lines_map.insert(key.to_string(), Some(value.to_string()));
            }
        }
    }

    Ok((lines_map, warnings))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_parser_env_basic() -> Result<()> {
        let mut file = NamedTempFile::new()?;
        writeln!(file, "KEY1=VALUE1")?;
        writeln!(file, "KEY2=\"VALUE2\"")?;
        writeln!(file, "KEY3='VALUE3'")?;
        writeln!(file, "EMPTY_KEY=")?;
        writeln!(file, "# COMMENT")?;
        writeln!(file, "")?;

        let (map, warnings) = parser_env(file.path().to_str().unwrap())?;

        assert_eq!(map.get("KEY1"), Some(&Some("VALUE1".to_string())));
        assert_eq!(map.get("KEY2"), Some(&Some("VALUE2".to_string())));
        assert_eq!(map.get("KEY3"), Some(&Some("VALUE3".to_string())));
        assert_eq!(map.get("EMPTY_KEY"), Some(&None));
        assert!(warnings.is_empty());
        Ok(())
    }

    #[test]
    fn test_parser_env_malformed() -> Result<()> {
        let mut file = NamedTempFile::new()?;
        writeln!(file, "MALFORMED_LINE")?;

        let (map, warnings) = parser_env(file.path().to_str().unwrap())?;

        assert!(map.is_empty());
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("malformed"));
        Ok(())
    }
}
