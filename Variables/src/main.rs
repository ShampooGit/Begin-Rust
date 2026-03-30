/* 
// example of immutable variable
fn main(){
    let x = 5;
    println!("x = {x}");

    x = 6;
    println!("x = {x}");
}
*/

/* 
// example of mut keyword
fn main() {
    let mut x = 5;
    println!("x = {x}");

    x = 6;
    println!("x = {x}");
}
*/

/* 
// example of shadowing
fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("x inner scope value = {x}");
    }
    println!("x std value = {x}");
}
*/

///* 
// example mutiple types can use same name using shadowing
fn main() {
    let spaces = "   ";
    println!("spaces (string) = {spaces}");
    let spaces = spaces.len();
    println!("spaces (number) = {spaces}");
}
//*/

/* 
// example of using mut while type changing 
fn main() {
    let mut spaces = "   ";
    println!("spaces (string) = {spaces}");
    // here we dont use a let \/ \/ wich gives error
    spaces = spaces.len();
    println!("spaces (number) = {spaces}");
    
}
*/
