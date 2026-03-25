#[macro_use] extern crate rocket;

mod auth;
mod schema;
mod models;
mod repositories;


use rocket::Rocket;
use rocket::fairing::AdHoc;
use rocket::http::Status;
// use diesel::sql_types::Json;
use rocket::serde::json::Json;
// use schema::rustaceans;


use auth::BasicAuth;
// use rocket::http::Status;
use rocket::serde::json::{Value, json};
use rocket::response::status::{self, Custom};
use rocket_sync_db_pools::database;


// use diesel::prelude::*;
use models::{Rustacean , NewRustacean};
use  repositories::RustaceanRepository;

// use crate::schema::rustaceans;

// use crate::models::NewRustacean;

// use crate::schema::rustaceans;

// use rocket::request::{FromRequest, Request, Outcome};
// use rocket::http::Status;
// use base64;
// use base64::Engine::decode;
// use base64;
// use serde::Serialize;

// #[derive(Serialize)]
// pub struct BasicAuth{
//     pub username:String,
//     pub password:String,

// }


// impl BasicAuth{
//     fn from_authorization_header(header: &str) -> Option<BasicAuth>{
//         let split = header.split_whitespace().collect::<Vec<_>>();
//         if split.len() !=2{
//             return None;
//         }

//         if split[0]!="Basic"{
//             return None;
//         }

//         Self::from_base64_encoded(split[1])
//     }

//     fn from_base64_encoded(base64_string:&str) -> Option<BasicAuth>{
//         let decoded = base64::decode(base64_string).ok()?;
//         // ? will stop the function and stop the fuction
//         let decoded_str = String::from_utf8(decoded).ok()?;
//         let split = decoded_str.split(':').collect::<Vec<_>>();

//         //if exactly username & password pair are present
//         if split.len() != 2 {
//             return None;
//         }
//         let (username, password) = (split[0].to_string(), split[1].to_string());

//         Some(BasicAuth { username, password })
//     }
// }

// #[rocket::async_trait]
// impl<'r> FromRequest<'r> for BasicAuth{
//     type Error = ();

//     async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error>{
//         let auth_header = request.headers().get_one("Authorization");
//         if let Some(auth_header) = auth_header{
//             if let Some(auth) = Self::from_authorization_header(auth_header){
//                 return Outcome::Success(auth)
//             }
//         }

//         // Outcome::Failure((Status::Unauthorized, ()))
//         Outcome::Error((Status::Unauthorized,()))
//     }
// }

// #[get("/")]
// fn hello() -> Value{
//     json!("Hello, World!")
// }
// // fn hello() -> &'static str {
// //     "Hello World!\n"
// // }

// #[derive(Serialize)]

#[database("sqlite")]
struct DbConn(diesel::SqliteConnection);


// println!("DATABASE_URL = {:?}", std::env::var("DATABASE_URL"));

#[catch(404)]
fn not_found() -> Value{
    json!("You are an idiots")
}

#[catch(401)]
fn unauthorized() -> Value{
    json!("You are imposter \n")
}

#[catch(422)]
fn unprocessabe() -> Value{
    json!("You have entered something wrong correct it")
}

