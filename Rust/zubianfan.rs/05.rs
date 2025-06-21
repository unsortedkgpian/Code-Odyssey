// shadowing 
fn main() {
    let x:i32 =5;
    {
        println!("Enter the mini-scope");
        let x =12;
        //assert_eq!(x,5);// its showing the value of 12 shadowing
        assert_eq!(x,12 );
    }
    println!("The scope is end ");

    //assert_eq!(x, 12);// its showing the value of x is 5 scope kill the x=12;
    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x);// 

    
}
