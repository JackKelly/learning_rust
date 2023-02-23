fn main() {
    let condition: bool = true;

    let number = if condition {5} else {7};

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false")
    }
}
