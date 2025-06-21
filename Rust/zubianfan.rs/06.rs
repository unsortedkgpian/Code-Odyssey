
fn main() {
    let mut x :i32 = 1;
    x =7;
    // Shadowing and re-binding
    
    let mut x  = x;
    x = x+3; // -> this line give error as its it unmutable


    let y = 4;
    // Shadowing
    let y="I can also be bound to text!";

    println!("Success!")
}
