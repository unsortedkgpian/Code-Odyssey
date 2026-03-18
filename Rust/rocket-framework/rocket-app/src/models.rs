use diesel::prelude::*;
use serde::Serialize;
use chrono;

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name =  crate::schema::rustaceans)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]  
pub struct Rustacean{
    pub id: Option<i32>,
    pub name:String,
    pub email:String,
    pub created_at: chrono::NaiveDateTime,
}