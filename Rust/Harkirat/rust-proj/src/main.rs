fn main() {
    println!("Hello, world!");

    let x = 5;
    // by default its i:32
    let x:i8 = 67;

    let y:u32 = 345;
    let z:f32 = 349.002;

    println!("x:{}, y:{}, z:{}", x ,y, z);

    let p:i8= 2;



    let mut t:i8 = 10;

    for i in 0..100{
        //t = t+100;
    }

    println!("t= {}", t);


    // ************ Boolean ***********************
    //
    let is_male = false;
    let is_above_18 = true;

    if is_male{
        println!("You are a male");
    } else {
        println!("you are not a male");
    }

    if is_male && is_above_18{
        println!("You are a legal male");
    }


    // **************** String ****************************
    
    let greeting = String::from("hello world");
    let great = "How are you";
    println!("{}", greeting);
    

    println!("Type of greeting is {}", std::any::type_name_of_val(&greeting));
    println!("Type of great is {}", std::any::type_name_of_val(&great));


    // How to find the rust is hard
    //println!("{}", greeting.chars().nth(1000)); // -> It will return option which can be zero or
    // not  
    let char1 = greeting.chars().nth(1000); // Options -> char1 
    

    match char1 {
        Some(c) => println!("{}", c),
        None => println!("No character at index 10000"),
    }


    // also can use unwrap()-> char1.unwrap()
    //
    //
    // ***********************************************************************************************************************************
    
    let sentence = String::from("Song my name is Aditya");

    let first_word = get_first_word(sentence);
    
    //for _i in 1..1000 {
      //  println!("Testing : {}th time", _i);
    //}
    

    println!("The first word in the sentence is {}", first_word);

    update_memory();

}

fn update_memory() {
    let mut s = String::from("Inital String");
    println!("Before update s string: {}", s);
    println!("Capacity:{} , Length:{}, pointer:{:p}", s.capacity(), s.len(), s.as_ptr());

    for _i in 0..1000 {
        s.push_str(" adding some exteranl string to increase the function with some vry teext jdkfjljskfjljdkfjkldjkfjlajjfijdfiajfifjijijiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii iihsdjf");
        println!("Capacity:{}, Lenght:{}, pointer:{:p}", s.capacity(), s.len(), s.as_ptr());
    }

}


fn get_first_word(sentence:String) -> String{
    //let mut ans = String::from("");
    let mut ans = String::new();

    for char in sentence.chars(){
        ans.push_str(char.to_string().as_str());
        if char == ' '{
            break;
        }
    }

    return ans;
}
