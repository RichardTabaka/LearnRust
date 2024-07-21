use std::io::{self, Write};

mod commands;

fn main() {
    loop {
        print!("> ");
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
