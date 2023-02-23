fn main() {
    let mut counter = 0;
    let number = loop {
        counter += 1;
        if counter > 5 {
            break counter;
        }
    };
    println!{"{number}"};

    for element in (0..5).rev() {
        println!("{element}")
    }
}
