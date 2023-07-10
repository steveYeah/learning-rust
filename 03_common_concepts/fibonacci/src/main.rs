use std::io;

fn main() {
    loop {
        println!("Enter a number ");
        let mut seq_n = String::new();

        io::stdin()
            .read_line(&mut seq_n)
            .expect("Failed to read lines");

        let seq_n: u64 = match seq_n.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You must enter a positive number");
                continue;
            }
        };

        let fib_n = get_fib_n(&seq_n);

        println!("The {seq_n} of the fibonacci sequence is: {fib_n}");
    }
}

fn get_fib_n(seq_n: &u64) -> u64 {
    let seq_n = *seq_n;

    if seq_n == 1 {
        return 0;
    }

    if seq_n == 2 {
        return 1;
    }

    let mut n2 = 0;
    let mut n1 = 1;
    let mut nth = 0;

    for _ in 0..seq_n - 2 {
        nth = n2 + n1;
        n2 = n1;
        n1 = nth;
    }

    nth
}
