fn main(){
    let x:i32 = 10;
    // Solution of this problem is 
    // Declaring the y in the main scope
    
    {
        let y:i32 = 5;
        println!("The value of x is {} and the value of y is {}",x,y);
    }

    println!("The value of x is {} and the value of y is {}", x, y);
}
