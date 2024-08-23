#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//Method
impl Rectangle { // Can have multiple impl blocks but not appropriate here
    fn area(&self) -> u32 { //&self is shorthand for self: &self
        self.width * self.height
    }

    fn can_hold(&self, rect2: &Rectangle) -> bool { 
        self.width > rect2.width && self.height > rect2.height
    }

    fn square(side: u32) -> Self { //Associated function
        Self {
            height:side,
            width:side,
        }
    }
}
fn main() {
    let my_rect = Rectangle {
        height: 89,
        width: 57,
    };
    let my_rect2 = Rectangle {
        height: 77,
        width: 33,
    };

    let rect3 = Rectangle::square(3);  //:: is used to call assosiated functions instead of dot notation
    println!("Can rect2 fit in rect1? {}", my_rect.can_hold(&my_rect2));

    println!("{}", my_rect.area());
}
