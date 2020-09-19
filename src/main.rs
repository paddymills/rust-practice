use std::io;
use std::io::prelude::Write;
use std::num::IntErrorKind;

fn main() -> io::Result<()> {
    let num = input();

    Ok(())
}

fn input() -> io::Result<i32> {
    let mut stdout = io::stdout();
    let stdin = io::stdin();

    let mut iters = 0;
    loop {
        let mut input = String::new();
        stdout.write(b"n!: n=")?;
        stdout.flush()?;

        stdin.read_line(&mut input)?;
        match input.trim().parse::<i32>() {
            Ok(number) => println!("n= {}", number),
            Err(e) => {
                println!("err: {:?}", e);
                match e.kind() {
                    IntErrorKind::Empty => println!("Empty string, abort!"),
                    IntErrorKind::InvalidDigit => println!("not a number"),
                }
            }
        };

        iters += 1;
        if iters > 2 {
            break;
        }
    }

    Ok(0)
}
