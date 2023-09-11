use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::process;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret_number}");

    loop {
        let guess = get_guess();
        
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn get_guess() -> i32 {
    println!("Please input your guess (or type quit):");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = guess.trim();

        if guess == "quit" {
            process::exit(0);
        }

        match guess.parse() {
            Ok(n) => break n,
            Err(e) => {
                println!("Please enter a number, not '{guess}'. Error: {e}");
            }
        }
    }
}