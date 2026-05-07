//
//  example
//
/*
this is to show when we might want to use a struct
*/
/*
fn main() {
    let widthM: u32 = 10;
    let heightM: u32 = 20;

    println!("the area is: {}", area(widthM, heightM));
}

fn area (width: u32, height: u32) -> u32 {
    width * height
}
*/
/*
the issue is that we would not know that width and hieght
belong togehter in this tyoe of case we would use a struct
so it clear for other programmers that those 2 values
belong togheter
*/


//
//  refactor with tuples
//
/*
fn main () {
    let rect1 = (30, 50);

    println!("the area of rect1 is : {}", area(rect1));
}

fn area (dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1 
}
*/
/*
wauw okay alot of optimization here
so first we create a tuple so now we have rect1
with the 2 values

then in fn area we call the indexses
of dimesions

the problem is we dont know wich one of the 2 values is
the hieght rn since tupels dont name thier values

this is where structs come in
*/


//
//  refactoring with structs
//
/*
struct Rectangle {
    width: u32,
    height: u32,
}

fn main () {
    let rect1 = Rectangle {
        width: 5,
        height: 15,
    };
    println!("the area of the rectangle is: {}", area(&rect1));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
*/
/*
now the code has become more complex and precies but
in return we get explicitnes wich is really nice
we can see exactly where / what happens
*/


//
//  Adding functionality with derived traits
//

#[derive(Debug)]
struct rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = rectangle {
        width: 7,
        height: 10,
    };
    // this does not work because of how println! works
    // println!("rect1 is {rect1}");
    /*
        how println! works in context of structs

    println has many kinds of possible formating options 
    the default : the curly brackets {} tell println! use Display
    this is what we use for primitive types like String, int, bool etc
    
    the formating is unclear when we use structs and since rust
    does not want to guess we have to use a diffrent type of fromatting
    for thr data we want to show
    */

    // here we use :? to print the data in debug format
    // but rn this wont work
    println!("rect1 is {rect1:?}");   
    // to make this work we need to tag the struct with #[derive(Debug)]
    // the result will is: rect1 is rectangle { width: 7, height: 10 }

    // now thats cool and all but there is another way of printing the
    // contents fo the struct but make it look a bit nicer
    println!("rect1 is {rect1:#?}");
    /*
    now this prints as follows:
    
    rect1 is rectangle {
        width: 7,
        height: 10,
    }
    
    okay to look back a bit we can use the : 
    to be like okay now we are gonna talk about the format
    inside of the curly brackets so {rect1:?} <- here
    rect1 is ofc the value we want to print and
    ? is the way of printing and by adding
    # we chnage the fromat type 
    
    */

    // now comes the actual crazy part
    // there is another wayyyy to print using the debug fromat

    // we do this with a macro called dbg! (derive debug)
    // and ! stands for calling a macro

    // the dbg! takes owenership of the expression,
    // println!() does not take ownership but a refrence
    // dbg also prints the file name and line number wher the code is written
    // this is how you write it
    dbg!(&rect1);

}