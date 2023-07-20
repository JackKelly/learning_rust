fn main() {
    let s1 = "This is a test of a string \
        that spans multiple lines \
        and that is exciting!";
    println!("{}\n\n", s1);

    let s2 = r###"This is a raw string that can contain escape
        sequences like \!{"}"###;
    println!("{}\n\n", s2);

    let s3 = "This is another string
        that spans multiple
        lines";
    println!("{}\n\n", s3);

    let s4 = b"This is a byte string";
    println!("{:?}", s4);

    string_counts("Hello");
    string_counts("Ӥ");
}

fn string_counts(s: &str) {
    println!("'{}' has {} bytes and {} chars.", s, s.len(), s.chars().count());
}