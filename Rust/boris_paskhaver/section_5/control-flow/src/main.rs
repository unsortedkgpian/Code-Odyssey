fn main() {
    println!("Hello, world!");

    // Control flow 
    if true {
        println!("Its true");
    }
    if false {
        println!("Its false");
    }
    
    let some_condition_that_we_cannot_predict_in_advanced = true;

    if some_condition_that_we_cannot_predict_in_advanced {
        println!("This line will be output");
    }

    let season = "summer";
    if season == "summer"{
        println!("School is out!");
    }else if season =="winter"{
         println!("Brr, so cold!");
    }else if season =="fall"{
        println!("Leaves falling");
    }else if season =="spring"{
        println!("Lots of rain");
    }
    else{//else -> fallback 
        println!("you have done some wrong");
    }
    //
    even_or_odd(321);
    even_or_odd(2832);
    //println!("The result is {}", even_or_odd(342));
    












    println!("\n\n\n\n\n\n\n\n\n");

    let evaluation :bool = true;
    
    match evaluation {
        true => {
            println!("The value is true");
        }
        false => {
            println!("The value is false");
        }
    }

    let eval = true;
    let value = match eval {
        true => 20,
        false => 30,
    };
    println!("The eval is {eval}");
    println!("The value is {value}");


    // _ => catch all pattern 
    let day = "0monday";
    
    match day {
        "monday" => println!("Today is monday"),
        "tuesday" => println!("Today is holiday"),
        _ => println!("I don't know"),
    }

    let mut  num = 191;
    match num  {
        2 | 4 | 6 | 8 | 10 => println!("{num} is even "),
        1 | 3 | 5 => println!("{num} is odd "),
        _ => println!("Unkonw for now"),
    }
    match num {
        value if value % 2 == 0 => println!("{value} is even"), 
        _ => println!("{num} is odd "),
        // _ => println!("{value} is odd"),
        // _=> unreachable!()
        


    }
    // loop {
    //     println!("Running {}",num);
    //     num = num*2;
    // }

    let mut second:i32 = 10;
    
    loop {
        // println!("The second {second}");
        // second -= 1;
        if second < 0 {
            println!("Blastoff!!!!!!!!!");
            break;
        }
        if second %2 ==0{
            println!("{second} seconds (even number), 3 second skippked...");
            second -=3;
            continue;
        }
        println!("{second} to blastoff!!!!");
        second -=1;
    }


    let mut second = 59;
    while second >0  {
        if second %2 ==0 {
            println!("{second} seconds (even number), 3 second skippked...");
            second -=3;
            continue;
        }
        println!("{second} to blastoff!!!!");
        second -=1;
    }
    println!("Blastoff!!!!!!");

    countdown(5);
    
}

fn countdown (seconds:i32){
    if seconds <= 0 {
        println!("Blastoff!!!");
        return;
    }
    println!("{seconds} seconds to blastoff");
    countdown(seconds-1);
}


fn even_or_odd(number:i32) {
    // if number %2 == 0 {
    //     println!("{number} is even");
    // }else {
    //     println!("{number} is odd");
    // }
    let result = if number % 2 == 0 {"even"} else {"odd"};
    println!("The number is {result}")
}
