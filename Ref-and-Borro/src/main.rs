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

fn main() {
    
}