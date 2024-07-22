use std::fs::File;
use std::io::{self, BufRead, BufReader};
use regex::Regex;

pub fn grep(args: &[String]) -> io::Result<()> {
    if args.len() < 2 {
        eprintln!("grep: missing pattern and/or file");
        return Ok(());
    }

    // load the pattern, do regex dark magic
    let pattern = &args[0];
    let re = Regex::new(pattern).map_err(|e| {
        eprintln!("grep: invalid regex pattern: {}", e);
        io::Error::new(io::ErrorKind::InvalidInput, "invalid regex pattern")
    })?;

    // check every line in the files
    for filename in &args[1..] {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        for (index, line) in reader.lines().enumerate() {
            let line = line?;
            if re.is_match(&line) {
                println!(" -{}: line {}: {}", filename, index + 1, line);
            }
        }
    }

    Ok(())
}
