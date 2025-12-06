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


use  std::any::type_name_of_val;

fn main(){
    let _age:i32 = 33;

    {
        let _is_handsome :bool = true;
    }
    // is_handsome variablbe goes out of scope here and clean up
    // error 
    // println!("{_is_handsome}"); -> exist on diffent scope in the same function
    println!("age varaible {_age}");// still work 

    // Copy Trait 
     
    let time = 2025;
    let year = time;

    // Two seprate indepentend copy -> 
    println!("This time is {time}. It is the year {year}");

    // time is variable out of scope at end of program 

    // The String Type 
    let food :&str = "pasta";
    // str -> string literals very special -> varience of string datatype
    // Not store on any data type 
    // Directly converted into the Binary file during compile time 
    
    // String datatype 
    let mut _name = String::new();
    let name = String::from("Aditya");
    //name = "Aditya";
    println!("The datatype of food is {}", type_name_of_val(&food));
    println!("The datatype of name is {}", type_name_of_val(&name));



    // The push_str method on the String Datatype 
    let mut nano = String::from("Tata Nano");
    println!("The car is {nano}");
    nano.push_str(" by TATA");
    println!("The car is {nano}");


    // Moves and Ownership -> transfer of ownership from one another 

    let person = String::from("Motorola");
    // this is valid 
    println!("My name is {person}");
    let phone = person.clone();
    drop(phone);
    //  println!("the phone variable {phone}");
    // ->value borrowed here after move
    

    // println!("{person}"); // -> Error value barrowed after move 
                          // ^ value borrowed here after move 

    let mut my_stack_value = 2;
    let my_integer_reference = &my_stack_value;

    println!("Test 1 my integer reference {my_integer_reference}");
    //my_stack_value = 3;

    println!("Test 1 original {my_stack_value}");
    println!("Test 2 my integer reference {my_integer_reference}");
    println!("\n type of my_integer_reference {}", type_name_of_val(&my_integer_reference));
    my_stack_value = 3;


    let my_heap_value = String::from("Mumbai"); 
    let my_string_referene = &my_heap_value;

    println!("Test new 1 my string reference is : {my_string_referene}");

    println!("Test new 2 my string value is : {my_heap_value}");

    // Browing means using somethings without taking owernship of it 
    //  & Operator 
    

    // Reference must never outlive their referent 



    // age variable exists here
}// age variable goes out of scope here 
 
// drop()












