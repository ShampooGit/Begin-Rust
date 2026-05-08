//
//  Methods
//


// a method is similiar to a function,
// we declare it with fn and a name, they can have parameters, a return value and can contain code 
// unlike functions, methods defind in the context of a struct/enum/trait_object
// and the first parameter is always self, wich is the intsnace of that structs that is callinf it 

#[derive(Debug)]
struct Rect {
    width: u32,
    height:u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1= Rect {
        width: 40,
        height: 30,
    };
    println!("the area of rect1 is {}",rect1.area());
}
