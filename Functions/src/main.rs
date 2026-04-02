
// Functions    vvv

/* 

calling another function in main

fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function(){
    println!("Another function.");
}
*/






// Parameters   vvv

/*

calling another function in main
with a argument / parameter

fn main() {
    println!("Hello, world!");

    another_function(5);
}
fn another_function(x : i32) {
    println!("The value of x is : {x}");
}
*/

/*

i must decalre the type of the parameter in the func signature
fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value : i16, unit_label : char) {
    println!("The measurement is : {value}{unit_label}");
}
*/







// statement and expressions    vvv
// Statements are instructions that perform some action and do not return a value.
// Expressions evaluate to a resultant value.

/*
fn main() {
    let _y = 6;     // is a statement
}
*/
/*
fn main() {
    let _y = {
        let _x = 3;      // is an expression
        _x + 1                // is an expression
    };                        
    // note
    // if i add a ; to the last line in the block
    // it becomes a stament and i can not return 
    // it to _y
    println!("The vale of y is : {_y}");
}
*/







// functions with return values    vvv

/*
    you use a -> to specify the return type
    in the function signature

    its the same as mutablity i need to tell
    rust yo i want this to return a value
    the last line that is od the specified type
    with no ; will be returnd

fn five() -> i16 {
    5
}

fn main() {
    let _x = five();
    println!("the value of x is {_x}");
}
*/

fn main() {
    let x = plus_one(5);

    println!("the value of x is : {x}");
}

fn plus_one(x : i16) -> i16 {
    x + 1
}
