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

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coin = Coin::Penny;
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

// v1
// this version does alot in once and is hard to read 
// thats why v2 is a thing 
// this really is complex T-T
/*
fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relativley new."))
        }
    } else {
        None
    }
}
*/
// v2
// here we assign the if let to another value early,
// so the state coin lives inside the var state now.
// but this is apperantly still to much to write so there is a v3
/*
fn describe_state_quarter(coin: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };
    // i do get the v3 from this one tho cuz there are 2 ifs that are connected
    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relativley new."))
    }
}
*/
// v3
// now id where let...else comes in we start with let
// ... represents code 
// else is well the else part
// the let...else does not have an if breabjc only a else branch

fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America"))
    } else {
        Some(format!("{state:?} is relativley new."))
    }
}
// this if let and let...else are tools for the match 
// pretty hgandy but i do feel liek theese are pretty complex for what they do
// there is just so much to remember 

impl UsState {
    fn existed_in(&self, year: u16) -> bool{
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            UsState::Arizona => year >= 1912,
            _ => false,
        }
    }
}