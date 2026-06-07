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
/*
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
*/


//
//  The Option<T> match Pattern
//


/*
fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(v) => {
                println!("Value Before: {}", v);
                Some(v + 1)
            },
        }
    }
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    let seven = plus_one(six);


    /*  own creation  */
    enum HasOwner {
        Homeless,
        WithOwner,
    }
    enum Dogs {
        Chiwawa(HasOwner),
        Tackel(HasOwner),
        Husky(HasOwner),
        Shiba(HasOwner),
    }

    fn up_for_adoption(owner: HasOwner) -> String {
        match owner {
            HasOwner::Homeless => String::from("Can adopt"),
            HasOwner::WithOwner => String::from("Cannot adopt"),
        }
    }

    fn what_dog(dog: Dogs) -> String {
        match dog {
            Dogs::Chiwawa(owner) => {
                let status = up_for_adoption(owner);
                println!("{} Chiwawa", status);
                String::from("Chiwawa")
            }
            Dogs::Tackel(_) => String::from("Tackel"),
            Dogs::Husky(_) => String::from("Husky"),
            Dogs::Shiba(_) => String::from("Shiba"),
        }
    }

    let yeti = Dogs::Chiwawa(HasOwner::WithOwner);
    let result = what_dog(yeti);
    println!("{}", result);
}
*/

//
//  Matches are exhaustive
//

// meaning every possible switch case must be handeld 
// i mean match case**
/*
fn main() {
    // so as an exmaple here well use the Option<T> 
    // and only make a case for some
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x { // <-- here we see that None is not coverd
            Some(i) => Some(i + 1),
        }
    }
}
*/

//
//  Catch all patterns with _
//

// so we can use 'other' and '_'
// the diffrence being that if we use _ then rust is like okay we wont use the value again
// while if we use other rust will keep the value in a var thats why we use other in the move_player(other) fn
// also 'other' can be any name
/*
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => rerrol(),
        // other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn rerrol() {}
    // fn move_player(num_spaces: u8) {}
}
*/
// the tuple unit type in match

fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), // unit type
        // meaning we dont want to run anything if its not a 3 or 7
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
}