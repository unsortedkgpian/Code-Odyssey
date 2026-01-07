fn main(){
    let x :u64 = 4_294_967_296;
    let y = x as u32;
    
    if x == y as u64 {
        println!("x equal to y");
    }else{
        println!("x not equalts to y");
    }
}
