fn main() {
    println!("first part");
    //let x:i32;// program will pacing Uninitialized but used err 
    let x:i32 = 23;
    let _y:i32; // -> warning to avoid it _ in front

    assert_eq!(x,10 );//-> this program check both are equal if not program will panic at
    //runtime(complile succuesfully)
    println!("Sucess");
}
