use std::cmp::Ordering;

use std::io;

// Rust has some set of items to defined in the standard  library that it brinngs into the scope of
// every program. This set is called prelude

use rand::Rng;

fn main() {
    //println!("Hello, world!");

    println!("Guess the number!");
    
    let secret_number = rand::rng().random_range(1..=100);
    println!("The secret number is : {secret_number}");

    loop{
        println!("Please input the guess."); 

        let mut guess = String::new();

        let apples = 5;// immutable
        let mut bananas = 5;// mutable   

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); 

        println!("You guessed: {}", guess);
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        let x = 5;
        let y = 10;
    
        //println!("x = {x}  and y + 2 = {}", y);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
