enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState), //Taking another enum as a value
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arkansas,
    Arizona,
    California,
    Colorodo,
    Conneticut,
    Florida,
    Georgia,
    Kansas,
    Michigan,
}


fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other), //This is used for all remaining possibilities, must be last
        // _ => (), Use _ if you don't intend on using the variable, () is for when nothing happens
    }
}

//Returns based on type of coin
fn value_in_cents (coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }//Comma unneeded maybe, yet to be confirmed
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

//Uses match to check if option is null or not
fn plus_one (x: Option<i32>) -> Option<i32> {
    match x { //Must cover all cases possible
        None => None,
        Some(i) => Some(i + 1),
    }
}

