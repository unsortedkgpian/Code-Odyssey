use std::fs;
use std::io::Error;

fn temp_string_test(
    a:String,
    b:&String,
    c:&str
){

}

fn extract_errors(text: &str) -> Vec<String> {
    let split_text = text.split("\n");
    let mut results = vec![];

    for line in split_text {
        if line.starts_with("ERROR"){
            results.push(line.to_string());
        }
    }

    results
}


fn main() -> Result<(), Error>{
    //println!("Hello, world!");
    temp_string_test(
        String::from("red"),
        &String::from("red"),
        "red"
    );



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

    let color = String::from("red");
    let c = color.as_str();
    println!("{}",color);

    let color = String::from("blue");
    let portion = &color[1..4];
    println!("{}",portion);


    let mut error_logs = vec![];
    match fs::read_to_string("logs.txt"){
        Ok(value) => {
            error_logs = extract_errors(&value);
            //println!("{:#?}",error_logs);
            match fs::write("errors.txt", error_logs.join("\n")){
                Ok(_) => println!("Wrote errors.txt"),
                Err(error) => println!("Failed: {}", error)
            }
        }
        Err(err) => {
            println!("{}", err);
        }
    };
    println!("{:#?}",error_logs);


    let mini = fs::read_to_string("logs.txt").expect("failed to read logs.txt");
    let mini_error_logs = extract_errors(mini.as_str());
    fs::write("mini_error.txt",mini_error_logs.join("\n")).expect("failed to write mini_error.txt");



    // best part 
    let adtext = fs::read_to_string("logs.txt")?;
    println!("{}",adtext.len());

    let aderror_logs = extract_errors(adtext.as_str());
    fs::write("adderrors.txt",error_logs.join("\n"))?;

    Ok(())
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
