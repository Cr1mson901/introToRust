#[derive(Debug)] // Need to add this so that rectangle can be printed in debug mode
struct Rectangle{
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let my_rect = Rectangle{
        width: dbg!(30 * scale), //prints the formula and returns the result
        height: 83,
    };

    println!("The area of my rectangle is {}", area(&my_rect));

    println!("my_rect is {my_rect:?}"); //:? prints the structure in debug mode
    // :#? prints in a bit more readable fromat

    dbg!(&my_rect); //Another way to print for debugging, use a ref cause we don't want dbg to take ownership
}

fn area (rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
