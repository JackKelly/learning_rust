use std::io;

fn main() {
    let integer = get_int();
    let fib = fibonacci(integer);
    println!("The {integer}th fib number is {fib}");
}

fn fibonacci(n: u64) -> u64 {
    let mut previous = [0, 1];
    for _ in 0..n {
        let sum = previous[0] + previous[1];
        previous[0] = previous[1];
        previous[1] = sum;
    }
    previous[0]
}

fn get_int() -> u64 {
    loop {
        let mut number = String::new();
        println!("Please enter an integer!");
        match io::stdin().read_line(&mut number) {
            Ok(_) => println!("Thanks!"),
            Err(_) => { 
                println!("Please enter a valid number!");
                continue;
            }
        }
        match number.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Could not parse '{number}', please enter a valid integer!");
                continue;
            }
        }
    }
}