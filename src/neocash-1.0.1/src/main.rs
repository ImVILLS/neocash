use std::fs;
use std::sync::atomic::{AtomicBool, Ordering};
use ctrlc::set_handler;
use nix::sys::signal::{signal, SigHandler, Signal};
use rustyline::{
    Editor, 
    Config,
    error::ReadlineError,
    completion::Completer,
    highlight::Highlighter,
    hint::Hinter,
    validate::Validator,
    Helper, 
    Context,
};
use neocash::config::ShellConfig;
use neocash::prompt::{get_prompt_context, render_prompt};
use neocash::completion::ShellCompleter;

static RUNNING: AtomicBool = AtomicBool::new(true);

struct ShellHelper {
    completer: ShellCompleter,
    config: ShellConfig,
}

impl Helper for ShellHelper {}
impl Completer for ShellHelper {
    type Candidate = <ShellCompleter as Completer>::Candidate;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        self.completer.complete(line, pos, ctx)
    }
}

impl Highlighter for ShellHelper {}
impl Hinter for ShellHelper {
    type Hint = String;
}
impl Validator for ShellHelper {}

fn main() {
    setup_signal_handlers();
    
    let config = ShellConfig::load();
    println!("Config location: {:?}", ShellConfig::get_config_path());
    
    let rl_config = Config::builder()
        .completion_type(rustyline::CompletionType::List)
        .build();
    
    let mut rl = Editor::with_config(rl_config).expect("Failed to create editor");
    let helper = ShellHelper {
        completer: ShellCompleter::new(),
        config: config.clone(),
    };
    rl.set_helper(Some(helper));

    let history_path = config.get_history_path();
    if let Some(parent) = history_path.parent() {
        fs::create_dir_all(parent).ok();
    }
    
    if let Err(e) = rl.load_history(&history_path) {
        eprintln!("No command history: {}", e);
    }

    let mut last_exit_code = 0;

    while RUNNING.load(Ordering::Relaxed) {
        let context = get_prompt_context(last_exit_code, &config);
        let prompt = render_prompt(&config, &context);

        match rl.readline(&prompt) {
            Ok(line) => {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }

                rl.add_history_entry(line).ok();
                
                if line == "exit" {
                    break;
                }

                last_exit_code = execute_command(line);
            },
            Err(ReadlineError::Interrupted) => {
                println!("Type 'exit' to quit");
            },
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }

    if let Err(e) = rl.save_history(&history_path) {
        eprintln!("Failed to save history: {}", e);
    }
}

fn setup_signal_handlers() {
    unsafe {
        signal(Signal::SIGINT, SigHandler::SigIgn).expect("Error ignoring SIGINT");
    }

    set_handler(move || {
        RUNNING.store(false, Ordering::Relaxed);
    }).expect("Error setting Ctrl-C handler");
}

fn execute_command(cmd: &str) -> i32 {
    let expanded_cmd = shellexpand::tilde(cmd).into_owned();
    
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(&expanded_cmd)
        .spawn();

    match output {
        Ok(mut child) => match child.wait() {
            Ok(status) => status.code().unwrap_or(1),
            Err(e) => {
                eprintln!("Command failed: {}", e);
                1
            }
        },
        Err(e) => {
            eprintln!("Failed to execute command: {}", e);
            1
        }
    }
}
