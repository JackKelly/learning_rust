#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Self) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let r = Rectangle{width: 10, height: 20};
    dbg!(&r);
    println!("{:?} has area = {}", r, r.area());

    let r2 = Rectangle{width: 5, height: 5};
    println!("Can {:?} hold {:?}? {}", r, r2, if r.can_hold(&r2) {"yes"} else {"no"});
}
