use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono;

use crate::schema::rustaceans;

#[derive(Serialize, Queryable, Selectable, Deserialize, AsChangeset)]
#[diesel(table_name =  crate::schema::rustaceans)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]  
pub struct Rustacean{
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name:String,
    pub email:String,
    #[serde(skip_deserializing)]
    pub created_at: chrono::NaiveDateTime,
}


#[derive(Deserialize, Insertable)]
#[diesel(table_name = rustaceans)]
pub struct NewRustacean {
    pub name: String,
    pub email:String
}