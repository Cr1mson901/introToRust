//Constants can be declared globaly
//Must be a constant expression not something calculated at run time
const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;

fn main(){
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is {x}");

    //Shadowing
    let y = 5;

    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope {y}");
    }

    println!("The value of y is {y}");

    //Conversion: Can not change a variables type. Must make a new variable with same name
    let spaces = "     ";
    let spaces = spaces.len();
    println!("{spaces} spaces");
}