// add some imports

fn main() {
    let test = String::from("ABBEACEEA");
    let mut k = 0;
    let len = test.len();

    for mut c in test.chars() {
        if !c.is_alphabetic() {
            panic!("Expected alphabetic character, got {}", c);
        }

        c.make_ascii_uppercase();

        let c_k = c as u32 - 64;
        if c_k > k {
            k = c_k;
        }
    }

    //should be k=5, len=9
    println!("{} is a string of k: {} and length: {}", test, k, len);
}
