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

    println!("{}", start_dir.display());
    find_recursive(&start_dir, &pattern, "")
}

fn find_recursive(dir: &Path, pattern: &str, prefix: &str) -> io::Result<()> {
    if dir.is_dir() {
        // Read dir entries and collect them into a vector
        let entries = fs::read_dir(dir)?
            .filter_map(|entry| entry.ok())
            .collect::<Vec<_>>();

        for (i, entry) in entries.iter().enumerate() {
            let path = entry.path();
            let file_name = path
                .file_name()
                .unwrap()
                .to_string_lossy();

            let is_last = i == entries.len() - 1;
            let new_prefix = format!("{}{}", prefix, if is_last { "└── " } else { "├── " });

            if path.is_dir() {
                println!("{}{}/", new_prefix, file_name);
                let child_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
                find_recursive(&path, pattern, &child_prefix)?;
            } else {
                if pattern.is_empty() || path.to_string_lossy().contains(pattern) {
                    println!("{}{}", new_prefix, file_name);
                }
            }
        }
    }

    Ok(())
}
