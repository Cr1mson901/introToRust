use std::io;//Input output library from the standard library

fn main(){
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();
    //let is used to declare variables
    //mut makes the var mutable: Immutable by default

    //similar to python but use . as attribute functions
    io::stdin()
        .read_line(&mut guess)//Changes the input into the users input. Needs to be mut
        //& makes it a reference so you don't need to pull it into memory. References are immutable by default 
 
        //Returns string if error is returned from .expect
        .expect("Failed to read line");

    //can be rewritten as
    //io::stdin().read_line(&mut guess).expect("Failed to read line")

    println!("You guessed: {}", guess);
}