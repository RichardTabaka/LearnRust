use rustyline::error::ReadlineError;
use rustyline::history::DefaultHistory;
use rustyline::Editor;
use std::env;
use std::path::PathBuf;
use dirs::home_dir;

mod commands;

fn main() {
    // shell should start in the home dir
    let mut current_dir = home_dir()
        .unwrap_or_else(|| PathBuf::from("/"));
    env::set_current_dir(&current_dir)
        .expect("Failed to set initial dir to home!");

    // Apparenly Rust's readline doesn't like backspaces
    let mut rl = Editor::<(), DefaultHistory>::new().unwrap();
    rl.load_history("history.txt").ok();

    loop {
        let prompt = format!("{}> ", current_dir.display());
        let readline = rl.readline(&prompt);

        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());

                let input = line.trim();
                if input.is_empty() {
                    continue;
                }

                let parts: Vec<&str> = input
                    .split_whitespace()
                    .collect();
                let command = parts[0];
                let args: Vec<String> = parts[1..]
                    .iter()
                    .map(|&s| s.to_string())
                    .collect();

                match command {
                    "cat" => {
                        if let Err(e) = commands::cat(&args) {
                            eprintln!("Error: {}", e);
                        }
                    },
                    "cd" => {
                        if let Err(e) = commands::cd(&args, &mut current_dir) {
                            eprintln!("Error: {}", e);
                        }
                    },
                    "clear" => {
                        if let Err(e) = commands::clear() {
                            eprint!("Error: {}", e);
                        }
                    },
                    "echo" => commands::echo(&args),
                    "find" => {
                        if let Err(e) = commands::find(&args, &current_dir) {
                            eprintln!("Error: {}", e);
                        }
                    },
                    "ls" => {
                        if let Err(e) = commands::ls(&args) {
                            eprintln!("Error: {}", e);
                        }
                    },
                    "exit" => break,
                    _ => eprintln!("Unknown command: {}", command),
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
