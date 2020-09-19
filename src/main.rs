#![feature(int_error_matching)]

use std::io;
use std::io::prelude::Write;
use std::num::IntErrorKind;

fn main() -> io::Result<()> {
    match input() {
        Ok(0) => println!("Aborting..."),
        Ok(num) => println!("{:}! = {:}", num, factorial(num)),
        Err(_) => println!("Aborting..."),
    }

    println!("{}", factorial(5));

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
            Err(e) => match e.kind() {
                IntErrorKind::Empty => println!("Empty string, abort!"),
                IntErrorKind::InvalidDigit => println!("not a number"),
                _ => println!("err: {:?}", e),
            },
        };

        iters += 1;
        if iters > 2 {
            break;
        }
    }

    Ok(0)
}

fn factorial(num: i32) -> i32 {
    match num {
        1 => 1,
        _ => num * factorial_functional(num - 1),
    }
}
