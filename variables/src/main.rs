const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    println!("{THREE_HOURS_IN_SECONDS}");
 
    let y = 50;
    let y = y + 4;

    let mut n = 54u128;
    n = n.saturating_add(40);

    const A: [i32; 5] = [3; 5];
    println!("{}, {}", A[0], A.len());
}
