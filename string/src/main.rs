fn main() {
    let data = "initial contents";
    let mut s = data.to_string();
    s.push_str(" Yo!");
    let sa = s + &String::from("yo yo yo");
    println!("`{sa}`");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s4 = format!("{s1}-{s2}-{s3}");
    println!("{s4}");

    for c in "Зд".chars() {
        println!("{c}");
    }
}
