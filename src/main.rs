use std::fs::File;
use std::io;
use std::io::Write; // required to flush stdout
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
    /// print average as whole number
    #[structopt(short, long)]
    int: bool,
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
    let avg = sum / numbers.len() as NumType;

    println!("numbers: {:?}", numbers);
    if opt.int {
        println!("average: {}", avg.round());
    } else {
        println!("average: {}", avg);
    }
}

fn get_numbers_from_user<T: FromStr>() -> io::Result<Vec<T>> {
    let mut numbers: Vec<T> = Vec::new();

    loop {
        print!("Enter a number: ");
        let _ = io::stdout().flush();

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
            let trimmed = line.trim_end();

            match trimmed.parse::<T>() {
                Ok(n) => LineResult::Number(n),
                Err(_) => {
                    if trimmed == "" {
                        LineResult::StopIteration
                    } else {
                        LineResult::SkipLine(String::from(trimmed))
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::matches;

    #[test]
    fn test_file() {
        let file = PathBuf::from_str("numbers.txt").unwrap();

        assert_eq!(
            get_numbers_from_file::<f64>(file).unwrap(),
            vec![1.0, 2.2, 3.0, 9.0]
        );
    }

    #[test]
    fn test_reads() {
        let file = PathBuf::from_str("numbers.txt").unwrap();
        let f = File::open(file).unwrap();
        let mut reader = io::BufReader::new(f);

        let mut expected_results: Vec<LineResult<f64>> = Vec::with_capacity(6);
        expected_results.push(LineResult::Number(1.0));
        expected_results.push(LineResult::Number(2.2));
        expected_results.push(LineResult::SkipLine(String::from("a")));
        expected_results.push(LineResult::Number(3.0));
        expected_results.push(LineResult::Number(9.0));
        expected_results.push(LineResult::StopIteration);

        for _expected_result in expected_results {
            assert!(matches!(
                read_and_parse_number::<f64>(&mut reader),
                _expected_result
            ));
        }
    }
}
