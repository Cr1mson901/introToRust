fn main() {
    println!("Hello, world!");

    another_function(5, 'm');

    // Statements are instructions that perform some action and do not return a value.
    //can not assign a statement as it does not return anything
    //let y = 6;
    // Expressions evaluate to a resultant value. Can be assigned. Does not have ;
    let y = {
        let x = 5;
        x + 1//The expression: does not end in semicolon or it would be a statment
    };
    println!("{y}");

    let x = five();
    let x = plus_one(x);
    println!("The value of x is: {x}");
}

//Does not matter that it is defined after main
fn another_function(x: i32, unit_label: char){
    println!("The value of x is: {x}{unit_label}");
}

fn five() -> i32 {
    5 // Returns the last expression in the body. Can use return to break early.
}

fn plus_one(x: i32) -> i32{
    x + 1
}