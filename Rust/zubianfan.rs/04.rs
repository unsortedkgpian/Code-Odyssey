fn main(){
    //println!("{}, world", x);// will give error 
    define_x();
}

fn define_x() {
    let x = "hello";
    println!("{}, world", x);// will fix the error
}
