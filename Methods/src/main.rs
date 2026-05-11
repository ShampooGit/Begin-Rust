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
/*
impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
/*
with an impl we can do just &self aka a refrence to the caller 
so we dont have to type this

fn area(width: u32, height: u32) -> u32{
    width * height
}

it saves alot of time imo and we are saying hey this functions belongs to the 
Rect struct wich is pretty nice aswell

*/

fn main() {
    let rect1= Rect {
        width: 40,
        height: 30,
    };
    println!("the area of rect1 is {}",rect1.area());
    /*
    also calling the fn is diffrent frist we did this
    let area_rect = area(rect1.width, rect1.height);
    and now its just 
    let area_rect = rect1.area()
    much much simpler 
    */
}

*/


// we can also give a method the same name as field form the struct

/*
impl Rect {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rect {
        width: 40,
        height: 30,
    };

    if rect1.width() {
        println!("the width is greater then 0 its, {}", rect1.width);
    }
}
*/
// here we refrence a value and check if its > to 0 then we say its 
// either true or false and based on that we do an action.
// wauw amazing stuff, what a time to be alive.

// rust knows we are tlaking about thw width method cuz we are using () at the end
// if we did not use () so rect1.width then rust would be like we are suing the field


// getters
// often when we just do this we would only want to return the value
// but not always methods like those are called getters
// getter are well nice cuz they are getters making it possible to make 
// the struct private so it will be read only and we can not edit the data
// chapter 7 will dive into getters and setters



//
//  Methods with more parameters
//

/*
impl Rect {
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other_rect: &Rect) -> bool {
        self.height > other_rect.height && // and operator 
        self.width > other_rect.width
    }
} 

fn main() {
    let rect1 = Rect {
        width: 30,
        height: 50,
    };

    let rect2 = Rect {
        width: 10,
        height: 40,
    };

    let rect3 = Rect {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2?  {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3?  {}", rect1.can_hold(&rect3));
}
*/

//
//  Associated Functions
//

// any functions inside of a impl is called a
// associated function
// we can define ass_funcs that dont have self as the first parameter
// and so they are not methods becasue they dont need an instance 
// ass_funcs are mostly used for constructers that return a new insatnce

// here we make a square that has the same value for height and width
impl Rect {
    fn square(size: u32) -> Self { // Self witts caps
        Self {  // Self witts caps
            width: size,
            height: size,
        }
        // Self represents Rect 
    }
}

fn main() {
    let my_square = Rect::square(3);
    // this is how we would call the ass_funcs, just like with
    let my_string = String::from("kaas");
    // this is sooooo flipping coooollll omg im about to busttt
    // the excitement i am having about this discovery is inmens
    // >///<
    // chapter 7 will go deeper in this
}


//
//  Mutiple impl blocks
//

/*
    we can have mutiple impl blocks wich can be handy in some cases
*/