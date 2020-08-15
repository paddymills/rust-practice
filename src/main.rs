use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() {
    match read_file("text.txt") {
        Ok(text) => {
            {
                let words = text.split_whitespace().count();
                println!("Words: {}", words);
            }

            {
                let vowels = text
                    .chars()
                    .filter(|&x| is_val(&x, &vec!['a', 'e', 'i', 'o', 'u']))
                    .collect::<Vec<char>>()
                    .len();
                println!("Vowels: {}", vowels);
            }

            {
                let v = text
                    .chars()
                    .filter(|&x| is_val(&x, &vec!['V', 'v']))
                    .collect::<Vec<char>>()
                    .len();
                println!("The letter V: {}", v);
            }
        }
        _ => println!("Error reading file"),
    };
}

fn read_file(filename: &str) -> io::Result<String> {
    let mut f = File::open(filename)?;
    let mut res = String::new();

    f.read_to_string(&mut res).unwrap();

    Ok(res)
}

fn is_val(letter: &char, chars: &Vec<char>) -> bool {
    if chars.contains(letter) {
        return true;
    }

    false
}
