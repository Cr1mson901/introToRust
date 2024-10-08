fn main() {
    let number = 3;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }
    //if number{} does not work as rust doesn't treat existance as true

    if number % 4 == 0 {
        println!("Number is divisible by 4")
    } else if number % 3 == 0 {
        println!("Number is divisible by 3")
    } else if number % 2 == 0 {
        println!("Number is divisible by 2")
    } else {
        println!("Number is not divisible by 4,3, or 2")
    }

    //Easy conditional assignment
    let condition = true;
    let number = if condition {5} else {6}; //Must be the same type
    println!("{number}");

    let mut counter = 0;

    let result = loop{
        counter += 1;

        if counter == 10{
            break counter * 2
        }
    };//end of the assignment statment so it needs a ;
    println!("{result}");

    //Exiting nested loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;//Calls the tag of the upper most loop. Breaks entire thing
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("{count}");

    //Conditional loops with while
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1//Does not need a ; can use one though
    }
    println!("LiftOff");

    //Looping through a collection with while
    let a = [10,20,30,40,50];
    let mut index = 0;

    while index < 5 {
        println!("the value is {}", a[index]);
        index += 1;
    }

    //Looping through with for
    for element in a {
        println!("the value is {element}");
    }

    //Countdown using for
    for number in (1..4).rev() { //rev is to reverse the order
        println!("{number}");
    }
}
