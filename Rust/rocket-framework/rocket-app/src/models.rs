use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono;

use crate::schema::rustaceans;

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name =  crate::schema::rustaceans)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]  
pub struct Rustacean{
    pub id: Option<i32>,
    pub name:String,
    pub email:String,
    pub created_at: chrono::NaiveDateTime,
}


#[derive(Deserialize, Insertable)]
#[derive(table_name = rustaceans)]
pub struct NewRustacean{
    pub name:String,
    pub email:String
}