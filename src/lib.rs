#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

use self::models::{NewUser, User};

pub fn create_user<'a>(conn: &PgConnection, first_name: &'a str, last_name: &'a str) -> User {
    use schema::users;

    let new_user = NewUser {
        first_name: first_name.to_string(),
        last_name: last_name.to_string(),
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new user")
}

// Insert create_habits function here
