#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn square(size: u32) -> Self {
        Self{width: size, height: size}
    }
}

fn main() {
    let rectangle = Rectangle{
        width: dbg!(100),
        height: 400,
    };

    println!("Area of {:#?} = {}", rectangle, rectangle.area());

    let square = Rectangle::square(10);
    println!("{:#?}", square);
}
