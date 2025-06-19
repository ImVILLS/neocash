// config.rs

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf};
use dirs;
use toml;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum PathDisplayMode {
    Full,
    Short,
    ShortAll,
    Current,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PromptConfig {
    pub template: String,
    pub path_mode: PathDisplayMode,
    pub status_icon_success: String,
    pub status_icon_error: String,
    pub show_time: bool,
    pub show_user: bool,
    pub show_host: bool,
    pub default_editor: String, 
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShellConfig {
    pub prompt: PromptConfig,
    pub colors: HashMap<String, String>,
    pub history_size: usize,
    pub history_file: String,
    #[serde(skip)]
    pub config_path: PathBuf,
}

impl Default for ShellConfig {
    fn default() -> Self {
        let mut colors = HashMap::new();
        colors.insert("reset".to_string(), "\x1b[0m".to_string());
        colors.insert("black".to_string(), "\x1b[30m".to_string());
        colors.insert("red".to_string(), "\x1b[31m".to_string());
        colors.insert("green".to_string(), "\x1b[32m".to_string());
        colors.insert("yellow".to_string(), "\x1b[33m".to_string());
        colors.insert("blue".to_string(), "\x1b[34m".to_string());
        colors.insert("magenta".to_string(), "\x1b[35m".to_string());
        colors.insert("cyan".to_string(), "\x1b[36m".to_string());
        colors.insert("white".to_string(), "\x1b[37m".to_string());
        colors.insert("bg_black".to_string(), "\x1b[40m".to_string());
        colors.insert("bg_red".to_string(), "\x1b[41m".to_string());
        colors.insert("bg_green".to_string(), "\x1b[42m".to_string());
        colors.insert("bg_yellow".to_string(), "\x1b[43m".to_string());
        colors.insert("bg_blue".to_string(), "\x1b[44m".to_string());
        colors.insert("bg_magenta".to_string(), "\x1b[45m".to_string());
        colors.insert("bg_cyan".to_string(), "\x1b[46m".to_string());
        colors.insert("bg_white".to_string(), "\x1b[47m".to_string());
        colors.insert("bold".to_string(), "\x1b[1m".to_string());
        colors.insert("underline".to_string(), "\x1b[4m".to_string());
        colors.insert("italic".to_string(), "\x1b[3m".to_string());
        colors.insert("strikethrough".to_string(), "\x1b[9m".to_string());

        ShellConfig {
            prompt: PromptConfig {
                template: "[$time] $user@$host:$path $status_icon ".to_string(),
                path_mode: PathDisplayMode::Short,
                status_icon_success: "$green✓$reset".to_string(),
                status_icon_error: "$red✗$reset".to_string(),
                show_time: true,
                show_user: true,
                show_host: true,
                default_editor: "nano".to_string(),
            },
            colors,
            history_size: 1000,
            history_file: "~/.local/share/ncash/history.txt".to_string(),
            config_path: Self::get_default_config_path(),
        }
    }
}

impl ShellConfig {
    pub fn load(custom_path: Option<PathBuf>) -> Self {
        let config_path = custom_path.unwrap_or_else(Self::get_default_config_path);
        let default_config = Self::default();

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).ok();
        }

        match fs::read_to_string(&config_path) {
            Ok(contents) => match toml::from_str::<ShellConfig>(&contents) {
                Ok(mut config) => {
                    config.config_path = config_path;
                    for (k, v) in &default_config.colors {
                        config.colors.entry(k.clone()).or_insert(v.clone());
                    }
                    config
                }
                Err(e) => {
                    eprintln!("Config error: {}. Using defaults.", e);

                    let mut config = default_config;
                    config.config_path = config_path;
                    config
                }
            },
            Err(_) => {
                // If the file does not exist, create it with default values
                fs::write(
                    &config_path,
                    toml::to_string_pretty(&default_config).unwrap(),
                )
                .ok();
                // Important: if the file is created, we need to set the config_path correctly
                let mut config = default_config;
                config.config_path = config_path;
                config
            }
        }
    }

    pub fn get_default_config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("neocash/ncashrc")
    }

    pub fn get_active_config_path(&self) -> &PathBuf {
        &self.config_path
    }

    pub fn get_history_path(&self) -> PathBuf {
        shellexpand::tilde(&self.history_file).into_owned().into()
    }
}
