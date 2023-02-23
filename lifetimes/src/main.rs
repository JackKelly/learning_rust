fn main() {
    let strings = vec!["abcd", "xyz"];

    let result = longest(&strings);
    println!("The longest string is '{}'.", result);
}


fn longest<'a>(strings: &'a [&str]) -> &'a str {
    let mut longest_str: &str = "";
    for &str in strings {
        if str.len() > longest_str.len() {
            longest_str = str;
        }
    }
    longest_str
}
