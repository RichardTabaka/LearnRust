use std::env;
use std::io::{self, Write};
use std::path::PathBuf;
use dirs::home_dir;

mod commands;

fn main() {
    // shell should start in the home dir
    let mut current_dir = home_dir()
        .unwrap_or_else(|| PathBuf::from("/"));
    env::set_current_dir(&current_dir)
        .expect("Failed to set initial dir to home!");

    loop {
        print!("{}> ", current_dir.display());
        io::stdout()
            .flush()
            .expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read_line");

        if input.is_empty() {
            continue;
        }

        // Split input into cmd and args
        let parts: Vec<&str> = input
            .split_whitespace()
            .collect();
        let command = parts[0];
        let args: Vec<String> = parts[1..]
            .iter()
            .map(|&s| s.to_string())
            .collect();

        // Match cmd, call requested func
        match command {
            "cd" => {
                if let Err(e) = commands::cd(&args, &mut current_dir) {
                    eprintln!("Error: {}", e);
                }
            },
            "echo" => commands::echo(&args),
            "ls" => {
                if let Err(e) = commands::ls(&args) {
                    eprintln!("Error: {}", e);
                }
            },
            "exit" => break,
            _ => eprintln!("Unknown command: {}", command),
        }
    }
}
