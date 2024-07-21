pub fn echo(args: &[String]) {
    for arg in args {
        print!("{} ", arg);
    }
    println!();
}
