//
//  enums 
//

/*
enum IpAddrKind {
    V4,
    V6,
}
// theese are a custom data type


fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    // here we create instances of the enum variants
    // okay so the vairants aare of the same kind IpAddrKind
    // we use :: to access the variants
    println!("Hello, world!");


    //  check fn route

    // and to call this func we can use the normail way of callinf funcs 
    // but we use the :: to say wich one we pass
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
    // pretty nifty ey
}


// now we can make a func that accepts any IpAddrKind <- this is where the custom data type coomes into play
fn route(ip_kind: IpAddrKind) { // here we say okay ip_kind is of type IpAddrKind
    // now thats dope as fuck love this idea 
}
*/

/*
fn main() {
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        adress: String,
    }

    // here we make home have the version and we assign a adress
    let home = IpAddr {
        kind: IpAddrKind::V4,
        adress: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        adress: String::from("::1"),
    };

    // and here we print them out
    println!("ip adress of home {}", home.adress);
    println!("ip adress of loopback {}", loopback.adress);
}

*/

// how ever we can write it shorter and more concise if we tell the version
// what type of data it will store so check this
/*
fn main() {
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
    /*
        here we attach data to the variant enum home = (V4)127.0.0.1
        so there is not need for a struct since we already say hey this variant 
        has this data type attached to it

        and it reutrns a instance of IpAddr
        also the varaintions become fucntions that take a String and return a
        instance of IpAddr

    */
}
*/

//
//  now comes something really cool 
//
/*

fn main() {
    enum IpAddr {
        // we can have mutiple in one enum variantion
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
}
*/

//
//  alot can be in a enum
//
/*
fn main() {

    struct Ipv4Addr {

    }
    struct Ipv6Addr {

    }
    enum IpAddr {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }
    /*
    this shows that enums can store a fuck ton of diffrent things 
    like a struct wtf is this, thats alot of power 
    gonna be honest dont even know how to to but damnn
    its gonna be fire when i do.
    */
}
*/

//
//  omg and there is moreee
//
/*
fn main() {
    #[derive(Debug)]
    enum Message {
        Quit,
        Move {x: i32, y: i32},  //fields so... key : value
        Write(String),
        ChnageColor(i32, i32, i32),
    }
    // so this enum is basicaly holding 4 struct and if we wrote it like struct 
    // it would look like this
    struct QuitMessage;     // unit struct aka empty struct
    struct MoveMessage {
        x: i32,
        y: i32,
    };
    struct WriteMessage(String);    // tuple struct
    struct ChangeColorMessage(i32, i32, i32);   // tuple struct
    // is that not crazy with the enum we dont only make all theese but we group
    // them together inside Message that is nutty


    // methods on enums
    impl Message {
        fn call(&self){
            println!("{:?}", self)
            // Write(Hello)
        }
    }

    let m = Message::Write(String::from("Hello"));
    m.call();
    println!("{:?}", m)
    // Write(Hello)

}
*/

//
//  The Option Enum
//

// the option enum is in the std
// and it is used for if the data is either something or nothing
// there for the name option

// the reason fo this is, the code will not compile if not all possible options
// have been acounted for so we dont get any runtime errors

// option can be used to represent a form of null so there value is present or absent
// https://doc.rust-lang.org/std/option/enum.Option.html

fn main() {
    enum OptionEx<T> {
        // T is the place holder var
        // can be anything 
        None,
        Some(T),
    }
    // cuz option is used so often it gets privalges
    //  - you dont have to use any prefix Option::
    //  - you dont have to bring option into scope
    //  - the varaints of option get brought into scope aswell

    // <T> deep dive
    // its a generic type parameter (no clue what that means lol)
    // 
}