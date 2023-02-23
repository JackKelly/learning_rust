fn main() {
    let config_max = Some(5u8);

    match config_max {
        Some(max) => println!("The max is {}", max),
        _ => (),  // Do nothing
    }

    // Read as "if `let` destructures `config_max` to `Some(max)`
    // then execute the code block."
    if let Some(max) = config_max {
        println!("The max is {}", max);
    }
}
