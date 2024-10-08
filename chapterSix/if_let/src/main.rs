fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    //Less boiler plate, used for when there is only one condition
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    } else {
        println!("There is no maximum configured right now");
    }
}
