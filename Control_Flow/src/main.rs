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
//  Loops 
//
//  there are 3 loop types
//  loop, while & for
//
/*
fn main() {
    loop {
        println!("again!");
        break;
    }
} 
*/


/*

//
//  we use a counter and an if to break the loop
//  then we pass the final counter number as result
//

fn main() {
    let mut counter = 0;

    let _result = loop {
        
        counter += 1;
        println!("{counter}");
        if counter == 10 {
            break counter *2
        }
    };
    println!("The result : {_result}");
}
*/

/*

//
//  Disambiguating with loop labels
//
//  i can add a label to a loop and then break a loop 
//  inside another loop using the label
// 

fn main() {
    let mut _count = 0;
    '_counting_up: loop {
        println!("count = {_count}");
        let mut _remaining = 10;

        loop {
            println!("remaining = {_remaining}");
            if _remaining == 9 {
                break;
            }
            if _count == 2 {
                break '_counting_up;
            } 
            _remaining -= 1;
        };
        _count +=1;
    };
    println!("End count = {_count}");
}
*/


//
//  while loops
//
//  i can not belive they made me write the while loop
//  using if's holy fuck wtf i feel nasty after those
//  so thank god we go onto those now
//
/*
fn main() {
    let mut _number = 3;

    while _number != 0 {
        println!("T-,{_number}");
        _number -= 1;
    }
    println!("Lift-off !");
}
*/

//
//  array looping with while
//
/*
fn main() {
    let _a = [10, 20, 30, 40, 50];
    let mut _index = 0;

    while _index <5 {
        println!("The value is : {}", _a[_index]);
        
        _index += 1;
    }
}
*/


//
//  for loop
//
//
//  this code snippet is saver then the one before since 
//  we directly take the length of the array
//  and this can be more preformant 
//
//  i can also just use a refrence to the array.
//
/*
fn main() {
    let _a = [10, 20, 30, 40, 50, 60];

    for _e in _a { //&_a
        println!("the value is :{_e}")
    }
}
*/

//
//  range in for loops
//
//  in a range we first use () <- inside we add the range
//  the range (1..4) will do tree loops cuz 4 is not included
//  so if the range was (1..7) then seven would not be included
//

fn main() {
    for _n in (1..4).rev() { // .rev is to reverse the range
        println!("{_n}");
    }
    println!("LIFTOFF!!!");
}
