//
//  MENTAL RECIPE
//
/*

================================================================================

While loop --- Conditional

Keyword: while

The Check: A condition that must result in a bool (true or false). 
No parentheses () are needed around this check in Rust.

The Body: {} curly brackets. The code inside runs, then jumps back to The Check.

The Exit: As soon as the condition becomes false, the loop stops.

Mental Recipe: -> "Check the gate (while condition) -> If open, 
enter and run code { ... } -> Go back to gate and check again."

================================================================================

Loop --- Infinite

Keyword: loop

The Body: {} curly brackets. This code will repeat forever by default.

The Escape: break. You must put a break inside an if block somewhere,
or the program will never stop.

The Value (Optional): If you put a variable after break,
that value is "thrown" out of the loop to be saved.

Mental Recipe: -> "Start running forever (loop) -> 
Do work {...} -> If a certain event happens, jump out (break).

================================================================================

if --- expression

The Variable: let x : i32 = (We start by declaring where the result will live).

Keyword: if condition -> The first option.

The Result: { value_a } (The value we want if the condition is true).
No semicolon here!

Mental Recipe: > "Create a variable (let x =) -> Check a condition 
(if) -> Pick one result ({ value }) or the other (else { value })."

================================================================================

For Loop --- Iterator

Keyword: for

The Pattern: for item in collection { ... }

The Range: a..b (exclusive, stops before b) or a..=b (inclusive, includes b).

The Body: {} curly brackets. The code runs once for every item in the collection or range.

Mental Recipe: -> Take a container (range or list) -> Pick up the first item -> Run the code
using that item { ... } -> Pick up the next item and repeat until the container is empty.

================================================================================

*/
//
// excersises
//

// i just need to start using theese things.


//
//  Countdown
//
/*
fn main() {
    for _number in (1..11).rev() {
        println!("{_number}");
    }
    println!("Liftoff!");
}
*/


//
//  Loop
//
/*
fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1; // incerement + and assign = 
        if counter == 10 {
            break counter
        }
    };
    println!("result = {result}");
}
*/


//
//  Temp
//
/* 
fn main() {
    let _temp = 25;

    if _temp >= 30 {
        println!("Hot its {_temp}");
    } else if _temp <= 10 {
        println!("Cold its {_temp}");
    } else {
        println!("Nice its {_temp}");
    }
}
*/


//Convert temperatures between Fahrenheit and Celsius.
// celsius to fahrenheit

fn main() {
    let cel = 100.5;
    let far = (cel*1.8) + 32.0;
    println!("{far}");
}
