use std::io;
use std::io::prelude::Write;

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    stdout.write(b"Number: ")?;
    stdout.flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    match input.trim().parse::<usize>() {
        Ok(number) => println!("{:}! = {:}", number, factorial(number)),
        Err(e) => println!("Error parsing input: {}", e),
    };

    Ok(())
}

fn factorial(num: usize) -> usize {
    match num {
        1 => 1,
        _ => num * factorial(num - 1),
    }
}
