
// main
fn main() {
    // task 6
    let temp = 25;
    let humidity = 20; 

    plant_status(temp, humidity);

    // task 5
    let number = Some(42);
    response(number);

    // task 4
    reply(HttpReply::Success);
    reply(HttpReply::NotFound);
    reply(HttpReply::InternalError);

    // task 3
    let mut user1 = User {
        username: String::from("None"),
        login_count: 0,
    };
    println!("{}, {}", user1.login_count, user1.username);
    user1.login_count = 1;
    user1.username = String::from("Bob");
    println!("{}, {}", user1.login_count, user1.username);
    
    // task 2
    let ref_string = String::from("Kaas");
    print_var(&ref_string);
    println!("{}", ref_string);
    
    
    // task 1
    check_score(67);
}


/*
Exercise 1: Score Checker
The Goal: Write basic control flow and logic from scratch.

Task: Write a function check_score(score: u32) that prints 
"Pass" if the score is 50 or higher, and "Fail" if it is lower.
*/
fn check_score(score: u32) {
    if score >= 50 {
        println!("you passed");
    } else {
        println!("you failed");
    }
}

/*
Exercise 2: Ownership Pass-Along
The Goal: Feel how ownership moves and gets blocked by the compiler.

Task: Create a String variable in main. Pass it to a function that just prints it. 
Try to print that same variable again in main after the function call. 
Watch the compiler yell at you, then fix it by changing the function to use a reference instead.
*/
fn print_var(string_var: &String) {
    println!("{}", string_var);
}


/*
Exercise 3: The User Struct
The Goal: Write a data structure and read/modify its fields.

Task: Create a struct named User with fields for username and login_count. 
In main, create an instance of this struct, change the login count, and print the username.
*/

struct User {
    username: String,
    login_count: u32
}

/*
Exercise 4: Web Traffic Enum
Task: Create an enum HttpReply with variants: Success, NotFound, and InternalError. 
Use a match statement inside a function or main to print a custom text message for each one.
*/

enum HttpReply {
    Success,
    NotFound,
    InternalError,
}

fn reply(status: HttpReply) {
    match status {
        HttpReply::Success => println!("Omg it was a success"),
        HttpReply::NotFound => println!("http reply not found"),
        HttpReply::InternalError => println!("Opps there was an error"), // we could use the other thing we did witht he coins for specific error
    }
}


/*
Exercise 5: The Safety Net (Option)
Task: Create a variable holding Some(42). 
Use an if let statement to extract the number and print it. 
Then change that variable to None to make sure your code handles it without crashing.
*/
fn response(mut n: Option<i32>) {
    if let Some(val) = n {
        println!("{val}");
        n = None
    } else {
        println!("oops");
    }
}

/*
Exercise 6: Pure rust oractice
Task: Write a small CLI program that takes a "temperature" and "humidity" as hardcoded variables 
and uses an enum like FaceState { Happy, Sleepy, Thirsty }, 
then a match statement that picks a state based on the values.
*/

fn plant_status(t: i32, h: i32) {
    enum Facestate{
        Happy,
        Thirsty,
        Sleepy
    }

    let face = match (t, h) {
        (t, h) if t >= 25 && h <= 50 => Facestate::Thirsty,
        (t, h)if t <= 24 && h >= 51 => Facestate::Happy,
        _ => Facestate::Sleepy
    };

    match face {
        Facestate::Happy => println!("Yay all good <3"),
        Facestate::Thirsty => println!("plss give me some water"),
        Facestate::Sleepy => println!("oops ima sleep"),
    }
}