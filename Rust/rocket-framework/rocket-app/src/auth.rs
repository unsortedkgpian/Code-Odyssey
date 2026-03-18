use std::collections::HashMap;

fn user_store() -> HashMap<&'static str, &'static str>{
    let mut users = HashMap::new();

    users.insert("Aladdin", "open sesame");
    users.insert("aditya", "password");
    users.insert("user", "test");
    users.insert("admin", "1234");
    users.insert("Jerk","31231");

    users
}

fn verify_user(username:&str, password:&str) -> bool{
    let users = user_store();
    match users.get(username){
        Some(stored_password) => stored_password==&password,
        None => false,
    }
}


use rocket::http::Status;
// use rocket::serde::json::{Value, json};
// use rocket::response::status;
use rocket::request::{FromRequest, Request, Outcome};
// use rocket::http::Status;
// use base64;
// use base64::Engine::decode;
#[warn(deprecated)]
use base64;
use serde::Serialize;

#[derive(Serialize)]
pub struct BasicAuth{
    pub username:String,
    pub password:String,

}


impl BasicAuth{
    fn from_authorization_header(header: &str) -> Option<BasicAuth>{
        let split = header.split_whitespace().collect::<Vec<_>>();
        if split.len() !=2{
            return None;
        }

        if split[0]!="Basic"{
            return None;
        }

        Self::from_base64_encoded(split[1])
    }

    fn from_base64_encoded(base64_string:&str) -> Option<BasicAuth>{
        let decoded = base64::decode(base64_string).ok()?;
        // ? will stop the function and stop the fuction
        let decoded_str = String::from_utf8(decoded).ok()?;
        let split = decoded_str.split(':').collect::<Vec<_>>();

        //if exactly username & password pair are present
        if split.len() != 2 {
            return None;
        }
        let (username, password) = (split[0].to_string(), split[1].to_string());

        Some(BasicAuth { username, password })
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BasicAuth{
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error>{
        let auth_header = request.headers().get_one("Authorization");
        if let Some(auth_header) = auth_header{
            if let Some(auth) = Self::from_authorization_header(auth_header){
                // Check against the selected auth pair
                // if auth.username == "Aladdin" && auth.password == "open sesame" {
                //     return Outcome::Success(auth);
                // } else {
                //     return Outcome::Error((Status::Unauthorized, ()));
                // }
                let valid = verify_user(&auth.username, &auth.password);
                if valid {
                    return Outcome::Success(auth);
                }else {
                    return Outcome::Error((Status::Unauthorized,()));
                }
            }
        }

        // Outcome::Failure((Status::Unauthorized, ()))
        Outcome::Error((Status::Unauthorized,()))
    }
}