use rand::Rng;
use rand::distributions::uniform::SampleRange;
use std::cmp::Ordering;
use std::io;
use std::num::{ParseIntError, IntErrorKind};
use std::ops::{Range, RangeInclusive};
use std::str::FromStr;

enum ResultOfGuess {
    Ok(u32),
    Err(<u32 as FromStr>::Err),
    OutOfRange,
    Quit,
}

fn main() {
    println!("Guess the number!");

    let range = 1..=100;
    let secret_number = rand::thread_rng().gen_range(range.clone());

    println!("The secret number is: {secret_number}");

    let mut valid_attempts = 0;

    loop {
        let guess = match get_guess(&range) {
            ResultOfGuess::Ok(n) => n,
            ResultOfGuess::Quit => {
                break;
            },
            ResultOfGuess::Err(_) | ResultOfGuess::OutOfRange => {
                continue;
            },
        };

        valid_attempts += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win (in {valid_attempts} valid attempts)!");
                break;
            },
        }
    }
}

fn get_guess(range: &RangeInclusive<u32>) -> ResultOfGuess {
    let mut guess = String::new();

    println!("Please enter your guess: (an integer between {} and {})", range.start(), range.end());
    println!("Or type `q` to quit.");

    io::stdin()
        .read_line(&mut guess) 
        .expect("Failed to read line");

    let guess = match guess.trim() {
        "q" => {
            println!("Quit!");
            return ResultOfGuess::Quit;
        },
        n => n,
    };

    match guess.parse() {
        Ok(n) => {
            if n < *range.start() || n > *range.end() {             
                println!("Your guess must be between {} and {}! Try again!", range.start(), range.end());
                ResultOfGuess::OutOfRange
            } else {
                ResultOfGuess::Ok(n)
            }
        }
        Err(e) => {
            println!("Error parsing string to int: '{e}'.");
            println!("Please try again!\n");
            ResultOfGuess::Err(e)
        }
    }
}