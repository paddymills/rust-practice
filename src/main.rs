use rand::prelude::*;

fn main() {
    const LOW: u8 = 0; // inclusive
    const HIGH: u8 = 10; // exclusive

    let mut rng = thread_rng();
    for a in 1..10 {
        let x = rng.gen_range(LOW, HIGH);
        println!("iteration {}: {}", a, x);
    }
}
