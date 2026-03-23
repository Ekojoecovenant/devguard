use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn parser_env(path: &str) -> Result<(HashMap<String, Option<String>>, Vec<String>), String> {
    let file = File::open(path).map_err(|_| format!("Could not find .env file at '{}'", path))?;
    let reader = BufReader::new(file);

    let mut lines_map: HashMap<String, Option<String>> = HashMap::new();
    let mut warnings: Vec<String> = Vec::new();

    for line in reader.lines() {
        // println!("{:?}", line);
        let line = line.unwrap();
        if line.chars().count() == 0 || line.starts_with("#") {
            continue;
        }

        match line.trim().split_once("=") {
            None => {
                warnings.push(format!("⚠️  '{}' is malformed - missing '='", line.trim()));
                continue;
            }
            Some((key, value)) => {
                let mut value = value;
                if value.chars().count() == 0 {
                    lines_map.insert(key.to_string(), None);
                    continue;
                }
                if (value.starts_with("\"") && value.ends_with("\""))
                    || (value.starts_with("'") && value.ends_with("'"))
                {
                    let val = value.trim_matches(['\'', '"']);
                    value = val;
                }

                lines_map.insert(key.to_string(), Some(value.to_string()));
            }
        }
    }

    Ok((lines_map, warnings))
}
