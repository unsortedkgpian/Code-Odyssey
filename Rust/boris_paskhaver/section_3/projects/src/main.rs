fn main() {
    println!("Hello, world!");

    let inte : i32 = 13_37;
    let demo = inte as i16;

    let floating = 32.28798291;
    println!("{floating:.3}");

    let with_milk:bool = false;
    let with_sugar:bool = true;

    let is_my_type_of_coffee:bool;
    if with_sugar && with_milk {is_my_type_of_coffee = true;}
    else{
        is_my_type_of_coffee = false;
    }

    let is_acceptable_coffee:bool;

    if with_milk || with_sugar {is_acceptable_coffee = true;}
    else {is_acceptable_coffee = false;}

    let array :[i8; 4] =[3,24,1,53];
    dbg!(array);
    println!("Array is {array:?}");

    let tuple :(i32, f32, bool, [i8;4]);
    
    tuple = (32, 23.21, true, array);

    dbg!(tuple);
    
    for value in tuple{
        println!("The value is {value}");
    }
}
