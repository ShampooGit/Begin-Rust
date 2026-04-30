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

/*
struct User {
    username: String,
    email: String,
    sign_in_count: u32,
    active: bool,
}
*/
/*
fn build_user(email: String, username: String) -> User {
    User {
        // here we can do email: email, to get the value we give in main
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }

}
*/
/*
fn main() {
    let user1 = build_user( 
        String::from("carl16@carol.com"),
        String::from("carl"),
    );
    //crazy stuff going on here we can fill in data with a fn call
}
*/


//
//  Field Init Shorthand
//

// we can use this  so we dont have to type field name twice
// this works cuz the siganture parameter names are the same 

// before
/*
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
*/
//after
/*
fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}
// pretty nice but the names do have to be the same
*/


//
//  Creating Instances with Struct update syntax
//

// we can make a new instance but with the same data from another instance
// this is called struct update syntax
/*
fn main() {
    let user1 = User {
        username: String::from("Kaasman"),
        email: String::from("Kaasman@og.com"),
        active: true,
        sign_in_count: 1,
    };

// we now give user3 the same values as user1 except email
/*
    let user3 = User {
        email: String::from("Kaasman@clone.com"),
        username: user1.username,
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
*/
// we can write this way shorter tho with the follwing syntax
    
    let user4 = User {
        email: String::from("Kaasman@droid.com"),
        ..user1 // must come last to symbolise the remaining fields
    };
// Note :
// doing this does move the data so we can not call the roginal 
// user1 field data anymore
// println!("{}", user1.username); // Not possible
// println!("{}", user1.email); // this is possible
// cuz we did not move the email data
}
*/


//
//  Tuple Structs
//

struct color(i32, i32, i32);
struct point(i32, i32, i32);

fn main() {
    let black = color(0, 0, 0);
    let origin = point(0, 0, 0);

    // to be abkle to get a value from the tuple struct we need to
    // destructre it    vvv
    let point(x, y, z) = origin;
    // now we can call x and we get the corespoding number
    println!("{}", x);

}

// 
//  Defining unit-Like Structs
//

