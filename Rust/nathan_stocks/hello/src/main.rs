use hello::greet;
use std::collections::HashMap;

fn main() {
    let bunnies:i32 =3;
    println!("Hello, world!");

    let(bunnies, carrots) =(8,50);
    println!("Bunnies :{} Carrots:{} ",bunnies,carrots);

    let mut deco = 10;

    // Scope
    let x1 = 10;
    {
        let y1 = 32;
        println!("{}, {}",x1,y1);
    }

    println!("{},",x1);

    // shadow
    let x2 = 32;
    {
        let x2 = 23;
        println!("{}",x2);
    }

    println!("{}",x2);


    let mut x_mut = 10;
    let x_mut = 20;
    // x_mut = 21; Error
    

    let enigma:i32;

    if true{
        enigma = 42;
    } else{
        enigma =32;
    }

    println!("enigma is {}", enigma);


    hello::greet();
    greet();

    let num = 1;
    let msg = if num ==5{
        "five"
    }else if num == 4{
        "four"
    }else{
        "other"
    };

    println!("{}", msg);

    let mut s1 =  String::from("Hi how are you");
    let s2 = s1;
    // println!("{}",s1);
    
}



