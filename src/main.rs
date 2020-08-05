use std::io;
use std::io::prelude::*;
use std::iter::Sum;
use std::str::FromStr;

// cli to use a number file?

fn main() {
    let numbers = get_numbers_from_user::<f64>().unwrap();
    let sum: f64 = Sum::sum(numbers.iter());

    println!("numbers: {:?}", numbers);
    println!("average: {}", sum / numbers.len() as f64)
}

fn get_numbers_from_user<T: FromStr>() -> io::Result<Vec<T>> {
    let mut numbers: Vec<T> = Vec::new();

    loop {
        print!("Enter a number: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let num = input.trim_end();

                if num == "" {
                    break;
                }
                match num.parse::<T>() {
                    Ok(n) => numbers.push(n),
                    Err(_) => println!("Invalid input: {}", num),
                }
            }
            Err(error) => {
                println!("io read error: {}", error);
                break;
            }
        }
    }

    Ok(numbers)
}
