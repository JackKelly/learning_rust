fn main() {
    print_labelled_measurement(50, 'h');
    println!("{}", five());
}

fn print_labelled_measurement(value: i32, unit: char) {
    println!("x={}{}", value, unit);
}

fn five() -> i32 {
    5
}