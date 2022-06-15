//use crate::schema::check_ins;
use crate::schema::habits;
use crate::schema::users;
use chrono::naive::NaiveDate;
//use diesel::sql_types::TinyInt;

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone, Queryable)]
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
}

// Struct for creating User
#[derive(Debug, Clone, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Serialize, Debug, Clone, Queryable)]
pub struct Habit {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub frequency: String,
    // pub days: [i32; 7],
    //pub check_ins_per_freq: i32,
    pub start_date: chrono::naive::NaiveDate,
}

#[derive(Debug, Clone, Insertable)]
#[table_name = "habits"]
pub struct NewHabit {
    pub user_id: i64,
    pub name: String,
    pub frequency: String,
    //pub days: &'a Box<[i32]>,
    //pub check_ins_per_freq: i32,
    pub start_date: NaiveDate,
}
/*
#[derive(Serialize, Debug, Clone, Queryable)]
pub struct CheckIn {
    pub id: i64,
    pub habit_id: i64,
    pub date: String,
    pub count: i64,
}

#[derive(Debug, Clone, Insertable)]
#[table_name = "check_ins"]
pub struct NewCheckIn {
    pub id: i64,
    pub habit_id: i64,
    pub date: String,
    pub count: i64,
}
*/
