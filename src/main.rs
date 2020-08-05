use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::iter::Sum;
use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;

// cli to use a number file
#[derive(Debug, StructOpt)]
#[structopt(name = "average", about = "Average number generator")]
struct Cli {
    /// Input file
    #[structopt(parse(from_os_str))]
    file: Option<PathBuf>,
}

enum LineResult<T> {
    Number(T),
    StopIteration,
    SkipLine(String),
}

fn main() {
    type NumType = f64;

    let opt = Cli::from_args();
    let numbers: Vec<NumType> = match opt.file {
        Some(f) => get_numbers_from_file::<NumType>(f).unwrap(),
        None => get_numbers_from_user::<NumType>().unwrap(),
    };

    let sum: NumType = Sum::sum(numbers.iter());

    println!("numbers: {:?}", numbers);
    println!("average: {}", sum / numbers.len() as NumType)
}

fn get_numbers_from_user<T: FromStr>() -> io::Result<Vec<T>> {
    let mut numbers: Vec<T> = Vec::new();

    loop {
        print!("Enter a number: ");
        io::stdout().flush().unwrap();

        match read_and_parse_number(&mut io::stdin().lock()) {
            LineResult::Number(n) => numbers.push(n),
            LineResult::StopIteration => break,
            LineResult::SkipLine(val) => {
                println!("Invalid input: {}", val);
                continue;
            }
        }
    }

    Ok(numbers)
}

fn get_numbers_from_file<T: FromStr>(filepath: PathBuf) -> io::Result<Vec<T>> {
    let mut numbers: Vec<T> = Vec::new();

    println!("file: {:?}", filepath);

    let f = File::open(filepath)?;
    let mut reader = io::BufReader::new(f);
    loop {
        match read_and_parse_number(&mut reader) {
            LineResult::Number(n) => numbers.push(n),
            LineResult::StopIteration => break,
            LineResult::SkipLine(val) => {
                println!("Invalid input in file: {}", val);
                continue;
            }
        }
    }

    Ok(numbers)
}

fn read_and_parse_number<T: FromStr>(reader: &mut dyn io::BufRead) -> LineResult<T> {
    let mut line = String::new();
    match reader.read_line(&mut line) {
        Ok(0) => LineResult::StopIteration,
        Ok(_) => {
            let num = line.trim_end();

            match num.parse::<T>() {
                Ok(n) => LineResult::Number(n),
                Err(_) => {
                    if num == "" {
                        LineResult::StopIteration
                    } else {
                        LineResult::SkipLine(line)
                    }
                }
            }
        }
        Err(error) => {
            println!("io read error: {}", error);
            LineResult::StopIteration
        }
    }
}
