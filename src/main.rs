use std::cmp::Ordering;
use std::{io,process};
use rand::Rng;

fn main() {
    let random_number = rand::thread_rng().gen_range(1..=10);

    let mut i: u32 = 0;
    loop {
        println!("Guess the number (attempt {})!", i + 1);
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read guess");

        let trimmed_guess = guess.trim_end();

        let guess_as_int = match trimmed_guess.parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input numbers only.");
                continue;
            }
        };

        println!("Your guess is {guess_as_int}.");

        match guess_as_int.cmp(&random_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => exit(),
        }
        i += 1;
    }
}

fn exit() {
    println!("You win!");
    process::exit(0x0100);
}
