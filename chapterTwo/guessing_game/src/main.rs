use std::io;//Input output library from the standard library
use rand::Rng; //random
use std::cmp::Ordering; //for comparing

fn main(){
    println!("Guess the number!");
    //Inclusive of both bounds
    let secret_number = rand::thread_rng().gen_range(1..=100);    

    //For debugging purposes
    //println!("The secret number is {secret_number}");

    loop {

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

        //trim takes out white space, parse converts one type to another
        //checks if result matches ok or err then proceeds
        //_ is a catch all value
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        //can be rewritten as
        //io::stdin().read_line(&mut guess).expect("Failed to read line")

        println!("You guessed: {}", guess);

        //match does the function that matches what is returned
        match guess.cmp(&secret_number){
            //3 outcomes when comparing
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too BIG!"),
            Ordering::Equal => {
                println!("YOU WIN!");
                break;
            }
        }
    }
}