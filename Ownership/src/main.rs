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
fn main() {
    let s1 = String::from ("Hello");
    let s2 = s1;
    // this does not simple copy the data like with the int
    // this is beacuse a string is an array and its a pointer to the data thats being stored on the heap
    // so in the end we just coppy the memory adress instead of the data itself
}
