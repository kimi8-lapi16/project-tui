use crate::error::{AppError, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubConfig {
    pub token: String,
    #[serde(default = "default_api_url")]
    pub api_url: String,
}

fn default_api_url() -> String {
    "https://api.github.com/graphql".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    #[serde(default = "default_theme")]
    pub theme: String,
}

fn default_theme() -> String {
    "default".to_string()
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            theme: default_theme(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub github: GitHubConfig,
    #[serde(default)]
    pub ui: UiConfig,
}

impl Settings {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;

        if !config_path.exists() {
            return Err(AppError::Config(format!(
                "Configuration file not found at: {}\nPlease create it with your GitHub token.",
                config_path.display()
            )));
        }

        let content = fs::read_to_string(&config_path).map_err(|e| {
            AppError::Config(format!(
                "Failed to read config file at {}: {}",
                config_path.display(),
                e
            ))
        })?;

        let settings: Settings = toml::from_str(&content).map_err(|e| {
            AppError::Config(format!("Failed to parse config file: {}", e))
        })?;

        Ok(settings)
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                AppError::Config(format!(
                    "Failed to create config directory at {}: {}",
                    parent.display(),
                    e
                ))
            })?;
        }

        let content = toml::to_string_pretty(self)
            .map_err(|e| AppError::Config(format!("Failed to serialize config: {}", e)))?;

        fs::write(&config_path, content).map_err(|e| {
            AppError::Config(format!(
                "Failed to write config file at {}: {}",
                config_path.display(),
                e
            ))
        })?;

        Ok(())
    }

    pub fn config_path() -> Result<PathBuf> {
        let proj_dirs = ProjectDirs::from("com", "project-tui", "project-tui").ok_or_else(
            || AppError::Config("Failed to determine config directory".to_string()),
        )?;

        let config_dir = proj_dirs.config_dir();
        Ok(config_dir.join("config.toml"))
    }

    pub fn example() -> String {
        r#"[github]
token = "ghp_your_token_here"
api_url = "https://api.github.com/graphql"

[ui]
theme = "default"
"#
        .to_string()
    }
}
