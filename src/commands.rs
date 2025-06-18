// commands.rs

use std::process::{Command, Stdio};
use crate::config::ShellConfig;

pub fn execute(cmd: &str, config: &ShellConfig) -> Result<i32, String> {
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    if parts.is_empty() {
        return Ok(0);
    }

    match parts[0] {
        "exit" => std::process::exit(0),
        "cd" => {
            let path = parts.get(1).unwrap_or(&"~");
            std::env::set_current_dir(path).map_err(|e| e.to_string())?;
            Ok(0)
        },
        "edit" => {
            let file = parts.get(1).ok_or("No file specified")?;
            Command::new(&config.prompt.default_editor)
                .arg(file)
                .status()
                .map(|s| s.code().unwrap_or(1))
                .map_err(|e| e.to_string())
        },
        _ => execute_system_command(parts),
    }
}

fn execute_system_command(parts: Vec<&str>) -> Result<i32, String> {
    let mut cmd = Command::new(parts[0]);
    cmd.args(&parts[1..])
       .stdin(Stdio::inherit())
       .stdout(Stdio::inherit())
       .stderr(Stdio::inherit());

    cmd.status()
       .map(|s| s.code().unwrap_or(1))
       .map_err(|e| e.to_string())
}
