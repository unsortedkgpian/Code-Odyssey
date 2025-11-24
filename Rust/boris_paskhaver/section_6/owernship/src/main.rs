// fn main() {
    // println!("Hello, world!");
    // // Ownership -> properties of comiler 
    // println!("\n\n\n Ownership ");
    // println!("Many Problem best of all");
    // println!("\n\n\n");
    //
    //
    // let age = 33;

    // {
    //     let is_handsome = true;//-> stack memory
    // }
    // // println!("{}", is_handsome); -> error -> out of scope
    //
    // // age variable exists here
    //
    //
    //



    // println!("\n\n\n\n");
    //
    // Copy trait 
    // Scalar type
    // let time = 2025;
    // let year = time;

    // println!("The time is {time}. It is the year {year}");
    //



    // let mut food = "Aditya";
    // println!("The food variable {food}");
    // food = "songshfhh";
    // println!("The food variable {food}");
    // println!(r"&str is driectly convert into binary executable not in stack or heap memory");
    //

    // String 
    // let new_string :String= String::new();
    //
    // let from_string :String = String::from("Kitkat");
    //


//     let name = String::from("Boris");
// }// age variable goes out of scope here 
//



fn main(){
    let age:i32 = 33;

    {
        let is_handsome :bool = true;
    }
    // is_handsome variablbe goes out of scope here and clean up
    // error 
    // println!("{is_handsome}"); -> exist on diffent scope in the same function
    println!("age varaible {age}");// still work 

    // Copy Trait 
     
    let time = 2025;
    let year = time;

    // Two seprate indepentend copy -> 
    println!("This time is {time}. It is the year {year}");

    // time is variable out of scope at end of program 

    


    // age variable exists here
}// age variable goes out of scope here 
