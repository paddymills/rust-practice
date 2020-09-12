use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "sieve of eratosthenes")]
struct Cli {
    max: usize,
}

fn main() {
    let args = Cli::from_args();

    let mut numbers = vec![0; args.max + 1];
    let mut primes = Vec::<usize>::new();
    for i in 2..args.max + 1 {
        if numbers[i] == 0 {
            primes.push(i);

            let mut x = i * 2;
            while x <= args.max {
                numbers[x] = 1;
                x += i;
            }
        }
    }

    print!("Primes:");
    for elem in primes {
        print!(" {}", elem);
    }
}
