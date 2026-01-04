use std::fs;
use std::io::Error;

fn temp_string_test(
    a:String,
    b:&String,
    c:&str
){

}

fn main() {
    //println!("Hello, world!");
    temp_string_test(
        String::from("red"),
        &String::from("red"),
        "red"
    )



    let texto = fs::read_to_string("logs.txt");

    println!("{:#?}", texto);

    match divide(4.0, 0.0){
        Ok(value) => {
            println!("{}", value);
        }
        Err(err) => {
            println!("Something went wrong {:#?}", err);
        }
    }

    match validate_email(String::from("adi@gmail.com")){
        Ok(..) => println!("email is valid"),
        Err(error) => {
            println!("Its wrong not a email");
        }
    }


    match fs::read_to_string("logs.txt"){
        Ok(text) =>{
            println!("{}", text.len());
        }
        Err(err) => {
            println!("Somthing went wrong Error{}",err );
        }
    }
}


fn divide(a: f64, b:f64) -> Result<f64, Error> {
    if b ==0.0{
        Err(Error::other("can't divide by 0"))
    }else {
        Ok(a/b)
    }
}


fn validate_email(email: String ) -> Result<(), Error> {
    if email.contains("@") {
        Ok(())
    }else{
        Err(Error::other("wrong email"))
    }
}
