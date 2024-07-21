use std::fs;
use std::io;

pub fn ls(args: &[String]) -> io::Result<()> {
    // If no args provided, use current dir
    let path = if args.is_empty() {
        "."
    } else {
        &args[0]
    };

    // read dir entries
    let entries = fs::read_dir(path)?;

    // iter over entries and print their names
    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy();
        println!("{}", file_name);
    }

    Ok(())
}
