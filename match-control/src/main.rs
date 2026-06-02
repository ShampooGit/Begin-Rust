//
// Match statements
//
/*
the big moment ive been waiting for kraaaa,
im really excited about this section
after this chapter i will prob try to make some functional application
*/

/*
fn main() {

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            // => is a seperator for the pattern (Coin::Penny) and the actual 
            // runinng code (1)
            // the code in a pattern is an expression
            Coin::Penny => {
                println!("Lucky Penny!");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
    // so the big diffence with an if is this can return any value 
    // wich is pretty nice 
}
*/
//
//  Patterns that bind to values
//

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter form {state:?}!");
            25
        }
    }
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Arizona));
    // damn thats alot of work just to print one line
    // but also really nice this way i can make it 
    // more responsive / adaptive
}


//
//  The Option<T> match Pattern
//