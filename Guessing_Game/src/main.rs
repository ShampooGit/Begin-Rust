use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let r_number = rand::thread_rng().gen_range(1,101);
    // println!("random number = {r_number}");
    println!("Guess the number game");
    println!("between 1 & 100");
    
    loop {
        println!("please enter a number...");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("could not read line");
        let guess:i8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("you guessed : {guess}");

        match guess.cmp(&r_number) {
            Ordering::Less => println!("thats below the number."),
            Ordering::Equal => {
                println!("Omg you Won!!");
                break;
            },
            Ordering::Greater => println!("thats above the number."),
        }
    }
}