#[get("/rustaceans")]
// async fn get_rustaceans(_auth: BasicAuth, db: DbConn) -> Value{
async fn get_rustaceans(_auth: BasicAuth, db: DbConn) -> Result<Value, Custom<Value>>{
    // json!([{"id":1, "name":"John Doe"}, {"id":2, "name":"John Doe again"}, {"Hi":_auth},  {"db":"comming soon"}])
    // db.run(|c|{
    //     let rustaceans = rustaceans::table
    //         .order(rustaceans::id.desc())
    //         .limit(1000)
    //         .select(Rustacean::as_select())
    //         .load::<Rustacean>(c)
    //         .expect("DB error");
    //     json!(rustaceans)
    // }).await
    db.run(|c|{
        RustaceanRepository::find_multiple(c, 1000)
            .map(|rustaceans| json!(rustaceans))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}

#[get("/rustaceans/<id>")]
async fn view_rustaceans(id:i32, _auth: BasicAuth, db:DbConn) -> Result<Value, Custom<Value>>{
    // json!({"id":id, "name": "John Doe", "email":"john@doe.com"})
    // db.run(move |c|{
    //     let  rustaceans = rustaceans::table
    //         .find(id)
    //         .get_result::<Rustacean>(c)
    //         .expect("DB error when selecting rustanceans");
    //     json!(rustaceans)
    // }).await

    db.run(move |c|{
        RustaceanRepository::find(c, id)
            .map(|rustaceans| json!(rustaceans))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}

#[post("/rustaceans", format="json", data = "<new_rustacean>")]
async fn create_rustacean(_auth: BasicAuth, db: DbConn, new_rustacean:Json<NewRustacean>) -> Result<Value, Custom<Value>>{
    // json!({"id":3, "name":"John Doe", "email":"john@doe.com"})

    // let new_rustacean = new_rustacean.into_inner();

    // db.run(|c|{
    //     let result = diesel::insert_into(rustaceans::table)
    //         .values(&new_rustacean)
    //         .execute(c)
    //         .expect("DB error  when  inserting");
    //     json!(result)
    // }).await
    // db.run(|c|{
    //     let result = diesel::insert_into(rustaceans::table)
    //         .values(new_rustacean.into_inner())
    //         .execute(c)
    //         .expect("DB error when inserting");
    //     json!(result)
    // }).await
    db.run(|c|{
        RustaceanRepository::create(c, new_rustacean.into_inner())
            .map(|rustaceans| json!(rustaceans))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}

#[put("/rustaceans/<id>", format="json", data="<rustacean>")]
async fn update_rustacean(id: i32, db: DbConn, _auth: BasicAuth, rustacean: Json<Rustacean>) -> Result<Value, Custom<Value>>{
    // json!({"id":id, "name":"John Doe", "email":"john@doe.com"})
    db.run(move |c|{
        RustaceanRepository::save(c,id, rustacean.into_inner())
            .map(|rustaceans| json!(rustaceans))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}

// #[delete("/rustaceans/<id>")]
// async fn delete_rustanean(id: i32, db: DbConn, _auth: BasicAuth) -> Result<status::NoContent, Custom<Value>>{
//     // status::NoContent 
//     db.run(move |c|{
//         // diesel::delete(rustaceans::table.find(id))
//         //     .execute(c)

//         match RustaceanRepository::find(c, id){
//             Ok(_) => {
//                 RustaceanRepository::delete(c, id)
//                     .map(|_| status::NoContent)
//                     .map_err(|e|{
//                         Custom(Status::InternalServerError, json!(e.to_string()))
//                     })
//             }

//             Err(diesel::result::Error::NotFound) =>{
//                 Err(Custom(
//                     Status::NotFound,
//                     json!({"error":"Rustacean not found"})
//                 ))
//             }

//             Err(e) => {
//                 Err(Custom())
//             }
//         }

//         RustaceanRepository::delete(c, id)
//             .map(|_| status::NoContent)
//             .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
//     }).await
// }

#[delete("/rustaceans/<id>")]
async fn delete_rustanean(
    id: i32,
    db: DbConn,
    _auth: BasicAuth
) -> Result<status::NoContent, Custom<Value>> {

    db.run(move |c| {
        match RustaceanRepository::find(c, id) {
            Ok(_) => {
                // ✅ exists → delete
                RustaceanRepository::delete(c, id)
                    .map(|_| status::NoContent)
                    .map_err(|e| {
                        Custom(Status::InternalServerError, json!(e.to_string()))
                    })
            }

            Err(diesel::result::Error::NotFound) => {
                // ✅ not found → 404
                Err(Custom(
                    Status::NotFound,
                    json!({"error": "Rustacean not found"})
                ))
            }

            Err(e) => {
                // ✅ other errors
                Err(Custom(
                    Status::InternalServerError,
                    json!(e.to_string())
                ))
            }
        }
    }).await
}

async fn run_db_migration(rocket: Rocket<Build> )-> Rocket<Build> {
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

    DbConn::get_one(&rocket)
        .await
        .expect("Unable to reterive connection").run(|c|{
            c.run_pending_migrations(MIGRATIONS).expect("Migrations failed")
        })
        .await;

    rocket
}

#[rocket::main]
async fn main(){

    // println!("DATABASE_URL = {:?}", std::env::var("DATABASE_URL"));
    let _ = rocket::build()
        .mount("/", routes![
            get_rustaceans,
            view_rustaceans,
            create_rustacean,
            update_rustacean,
            delete_rustanean,
        ])
        .register("/", catchers![
            not_found,
            unauthorized,
            unprocessabe
        ])
        .attach(DbConn::fairing())
        .attach(AdHoc::on_ignite("Diesel Migration", run_db_migration))
        .launch()
        .await;
}