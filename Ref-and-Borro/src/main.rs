//
//  my beloved, Refrences
//

/*
    so instead if pasing a value we pass the address
    of the value, this is more effinecient and
    the intial value is stays in its scope.

    & = andpersent
    and we use it to say het this is a refrence
*/
/*
fn main() {
    let s1 = String::from("Hello");
    let len = calc_length(&s1);

    println!("{s1} has a length of {len}");
}

fn calc_length(string :&String) -> usize {
    string.len()
}
*/
//  Here we pass the string in a refrence to count
//  the amount of letters with .len()
//  and we only have to return a int
//  and not the string aswell.



//  NOTE :
/*
    The opposite of referencing by using & is dereferencing, 
    which is accomplished with the dereference operator, *.

    Dereference will be handeld in Ch 8
*/

/*
//example
fn main() {
    let s1 = String::from("Hello");
    let len = calc_length(&s1);
    //  &s1 let us use the s1 string without moving it
    //  with this we can look at the value and evaluate it
    //  make decisions based on it and other things
    //  we cannot edit this value tho more on that later
    //  so a refrence is just to look at a value
}

fn calc_length(s :&String) -> usize { // s is the refrence to the string
    s.len()
}   // here, s goes out of scope, but since s does not own the value
//  the value stays valid
*/

//  so what if we try to edit the value that we borrowed
/*
fn main() {
    let s = String::from("Hello");
    change(&s); 
}

fn change(s: &String) {
    s.push_str(", world!");
}
*/
//  we get an error saying we need a &mut



//
//  Mutable Refrences
//

// to fix the last code snippet we can do &mut
/*
fn main() {
    let mut s = String::from("Hello");
    change(&mut s);
}

fn change(a_string :&mut String) {
    a_string.push_str(", world");
}
*/
// so s becomes a &mut
// in the signature change(a_string: &mut String)
// so a_string: wordt the var in the fn
// en &mut String laat zien van hey dit is een 
// refrence mutable en het is een String

// the one big restriction
// there can only be one &mut to that s var

// so this would not work
/*
fn main() {
    let mut s = String::from("Hello");
    let r1 = &mut s;
    let r2 = &mut s;

    println!("{r1},{r2}");
}
*/

// the reason for this 
// it prevents data races and makes the code 
// more reliable
/*
fn main() {
    let mut s = String::from("Hello");
    {
        let r1 = &mut s;
    }
    let r2 = &mut s;
}
// this works cuz r1 is in a diffrent scope
*/


// another thing
// there is a simaler rule like the one above
// for combining muable and imutable refrences
/*
fn main() {
    let mut s = String::from("Hello");
    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s;
    println!("{r1}, {r2}, and {r3}");
}
// so r3 is causing a problem 
*/
/*
fn main() {
    let mut s = String::from("Hello");

    let r1 = &s;
    let r2 = &s;
    println!("{r1} and {r2}");
// since this r1 & r2 do not get used after this print
// we can do a &mut to s

    let r3 = &mut s;
    r3.push_str(", World");  
    println!("{r3}");
// the same happens here if we dont use r3 after this print
// we are all good ofc i can reintilise them

    let r1 = &s;
    let r2 = &s;
    println!("{r1} and {r2}");
// but now the value will be ofc changed
}
*/



//
//  Dangling Refrences
//

fn main() {
    
}
