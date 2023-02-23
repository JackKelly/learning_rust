use std::io;

fn main() {
    loop {
        let fahrenheit = get_fahrenheit();
        let celsius = fahrenheit_to_celsius(fahrenheit);
        println!("fahrenheit: {fahrenheit}; celsius: {celsius}")
    }
}

fn get_fahrenheit() -> f64 {
    println!("Please enter fahrenheit:");
    let mut fahrenheit = String::new();

    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Failed to read line!");
    
    let fahrenheit: f64 = fahrenheit.trim().parse().expect("Must enter a number!");
    fahrenheit
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    let celsius = (fahrenheit - 32.0) * 5.0/9.0;
    celsius
}