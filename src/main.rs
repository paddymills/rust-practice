use std::io;
use std::io::Write; // required to flush stdout

fn main() {
    loop {
        print!("Enter some text: ");
        let _ = io::stdout().flush();

        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                let trimmed = line.trim_end();
                if trimmed == "" {
                    break;
                }

                if is_palindrome(trimmed) {
                    println!("Value is a palindrome!");
                } else {
                    println!("Value is not a palindrome!");
                }
            }

            Err(error) => {
                println!("io read error: {}", error);
                break;
            }
        }
    }
}

fn is_palindrome(val: &str) -> bool {
    let mut v = Vec::from(val.to_lowercase().replace(" ", ""));

    let at = v.len() / 2;
    let (v1, v2) = v.split_at_mut(at);
    v2.reverse();

    let mut zipped = v1.iter().zip(v2.iter());

    if zipped.all(|(&a, &b)| a == b) {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrome() {
        assert_eq!(is_palindrome("Mr owl ate my metal worm"), true);
        assert_eq!(is_palindrome("racecar"), true);
        assert_eq!(is_palindrome("the cake is a lie"), false);
    }
}
