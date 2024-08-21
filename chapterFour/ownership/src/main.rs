fn main() {
    let s = String::from("hello");

    let mut s2 = s; //s2 replaces s in the stack

    s2.push_str(", world!");

    takes_ownership(s2);

    let x = 5;

    makes_copy(x);

    // println!("{s2}"); Does not work because scope was transfered to takes_ownership and it is drop once that function ends
    println!("{x}") //Works because x is copied to makes_copy not transfered

}//Rust calls the drop function here 
//s and s2 are out of scope

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}//s2 is dropped here


fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}