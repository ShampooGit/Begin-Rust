//
//  Structs
//

/*
// a struct needs a name 
struct User {
// in a struct we have a key: value, <- one of those is called a field
// wich looks like this username: String here username is the key and String is the value
    username: String,
    email: String,
    sign_in_count: u32,
    active: bool,
}
// we use struct to stare related data togehter
// in this case a user so we store the data a user could have


// to use a struct we create a instance
// a instance is where we devine a struct
// so we make it user1 = User
// then we fill it up with data 
fn main() {
    let user1 = User {
// okay now that we have a instance of our struct we
// get to populate it with data (the order does not have to be the same as defined in the struct)
    
// so to assign a value to a key we simply write the key
// username: and then the value String::from("CheeseMaster"),
// we end up with the following   
    username: String::from("CheeseMaster"),
    email: String::from("Cheese@master.com"),
    sign_in_count: 1,
    active: !false, // useing ! infront makes it the iopsisite so !false = true
    };
// okay great but how do we acces the values now 
// we can call the instance so user1
// and then we do .key
// so to acces a specific value we call the instance and add the key with a dot as seprator

    let pp = user1.email;
    println!("{pp}");


//
//  mutable structs
//

// so thats great and all but lets say i need 
// to chnage the data well as we know we need 
// the mut keyword to edit stuff so we add mut
// to the struct wich woul look like this

// lets make another instance called user2 and make it mutable
    let mut user2 = User {
        username: String::from("PearBrother"),
        email: String::from("Pear@brother.com"),
        sign_in_count: 12,
        active: false,
    };


    println!("{}", user2.sign_in_count);
// now lets mutate(change) the user2 structs data
    user2.sign_in_count = 13;
    println!("{}", user2.sign_in_count);
// as you can see we can now change the value of any key 
// in the user2 instance
// NOTE:
// the entire instance must be mutable we can not 
// pick and choose fields we want to have mutable
}
*/



//
//  Build structs
//

struct User {
    username: String,
    email: String,
    sign_in_count: u32,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        // here we can do email: email, to get the value we give in main
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }

}

fn main() {
    let user1 = build_user( 
        String::from("carl16@carol.com"),
        String::from("carl"),
    );
    //crazy stuff going on here we can fill in data with a fn call
}



//
//  Field Init Shorthand
//
