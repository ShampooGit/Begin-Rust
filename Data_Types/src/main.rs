/*
    In most cases, you don’t have to tell 
    Rust which type you are using. 
    If you provide a value immediately, 
    Rust automatically assigns a default type 
    (like i32 for integers or f64 for decimals).

    However, 
    if you declare a variable without assigning a value, 
    you must explicitly tell Rust the type. 
*/

fn main() {
    println!("scalar types");
    println!("");
    println!("for infor about ints check out int_types()");
    int_types();
    println!("for info about floats check out f_types()");
    f_types();
    println!("for info about numeric Operations check out num_operations()");
    num_operations();
    println!("for info about booleans check out bool_types()");
    bool_types();
    println!("for info about chars check out char_types()");
    char_types();
    println!("");
    println!("============================================================");
    println!("");
    println!("compund types");
    println!("");
    println!("for info about tuples check out tup_types()");
    tup_type();
    println!("for info about arrays check out array_type()");
    array_type();

}

fn int_types() {
    /*
        lentgh in bits              signed      unsigned
        ------------------------------------------------
        8-bit                       i8          u8
        16-bit                      i16         u16
        32-bit                      i32         u32
        64-bit                      i64         u64
        128-bit                     i128        u128
        Architecture-dependent      isize       usize
        ------------------------------------------------
        signed means it has - and + numbers
        Rust integer default = signed 32-bit
    */
    let _x: u32 = 10;   // simple example
}

fn f_types() {
    /*
        floats in rust are alwyas signed 
        they can be either 32 or 64 bit
        also there are ways to make the 
        unsigend but requires a work around
        floats use the IEEE-754 standard
    */
    let _x:f64 = 2.0; // f64
    let _y:f32 = 3.0; // f32
    /*
        the default is f64 since the speed 
        is rougly the samne as f32 
        but f64 is more precies than f32
    */
}

fn num_operations() {
    /*
        here are the mathematical opreations
        addition            +
        subtraction         -
        multiplication      *
        division            /
        remainder           %

        remainder is a bit special as it has
        mutple oparation in one symbol

    */

    // addition
    let _sum = 5 + 10;
    // + adds 2 number together

    // subtraction
    let _difference = 95.5 - 4.3;
    // - subtracts the second number from the first

    // multiplication
    let _product = 4 * 30;
    // * multiplies the numbers together

    // division
    let _quotient = 56.7 / 32.2;
    // / divides the first number by the second

    // remainder
    let _remainder = 43 % 5;
    // % gives the remainder left over after dividing the first number by the second so
    // 43 / 5 = 8
    // 8 * 5 = 40
    // 43 - 40 = 3
    // so we end up with 3 as the remainder
}

fn bool_types() {
    /*
        a boolean can either be true or fasle
        the computer stores this in a byte (8 bits)
    */
    let _t = true;
    let _f = false;
}

fn char_types() {
    /*
        a char uses '' instead of "" like with strings
        char uses 4 bytes (32 bits) it uses Unicode
    */
    let _c = 'c';
    let _emonji_sparkles = '✨';
}

fn tup_type() {
    /*
        a tuple can store mutple data types in a list,
        tuples cannot not grow or shrink in size
    */
    let _tup_p = (500, 6.4, 'h', true);
    let _tup_p: (i32, f64, char, bool) = (500, 6.4, 'h', true);
    /*
        there are mutiple ways of retrieving the data 
        pattern matching & indexing
    */
    // pattern matching
    let (a, b, c, d) = _tup_p;
    println!("the value of a is {a}");
    println!("the value of b is {b}");
    println!("the value of c is {c}");
    println!("the value of d is {d}");
    // here we create seprate vars for each value

    let _tup_i: (bool, char, f64, i32) = (false, 'g', 4.6, 5); 
    // indexing
    let _e = _tup_i.0;
    let _f = _tup_i.1;
    let _g = _tup_i.2;
    let _j = _tup_i.3;
    // here we create seprate vars for each value
    // and we use 0..4 to grab the value from that var

    // unit tuple "special type"
    /*
        An empty tuple () is called a 'unit'. 
        It represents an empty value or an empty return type.
        If a piece of code doesn't specifically result in a value, 
        Rust automatically returns () instead.
    */
    let _tup_u: () = ();
}

fn array_type() {
    /*
        arrays can only hold one type of data and like the tuple
        they have a fixed length, data stored in an array is
        on the stack so when you know the amount of elements
        will not need to change you should use an array think of
        the months there are 12 and will not change.
    */
    let _a = [1, 2, 3, 4, 5];
    let _a: [i16; 5] = [1, 2, 3, 4, 5];

    // you can also make an array and add a value intead of a type 
    let _ac = [3; 5];
    // now the array is the same as let _a = [3, 3, 3, 3, 3];

    // acesing the value of an array
    // can be done via indexing like with tuples
    let _first = _a[0];
    let _second = _a[1];
}

