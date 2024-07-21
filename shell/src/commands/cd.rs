use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

pub fn cd(args: &[String], current_dir: &mut PathBuf) -> io::Result<()> {
    if args.is_empty() {
        eprintln!("cd: missing operand");
        return Ok(());
    }

    let new_dir = if args[0] == ".." {
        // Handle the `..` case to move up one directory level
        current_dir
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| PathBuf::from("/"))
    } else if args[0].starts_with("/") {
        // Handle absolute paths
        PathBuf::from(&args[0])
    } else {
        // Handle relative paths
        current_dir.join(&args[0])
    };

    // Check if the directory exists and is accessible
    match fs::metadata(&new_dir) {
        Ok(metadata) => {
            if metadata.is_dir() {
                match env::set_current_dir(&new_dir) {
                    Ok(_) => {
                        *current_dir = new_dir.canonicalize()?;
                    },
                    Err(_) => {
                        eprintln!("cd: permission denied: {}", args[0]);
                    },
                }
            } else {
                eprintln!("cd: not a directory: {}", args[0]);
            }
        },
        Err(_) => {
            eprintln!("cd: no such file or directory: {}/{}", current_dir.display(), args[0]);
        }
    }

    Ok(())
}
