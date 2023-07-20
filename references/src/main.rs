fn main() {
    let mut n = 48;
    {
        let rn = &mut n;

        *rn += 1;

        println!("{}", rn);
    }

    println!("{}", n);
}
