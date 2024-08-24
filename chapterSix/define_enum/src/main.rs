//Can put any type of data into an enum
enum IpAddrKind{
    V4(u8, u8, u8, u8),
    V6(String),
}

//Equivalent of 4 structs all grouped together under Message
enum Message {
    Quit,
    Move { x:i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        //method body defined here
    }
}

// How null is implemented in rust, included in the prelude so you don't need to bring it into scope
enum Option<T> { //<T> is a generic place holder for variable types
    None, //Can use these without Option::
    Some(T),
}
// Using Structs to pair data
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

fn main() {
    //Both are the same type so can be called by a function
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));
    
    let m = Message::Write(String::from("hello"));
    m.call();

    //Both of these can go into route function as they are part of the same enum
    route(four);
    route(six);

    // Using option values to hold number types and string types
    let some_number: Option<i32> = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None; //Must define the type as rust doesn't know based on value None

    let another_number: i32 = 8;

    // let total = some_number + another_number This does not work as they are two different types
    //Must address the case in which option is null before you can use it
}

fn route(ip_kind: IpAddrKind) {

}