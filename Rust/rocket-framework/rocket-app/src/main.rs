#[macro_use] extern crate rocket;

use rocket::serde::json::{Value, json};
use rocket::response::status;

// #[get("/")]
// fn hello() -> Value{
//     json!("Hello, World!")
// }
// // fn hello() -> &'static str {
// //     "Hello World!\n"
// // }

#[catch(404)]
fn not_found() -> Value{
    json!("You are an idiots")
}

#[get("/rustaceans")]
fn get_rustaceans() -> Value{
    json!([{"id":1, "name":"John Doe"}, {"id":2, "name":"John Doe again"}])
}

#[get("/rustaceans/<id>")]
fn view_rustaceans(id:i32) -> Value{
    json!({"id":id, "name": "John Doe", "email":"john@doe.com"})
}

#[post("/rustaceans", format="json")]
fn create_rustacean() -> Value{
    json!({"id":3, "name":"John Doe", "email":"john@doe.com"})
}

#[put("/rustaceans/<id>", format="json")]
fn update_rustacean(id: i32) -> Value{
    json!({"id":id, "name":"John Doe", "email":"john@doe.com"})
}

#[delete("/rustaceans/<id>")]
fn delete_rustanean(id: i32) -> status::NoContent{
    status::NoContent 
}

#[rocket::main]
async fn main(){
    let _ = rocket::build()
        .mount("/", routes![
            get_rustaceans,
            view_rustaceans,
            create_rustacean,
            update_rustacean,
            delete_rustanean,
        ])
        .register("/", catchers![
            not_found
        ])
        .launch()
        .await;
}