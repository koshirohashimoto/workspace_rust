#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn squre(size: u32) -> Rectangle {
        Rectangle { width: size, height: size}
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50};
    let rect2 = Rectangle { width: 10, height: 40};
    let rect3 = Rectangle { width: 60, height: 45};
    let sq1 = Rectangle::squre(40);

    println!("rect1 is {:#?}", rect1);
    println!("rect1 holds rect2?: {}", rect1.can_hold(&rect2));
    println!("rect1 holds rect3?: {}", rect1.can_hold(&rect3));
    println!("rect1 holds sq1?:   {}", rect1.can_hold(&sq1));
    println!("rect1: {} sq pixels", rect1.area());
    println!("rect2: {} sq pixels", rect2.area());
    println!("rect3: {} sq pixels", rect3.area());
    println!("sq1:   {} sq pixels", sq1.area());
}