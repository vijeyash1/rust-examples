#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle::square(80);

    println!("can rect2 hold rect1? {}", rect2.can_hold(&rect1));
}

// can_hold method receives self and another rectangle as arguments
// square is an associated function of Rectangle struct
// to use it we need to use Rectangle::square(...)
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
