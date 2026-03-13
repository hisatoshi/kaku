use anyhow::Result;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
struct RawConfig {
    directory: Option<String>,
    editor: Option<String>,
}

pub struct Config {
    pub directory: PathBuf,
    pub editor: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = dirs_path();

        let raw: RawConfig = if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            toml::from_str(&content)?
        } else {
            RawConfig {
                directory: None,
                editor: None,
            }
        };

        let directory = raw
            .directory
            .map(|d| expand_tilde(&d))
            .unwrap_or_else(default_directory);

        let editor = raw.editor.unwrap_or_else(|| "nvim".to_string());

        Ok(Config { directory, editor })
    }
}

fn dirs_path() -> PathBuf {
    home_dir().join(".quick-prompt-editor")
}

fn default_directory() -> PathBuf {
    home_dir().join(".prompts")
}

fn home_dir() -> PathBuf {
    std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."))
}

fn expand_tilde(path: &str) -> PathBuf {
    if let Some(rest) = path.strip_prefix("~/") {
        home_dir().join(rest)
    } else if path == "~" {
        home_dir()
    } else {
        PathBuf::from(path)
    }
}
