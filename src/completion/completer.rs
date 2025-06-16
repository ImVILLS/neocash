use std::collections::HashSet;
use std::path::PathBuf;
use std::ffi::OsStr;
use std::fs;
use rustyline::{
    completion::{Completer, Pair},
    Context, Result as RLResult
};
use crate::completion::CompletionMenu;

#[derive(Clone)]
pub struct ShellCompleter {
    current_dir: PathBuf,
}

impl ShellCompleter {
    pub fn new() -> Self {
        Self {
            current_dir: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        }
    }

    /// Returns a list of all commands available in the system's PATH.
    /// Returns a sorted vector of command names.
    /// Uses a HashSet to avoid duplicates.
    pub fn get_all_commands(&self) -> Vec<String> {
        let path_var = std::env::var("PATH").unwrap_or_default();
        let path_dirs = path_var.split(':').map(PathBuf::from);

        let mut commands = Vec::new();
        let mut seen = HashSet::new();

        for dir in path_dirs {
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.filter_map(|e| e.ok()) {
                    if let Some(file_name) = entry.file_name().to_str() {
                        if !seen.contains(file_name) {
                            seen.insert(file_name.to_string());
                            commands.push(file_name.to_string());
                        }
                    }
                }
            }
        }
        commands.sort();
        commands
    }

    pub fn filter_commands(&self, prefix: &str) -> Vec<Pair> {
        self.get_all_commands()
            .into_iter()
            .filter(|cmd| cmd.starts_with(prefix))
            .map(|cmd| Pair { display: cmd.clone(), replacement: cmd })
            .collect()
    }

    fn complete_paths(&self, prefix: &str) -> Vec<Pair> {
        let path = self.current_dir.join(prefix);
        let mut completions = Vec::new();

        if let Some(parent) = path.parent() {
            if let Ok(entries) = fs::read_dir(parent) {
                let file_stem = path.file_stem()
                    .and_then(OsStr::to_str)
                    .unwrap_or("");

                for entry in entries.filter_map(|e| e.ok()) {
                    if let Some(name) = entry.file_name().to_str() {
                        if name.starts_with(file_stem) {
                            let full_path = parent.join(name);
                            let display = if full_path.is_dir() {
                                format!("{}/", name)
                            } else {
                                name.to_string()
                            };
                            
                            completions.push(Pair {
                                display: display.clone(),
                                replacement: display,
                            });
                        }
                    }
                }
            }
        }

        completions.sort_by(|a, b| a.display.cmp(&b.display));
        completions
    }

    fn should_complete_path(&self, line: &str) -> bool {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            return false;
        }

        let last_part = parts.last().unwrap();
        !line.ends_with(char::is_whitespace) &&
        !last_part.contains(|c| matches!(c, '|' | '&' | ';'))
    }

    fn show_completion_menu(&self, items: Vec<Pair>) -> Option<String> {
        let item_strings: Vec<String> = items.iter()
            .map(|p| p.display.clone())
            .collect();
        
        let mut menu = CompletionMenu::new(item_strings);
        menu.show()
    }
}

impl Completer for ShellCompleter {
    type Candidate = Pair;

    fn complete(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> RLResult<(usize, Vec<Pair>)> {
        let prefix = if self.should_complete_path(line) {
            line.split_whitespace().last().unwrap_or("")
        } else {
            line
        };

        let completions = if self.should_complete_path(line) {
            self.complete_paths(prefix)
        } else {
            self.filter_commands(prefix)
        };

        if completions.len() > 1 {
            if let Some(selected) = self.show_completion_menu(completions.clone()) {
                return Ok((pos - prefix.len(), vec![Pair {
                    display: selected.clone(),
                    replacement: selected,
                }]));
            }
        }

        Ok((pos - prefix.len(), completions))
    }
}
