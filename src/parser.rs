use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn parser_env(path: &str) -> Result<HashMap<String, Option<String>>, String> {
    let file = File::open(path).map_err(|_| format!("Could not find .env file at '{}'", path))?;
    let reader = BufReader::new(file);

    let mut lines_map: HashMap<String, Option<String>> = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if line.chars().count() == 0 || line.starts_with("#") {
            continue;
        }

        let mut envs: Vec<&str> = line.trim().split("=").collect();
        if envs.len() == 0 {
            continue;
        }
        if envs.len() == 1 {
            lines_map.insert(envs[0].to_string(), None);
            continue;
        }
        if (envs[1].starts_with("\"") && envs[1].ends_with("\""))
            || (envs[1].starts_with("'") && envs[1].ends_with("'"))
        {
            let val = envs[1].trim_matches(['\'', '"']);
            envs[1] = val;
        }

        lines_map.insert(envs[0].to_string(), Some(envs[1].to_string()));
    }

    Ok(lines_map)
}
