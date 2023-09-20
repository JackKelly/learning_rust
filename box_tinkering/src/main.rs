fn main() {
    let b = Box::new(5i32);

    // let b_abs = i32::abs(*b);

    println!("{}, {}", b.abs(), b.abs());
}
