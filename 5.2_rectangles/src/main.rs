#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height >= other.height && self.width >= other.width
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 60,
        height: 100,
    };

    println!("rect1: {rect1:?} rect2: {rect2:?}");
    println!("rect1 area: {}",rect1.area());
    println!("rect2 area: {}",rect2.area());
    println!("rect1 can hold rect 2: {}",rect1.can_hold(&rect2));
    println!("rect2 can hold rect 1: {}",rect2.can_hold(&rect1));
}