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

