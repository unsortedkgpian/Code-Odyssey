mod basket;
mod stack;
mod container;

use basket::Basket;
use stack::Stack;
use container::Container;


fn add_string<T:Container<String>>(c:&mut T, s:String){
    c.put(s);
}

fn main() {

    //println!("Hello, world!");

    let b1 = Basket::new(String::from("hi there"));
    let b2 = Basket::new(10);
    let b3 = Basket::new(true);
    println!("{:#?}",b1);
    println!("{:#?}",b2);
    println!("{:#?}",b3);


    println!("\n\n\n");

    let mut s1 = Stack::new(vec![String::from("hi")]);
    let s2 = Stack::new(vec![1,2,3]);
    println!("{:#?}",s1);
    println!("{:#?}",s2);

    add_string(&mut s1, String::from("Aditya"));
    println!("{:#?}",s1);
}
