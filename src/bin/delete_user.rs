extern crate rust_habit_server;
extern crate diesel;

use self::diesel::prelude::*;
use self::rust_habit_server::*;
use std::env::args;

fn main() {
    use rust_habit_server::schema::users::dsl::*;

    let id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");
    let connection = establish_connection();
    let num_deleted = diesel::delete(users.find(id))
        .execute(&connection)
        .expect("Error deleting user");

    println!("Deleted {} users", num_deleted);
}