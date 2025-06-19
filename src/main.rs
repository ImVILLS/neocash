// main.rs

use clap::Parser;
use ctrlc::set_handler;
use neocash::cli::Args;
use neocash::commands;
use neocash::completion::ShellCompleter;
use neocash::config::ShellConfig;
use neocash::prompt::{get_prompt_context, render_prompt};
use neocash::version;
use nix::sys::signal::{SigHandler, Signal, signal};
use rustyline::{
    Config, Context, Editor, Helper, completion::Completer, error::ReadlineError,
    highlight::Highlighter, hint::Hinter, validate::Validator,
};
use semver::Version;
use std::fs;
use std::sync::atomic::{AtomicBool, Ordering};

static RUNNING: AtomicBool = AtomicBool::new(true);

struct ShellHelper {
    completer: ShellCompleter,
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

    // --- COMMAND LINE ARGUMENTS PARSING ---
    let args = Args::parse(); // Using `clap` for argument parsing

    if args.version {
        show_version(args.verbose, args.check_updates);
        return; // Exiting after showing version
    }
    // --- END OF COMMAND LINE ARGUMENTS PARSING ---

    let config = ShellConfig::load(args.config_path);
    println!("Config location: {:?}", config.get_active_config_path());

    let rl_config = Config::builder()
        .completion_type(rustyline::CompletionType::List)
        .build();

    let mut rl = Editor::with_config(rl_config).expect("Failed to create editor");
    let helper = ShellHelper {
        completer: ShellCompleter::new(),
    };
    rl.set_helper(Some(helper));

    let history_path = config.get_history_path();
    if let Some(parent) = history_path.parent() {
        fs::create_dir_all(parent).ok();
    }

    if !args.no_history { // <-- Check for not no_history flag
        if let Some(parent) = history_path.parent() {
            fs::create_dir_all(parent).ok();
        }

        if let Err(e) = rl.load_history(&history_path) {
            eprintln!("No command history: {}", e);
        }
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

                if !args.no_history { // <-- Check for not no_history flag (again)
                    rl.add_history_entry(line).ok();
                }

                if line == "exit" {
                    break;
                }

                last_exit_code = execute_command(line, &config);
            }

            Err(ReadlineError::Interrupted) => {
                println!("Type 'exit' to quit");
            }
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
    })
    .expect("Error setting Ctrl-C handler");
}

fn execute_command(cmd: &str, config: &ShellConfig) -> i32 {
    let expanded_cmd = shellexpand::tilde(cmd).into_owned();

    // Check if the command is a built-in command
    match commands::execute(&expanded_cmd, config) {
        Ok(code) => return code,
        Err(e) if e.contains("No such file or directory") => {
            eprintln!("{}", e);
            return 1;
        }
        Err(_) => {} // Ignore other errors for now
    }

    // If not a built-in command, execute it as a shell command
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

fn show_version(verbose: bool, check_updates: bool) {
    let current_version_full = env!("CARGO_PKG_VERSION");
    // Parsing the full version string to get the base semantic version
    let current_semver_base =
        Version::parse(version::get_base_version_str(current_version_full).as_str())
            .unwrap_or_else(|_| {
                Version::parse("0.0.0").expect("Fallback current semver parse failed")
            });

    println!("NeoCASH v{}", current_version_full);

    if verbose {
        println!(
            "Build: {}",
            std::env::var("PROFILE").unwrap_or_else(|_| "unknown".to_string())
        );
        println!("Repository: https://github.com/ImVILLS/neocash");
    }

    if check_updates {
        match version::check_for_updates() {
            // This function checks for updates in AUR
            Ok(latest_aur_version_full) => {
                let latest_aur_semver_base = Version::parse(
                    version::get_base_version_str(&latest_aur_version_full).as_str(),
                )
                .unwrap_or_else(|_| {
                    Version::parse("0.0.0").expect("Fallback AUR semver parse failed")
                });

                // Compare the base semantic versions
                if current_semver_base < latest_aur_semver_base {
                    // This is the real case when the user needs to update
                    println!(
                        "\n⚠️ Update available: v{} (your version: v{})",
                        latest_aur_version_full, current_version_full
                    );
                    println!("   Run: yay -Syu neocash");
                } else if current_semver_base > latest_aur_semver_base {
                    // This is the case when the user has a newer version than in AUR
                    // This is rare case, but it can happen if the user has built from source or
                    // installed a newer version manually.
                    println!(
                        "\n✨ Your version is newer than on AUR: v{} (AUR: v{})",
                        current_version_full, latest_aur_version_full
                    );
                } else {
                    // current_semver_base == latest_aur_semver_base
                    // Base versions are equal, so we can assume that the user is up-to-date.
                    println!("\n✓ You're up-to-date!");
                }
            }
            Err(e) => println!("\n⚠️ Update check failed: {}", e),
        }
    }
}
