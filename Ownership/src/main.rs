//
//  info
//
/*
    a pointer is just a memory adress to the actual data wich is 8 bytes (on 64 bit machines)
    that means that the pointer gets stored on the stack but the actual data on the heap.

    - The Stack: Stores the Pointer (Fixed size: 8 bytes). This is the "Address."

    - The Heap: Stores the Actual Data (Variable size: 0 to GBs). This is the "Content."

    - The Result: Your function stays fast and "light" because the stack never has to move large chunks of data around—it just passes that 8-byte address.

    so what does ownership do,
    - It keeps track of what is using heap alocated data
    - Minimizes the amount of duplicate data
    - Cleans up unsused data on the heap



    Ownership Rules
    - Each value in Rust has an owner                  intellej gen TvT -> Each value in Rust has a variable that’s called its owner.
    - There can only be one owner at the time
    - When the owner goes out of scope, the value will be dropped

*/


//
//  varibale scope
//
/*
fn main() {
    {
        let s = "Hello";    // s is only in scope for this code block
    }
    println!("{s}");        // out of scope cuz we have s in brakckets wich is a code block
}
*/



//
//  The string Type
//
/* 
fn main() {
    //let s = String::from("String");
    let mut s = String::from("hello");
    s.push_str(", world!");     // this appends a literal to a String
    println!("{s}");        // i think that push is possible cuz a string is a array and this should be an vector
}
*/

//
//  Memory and Allocation
//
/*
    The memory must be requested from the memory allocater at runtime   --- we do this in ---   String::from
    We need a wat of returning this memory to allocator when we're done with String   --- but how --- vvv

    in rust there is no garbage collector (0.0)
    in rust the memory gets freed after the scope ends

*/

//  Variables and Data Interacting with Move
/*  int ver
fn main() {
    let x = 5;
    let y = x;
    // both of theese are = to 5 
    // both are ints wich are simple values with a known fixed size, 
    // so they are pushed on the stack
}
*/

//  String ver
/*
fn main() {
    let s1 = String::from ("Hello");
    let s2 = s1;
    // this does not simply copy the data like with the int
    // We instead copy the "name tag" info (pointer, len, cap)
    // this is beacuse a string is an array and its a pointer to the data thats being stored on the heap
    // so in the end we just coppy the memory adress instead of the data itself
    // if i really had to copy the data i could use .clone
}
// to prevent double memory Freeing Rust says yo s1 fuck you your now invaled (except if i use .clone)
// so this is called a move, 
*/
/*
fn main () {
    let s1 = String::from("Hello");
    let s2 = s1;
    println!("{s1} , world!");
    // if i try to run this
    // compiler says hey this is not possible add .clone behind s1  ->  let s2 = s1.clone;
    // this is cuz we moved from the string s1 to s2
}
*/


//
//  crazy discovery
//
//  there is something called .len
//  wich gives the length of an array that means i cna use it on strings right
//  also what about the other to things that are stored there like cap and pointer
//  i must be able to show those aswell somehow...
//



//
//  Scope & Assignment
//
/*
fn main() {
    let mut s = String::from("Hello"); // this got dropped
    s = String::from("Ahoy");
    println!("{s}, World");
}
*/

//
//  Var and data interact with clone
//
/*
fn main() {
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("s1 = {s1} & s2 = {s2}");
}
*/
//  usesing clone can be very expensive



//
//  Stack-Only Data : Copy
//
/*
fn main() {
    let x = 5;
    let y = x;
    println!("x = {x} & y = {y}"); 
}
// there is a magic word called copy wich can copy scaler data types
// for like way cheaper then clone cuz it only works on stack saved data types
*/


//
//  Ownership & Functions
//
/*
fn main() {
    let s = String::from("Hello");
    take_ownership(s);  // s has been moved to the function
    // println!("{s}"); <- this is not possible
    
    let x = 5;
    makes_copy(x); // x is copied to the function
    println!("{x}"); // this is possible cuz we copy scaler types
} // Here, x goes out of scope, then s. However, because s's value was moved,
  // nothing special happens.

fn take_ownership(Val_1: String){
    println!("{Val_1}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(Val_1: u32){
    println!("{Val_1}");
} // Here, some_integer goes out of scope. Nothing special happens.
*/


//
//  Return Values & Scope
//
/*
fn main() {
    let s1 = gives_ownership(); // returns a value

    let s2 = String::from("Hello"); // sets a value

    let s3 = takes_and_gives_back(s2); // moves a value & returns it
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String { // -> is used to indicate the returing type
    let val_1 = String::from("yours");
    val_1 
}

// 
fn takes_and_gives_back(val_1: String) -> String {
    val_1 // val_1 is returned and moves out to the calling function
}
*/


//
//  returning mutple values with a tuple
//

/*
its says that retunring a value like this (Pasing the ownership back and forth), 
is anoying. i agree, they mention using tuples as another thing we can do
to return mutple values. i wonder when we will get into refrences tho.
*/

fn main() {
    let s1 = String::from("Hello");

    let (s1, len) = calculate_length(s1);

    println!("the word '{s1}' has a length of {len}");
}

fn calculate_length(val_1: String) -> (String, usize) {// because we returning mutiple values we use ()
    let length = val_1.len(); // .len() returns the lentgh of a string

    (val_1, length)
}

// yesss now we go onto refrences finally...

