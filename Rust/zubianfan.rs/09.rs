fn main() {
    // let x ; let y;
    let (x, y);

    // Destructure assignments;
    (x,..) =(3,4);
    [.., y] = [1,2];

    assert_eq!([x,y], [3,2] );

    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
     
}
