fn main() {
    let s = String::from("This is a test!");
    println!("`{}`", first_word(&s));

    let a = [1, 2, 3, 0, 4, 5, 6];
    let a_slice = until_first_zero(&a);
    println!("{}", a_slice.len());
}

fn first_word(s: &str) -> &str {
    for (i, &char) in s.as_bytes().iter().enumerate() {
        if char == b' ' {
            return &s[..i];
        }
    }
    s
}

fn until_first_zero(a: &[i32]) -> &[i32] {
    for (i, &element) in a.iter().enumerate() {
        if element == 0 {
            return &a[..i];
        }
    }
    return a;
}