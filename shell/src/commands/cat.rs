use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn cat(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        eprintln!("cat: missing operand");
        return Ok(());
    }

    // foreach file, open it and print every line
    for filename in args {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            println!("{}", line?);
        }
    }

    Ok(())
}