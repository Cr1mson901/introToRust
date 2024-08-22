fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&s1); // use reference to not move variable

    println!("The length of {s1} is {len}");

    change(&mut s1);//Not more then one mut reference can be made for a variable at a time
    //Can not have a mut and immut ref at the same time
    //Multiple immut ref are fine

    println!("{s1}"); //Hello, world!
}

fn calculate_length(s: &String) -> usize { // Must include reference with the string
    s.len() //Don't use ; as it will be a statment and wont return
}

fn change(s:&mut String){
    s.push_str(", world!") // Changes the mutable reference
}

// fn dangle() -> String {
//     let s = String::from("Wrong");

//     &s This doesn't work because it returns a reference to a value that doesn't exist anymore cause it left scope
// }