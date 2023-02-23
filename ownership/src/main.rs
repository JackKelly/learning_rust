fn main() {
    let s = String::from("Hello!");

    let len = calculate_length(&s);

    println!("The length of `{s}` is {len}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
