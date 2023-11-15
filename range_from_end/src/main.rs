fn main() {
    let a = vec![0, 1, 2, 3, 4, 5];

    let b = &a[2..];

    println!("{:?}", b);
}
