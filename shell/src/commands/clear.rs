use std::io;

pub fn clear() -> io::Result<()> {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    Ok(())
}
