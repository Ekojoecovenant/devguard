use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

// keyword pattern
const SECRET_KEYWORDS: &[&str] = &[
    "secret",
    "password",
    "passwd",
    "token",
    "apikey",
    "api_key",
    "credential",
    "private_key",
    "access_token",
    "auth_token",
];

// value patterns that look like real secrets
const SECRET_PATTERNS: &[&str] = &[
    "sk_live_",
    "pk_live_",
    "ghp_",
    "gho_",
    "xoxb-",
    "xoxp-",
    "AKIA",
    "sq0csp-",
    "access_key_id",
    "secret_access_key",
    "amzn\\.mws\\.",
    "SG\\.",
    "key-[0-9a-zA-Z]{32}",
    "AIza[0-9A-Za-z-_]{35}",
];

// files to scan
const TARGET_FILES: &[&str] = &[
    ".env.local",
    ".env.production",
    ".env.staging",
    ".env.development",
    ".env.test",
    ".env.backup",
];

// files to skip
const EXCLUDED_FILES: &[&str] = &[
    "README.md",
    "readme.md",
    ".env",
    ".env.example",
    "guardstack.config.toml",
    "package.json",
    "package-lock.json",
    "Cargo.toml",
    "Cargo.lock",
    ".gitignore",
];

// extensions to skip
const EXCLUDED_EXTENSIONS: &[&str] = &[".md", ".toml", ".lock", ".txt", ".yaml", ".yml"];

// directories to skip
const EXCLUDED_DIRS: &[&str] = &[
    "node_modules",
    ".git",
    "dist",
    "build",
    "target",
    ".next",
    "coverage",
    ".cache",
];

pub struct ScanResult {
    pub file: String,
    pub line_number: usize,
    pub line: String,
    pub reason: String,
}

use rayon::prelude::*;

pub fn scan_files(custom_path: Option<&str>) -> Vec<ScanResult> {
    match custom_path {
        Some(path) => {
            let mut results = Vec::new();
            scan_directory(path, &mut results);
            results
        }
        None => {
            // scan current directory by default if no path provided
            let mut results = Vec::new();
            scan_directory(".", &mut results);

            // also check common target files explicitly if they were missed
            for file in TARGET_FILES {
                if Path::new(file).exists()
                    && !results.iter().any(|r| r.file == *file)
                {
                    scan_single_file(file, &mut results);
                }
            }
            results
        }
    }
}

fn scan_directory(path: &str, results: &mut Vec<ScanResult>) {
    let dir = match std::fs::read_dir(path) {
        Ok(d) => d,
        Err(_) => return,
    };

    let entries: Vec<_> = dir.flatten().collect();

    let (dirs, files): (Vec<_>, Vec<_>) = entries.into_iter().partition(|e| e.path().is_dir());

    // process files in current directory in parallel
    let file_results: Vec<ScanResult> = files
        .into_par_iter()
        .filter_map(|entry| {
            let entry_path = entry.path();
            let mut local_results = Vec::new();
            if let Some(path_str) = entry_path.to_str() {
                scan_single_file(path_str, &mut local_results);
            }
            if local_results.is_empty() {
                None
            } else {
                Some(local_results)
            }
        })
        .flatten()
        .collect();

    results.extend(file_results);

    // recurse into directories
    for entry in dirs {
        let entry_path = entry.path();
        let name = entry_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");

        if EXCLUDED_DIRS.iter().any(|d| *d == name) {
            continue;
        }

        if let Some(path_str) = entry_path.to_str() {
            scan_directory(path_str, results);
        }
    }
}

fn scan_single_file(path: &str, results: &mut Vec<ScanResult>) {
    // Pre-compile regexes for patterns
    let patterns: Vec<Regex> = SECRET_PATTERNS
        .iter()
        .filter_map(|p| Regex::new(p).ok())
        .collect();

    // check excluded files
    let filename = Path::new(path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");

    if EXCLUDED_FILES.iter().any(|f| *f == filename) {
        return;
    }

    // check excluded extensions
    let ext = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    if EXCLUDED_EXTENSIONS
        .iter()
        .any(|e| e == &format!(".{}", ext).as_str())
    {
        return;
    }

    let file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return,
    };

    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(l) => l,
            Err(_) => continue,
        };

        let line_number = index + 1;
        let lower = line.to_lowercase();
        let trimmed = line.trim();

        // skip empty lines
        if trimmed.is_empty() {
            continue;
        }

        // skip Rust specific definitions only
        let is_rust_definition = trimmed.starts_with("pub struct")
            || trimmed.starts_with("struct ")
            || trimmed.starts_with("pub fn")
            || trimmed.starts_with("fn ")
            || trimmed.starts_with("impl ")
            || trimmed.starts_with("use ");

        if is_rust_definition {
            continue;
        }

        // check if line has assignment
        let has_assignment = trimmed.contains('=') || trimmed.contains(':');

        // skip lines without assignment
        if !has_assignment {
            continue;
        }

        // detect comments
        let is_comment =
            trimmed.starts_with("//") || trimmed.starts_with('#') || trimmed.starts_with('*');

        // check patterns (high confidence)
        let pattern_match = patterns.iter().find(|re| re.is_match(&line));

        if let Some(pattern) = pattern_match {
            results.push(ScanResult {
                file: path.to_string(),
                line_number,
                line: line.clone(),
                reason: if is_comment {
                    format!("possible leak in comment - pattern '{}'", pattern)
                } else {
                    format!("contains secret pattern '{}'", pattern)
                },
            });
            continue;
        }

        // check keywords (medium confidence, higher chance of false positives)
        let keyword_match = SECRET_KEYWORDS.iter().find(|k| lower.contains(*k));

        if let Some(keyword) = keyword_match {
            // Heuristic to reduce false positives in code:
            // If the keyword is part of a string literal, it's more likely a secret.
            // If it's just the variable name and assigned another variable, it might not be.
            let is_likely_secret = if is_comment {
                true
            } else {
                // Look for strings in the value part
                if let Some((_, value)) = trimmed.split_once('=') {
                    let v = value.trim().trim_end_matches(';');
                    (v.starts_with('"') && v.ends_with('"'))
                        || (v.starts_with('\'') && v.ends_with('\''))
                        || (v.starts_with('`') && v.ends_with('`'))
                } else if let Some((_, value)) = trimmed.split_once(':') {
                    let v = value.trim().trim_end_matches(';');
                    (v.starts_with('"') && v.ends_with('"'))
                        || (v.starts_with('\'') && v.ends_with('\''))
                        || (v.starts_with('`') && v.ends_with('`'))
                } else {
                    false
                }
            };

            if is_likely_secret {
                results.push(ScanResult {
                    file: path.to_string(),
                    line_number,
                    line: line.clone(),
                    reason: if is_comment {
                        format!("possible leak in comment - keyword '{}'", keyword)
                    } else {
                        format!("contains keyword '{}' with hardcoded value", keyword)
                    },
                });
            }
        }
    }
}
