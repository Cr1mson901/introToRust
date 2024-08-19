fn main() {
    //Scaler types: Represents a single value
        //INT, FLOAT, BOOL, CHAR
    //addition
    let sum = 5 + 10;

    //subtraction
    let difference = 94.5 - 4.3;

    //multiplication
    let product = 4 * 30;

    //division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // returns -1

    //remainder / mod
    let remainder = 43 % 5;

    //Bool
    let t = true;
    let f: bool = false;

    //Char
    let c = 'z'; // Single quotes over double quotes
    let z: char = 'Z';
    let heart_eyed_cat = '😻';

    //Compound Types
        //Tuple: set length
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    //Deconstruction
    let (x,y,z) = tup;
    //Indexed into tup
    let five_hundred = tup.0;
    println!("This is the first value of the tuple: {five_hundred}");
    println!("The value of y is: {y}");

    //Array fixed length, must be the same type
    let a = [1,2,3,4,5];

    let a: [i32; 5] = [1,2,3,4,5];//Declaritive

    let b = [4; 5];//5 fours
    println!("This is an array of 4's: {}{}{}{}{}", b[0], b[1], b[2], b[3], b[4]);
}
