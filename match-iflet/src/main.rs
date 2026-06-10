/*
fn main() {
    // this is very normal very demure very mindful
    /*
    let config_max = Some(Option<3u8>);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}");
        _ => (),
    }
    */
    let config_max = Some(Option<3u8>);
    // using the if let syntax makes it so we dont need the none case or any other case
    // aka     _ => (),
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
    // so its just an if statement but it checks a
    // match case,
}
*/


//
//  in the coin exmaple 
//

[derive(Debug)]
enum Ustate {
    Alabama
    Alaska
    Arizona
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(Ustate),
}

fn main() {
    let coin = Coin:Penny;
    let mut count = 0;
    // match version
    /*
    match coin {
        Coin::Quarter(state) => println!("State quarter from {state:?}"),
        _ => count += 1,
    }
    */
    // if let version
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}");
    } else {
        count += 1;
    }
}

//
//  The Happy path  let else
//