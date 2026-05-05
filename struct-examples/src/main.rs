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
/*
now the code has become more complex and precies but
in return we get explicitnes wich is really nice
we can see exactly where / what happens
*/