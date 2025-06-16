use chrono::Local;
use std::{
    env,
    path::PathBuf,  // Убрали неиспользуемый импорт Path
};
use crate::config::{ShellConfig, PathDisplayMode};

pub struct PromptContext {
    pub time: String,
    pub last_error: i32,
    pub hostname: String,
    pub username: String,
    pub path: String,
    pub status_icon: String,
}

pub fn get_prompt_context(last_error: i32, config: &ShellConfig) -> PromptContext {
    let time = if config.prompt.show_time {
        Local::now().format("%H:%M:%S").to_string()
    } else {
        String::new()
    };

    let username = if config.prompt.show_user {
        whoami::username()
    } else {
        String::new()
    };

    let hostname = if config.prompt.show_host {
        whoami::fallible::hostname().unwrap_or_else(|_| "unknown".to_string())
    } else {
        String::new()
    };

    let current_path = env::current_dir()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "?".to_string());

    let path = format_path(&current_path, &config.prompt.path_mode);

    let status_icon = if last_error == 0 {
        config.prompt.status_icon_success.clone()
    } else {
        config.prompt.status_icon_error.clone()
    };

    PromptContext {
        time,
        last_error,
        hostname,
        username,
        path,
        status_icon,
    }
}

fn format_path(path: &str, mode: &PathDisplayMode) -> String {
    let home_dir = dirs::home_dir()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|| "~".to_string());

    let path_buf = PathBuf::from(path);
    let is_home = path.starts_with(&home_dir);

    match mode {
        PathDisplayMode::Full => path.to_string(),
        
        PathDisplayMode::Short => {
            if is_home {
                path.replacen(&home_dir, "~", 1)
            } else {
                path.to_string()
            }
        },
        
        PathDisplayMode::ShortAll => {
            if path == home_dir {
                return "~".to_string();
            }

            let mut result = String::new();
            let components: Vec<_> = path_buf.components().collect();

            for (i, component) in components.iter().enumerate() {
                let part = component.as_os_str().to_string_lossy();
                
                if i == 0 && is_home {
                    result.push('~');
                    continue;
                }

                if !part.is_empty() {
                    if !result.is_empty() && !result.ends_with('/') {
                        result.push('/');
                    }

                    if i < components.len() - 1 {
                        // Для всех кроме последнего - первый символ
                        if let Some(c) = part.chars().next() {
                            result.push(c);
                        }
                    } else {
                        // Последний компонент полностью
                        result.push_str(&part);
                    }
                }
            }

            result
        },
        
        PathDisplayMode::Current => {
            path_buf.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("?")
                .to_string()
        }
    }
}

pub fn render_prompt(config: &ShellConfig, ctx: &PromptContext) -> String {
    let mut result = config.prompt.template.clone();

    // Заменяем переменные
    result = result.replace("$time", &ctx.time);
    result = result.replace("$err", &ctx.last_error.to_string());
    result = result.replace("$host", &ctx.hostname);
    result = result.replace("$user", &ctx.username);
    result = result.replace("$path", &ctx.path);
    result = result.replace("$status_icon", &ctx.status_icon);

    // Применяем цвета и стили
    for (key, value) in &config.colors {
        result = result.replace(&format!("${}", key), value);
    }

    result
}
