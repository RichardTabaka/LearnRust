use std::fs;
use std::io;
use std::path::Path;

pub fn find(args: &[String], current_dir: &Path) -> io::Result<()> {
    // If no arguments are provided, use the current directory and an empty pattern.
    let (start_dir, pattern) = match args.len() {
        0 => (current_dir.to_path_buf(), String::new()),
        1 => (current_dir.to_path_buf(), args[0].clone()),
        _ => (current_dir.join(&args[0]), args[1].clone()),
    };

    find_recursive(&start_dir, &pattern)
}

fn find_recursive(dir: &Path, pattern: &str) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                // Recursively find in subdirectories
                find_recursive(&path, pattern)?;
            }

            // Print the file or directory if it matches the pattern
            if pattern.is_empty() || path.to_string_lossy().contains(pattern) {
                println!("{}", path.display());
            }
        }
    }

    Ok(())
}
