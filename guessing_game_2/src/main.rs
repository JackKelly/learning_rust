use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please enter your guess: (an integer between 1 and 100)");

    let guess = get_guess();

    println!("You guessed: {guess}");
}

fn get_guess() -> String {
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess) 
        .expect("Failed to read line");

    guess
}