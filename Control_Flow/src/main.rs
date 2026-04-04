// control flow
//
// if expressions
//
//  comparision operator :
//  ==      equal to
//  !=      not equal to
//  <       greater then
//  >       less then
//  <=      greater then or equal to
//  >=      less then or equal to
//
//  logical operator :
//  &&      and
//  ||      or
//  !       not
//
//  RUST RULES 
//  Strict Typing: In C++, you can sometimes compare a character to a number or a float to an int. In Rust, 
//  you cannot. Both sides must be the exact same type.
//  if 5 as f64 == 5.0 (THIS CAN BE DONE) 
//  if 5 == 5.0 (THIS CANNOT BE DONE)
//
//  No "Truthiness": In C++, the number 1 is often treated as true. 
//  In Rust, only true is true. You cannot say if 1 { ... }.
//
//  Booleans only: Logic operators (&&, ||) only work on actual booleans, 
//  not numbers or strings.
//


/*
fn main() {
    let _number = 4;

    if _number < 5 {
        println!("true");
    } else {
        println!("false");
    }
}
*/


/*

    if i need to check a value use == like cpp 

fn main() {
    let _number = 3;

    if _number == 3 {
        println!("number was three")
    }
}

// or i can do this

fn main() {
    let _bool = true;

    if _bool {
        println!("Bool = {_bool}");
    } else {
        println!("Bool is {_bool}");
    }
}

*/

/*

    if i want to do not equal to use !=

fn main() {
    let _number = 3;

    if _number != 0 {
        println!("number is not zero");
    }
}
*/

/*

// else if 
// Note : to many of these is not handy so look into 
// Match cases they are simalar to switch cases but a lil diffrent

fn main() {
    let _number = 6;

    if _number % 4 == 0 {
        println!("number is divisible by 4");
    } else if _number % 3 == 0 {
        println!("number is divisible by 3");
    } else if _number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
*/

/*

// since the if statment is an expression we can use it 
// to assign a value to a variable with a let   vvv

fn main() {
    let _condition = false;
    let _number = if  _condition {5} else {6};
    println!("number = {_number}");
}
*/

//
//  Remember that blocks of code evaluate to the last expression in them, 
//  and numbers by themselves are also expressions.
//




//
// Loops 
//

fn main() {

} 
