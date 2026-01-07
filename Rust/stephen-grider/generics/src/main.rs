use num_traits::{ToPrimitive, Float};


fn solve<T:Float, U:Float>(a:T, b:U) ->f64 {

    let a_f64 =a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();
    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}


fn Solve<T:ToPrimitive, U:ToPrimitive>(a:T,b:U) -> f64{
    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();
    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}



fn main() {
    //println!("Hello, world!");
    

    let a:i32 = 3;
    let b:f64 = 4.0;

    let a_64 = a as f64;
    let a_64 = a.to_f64();



    println!("Sove :{}",Solve::<f32,f64>(a as f32,b));
    println!("Solve: {}", Solve(a,b));
}
