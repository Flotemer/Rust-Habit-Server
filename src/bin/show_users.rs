extern crate rust_habit_server;
extern crate diesel;


use self::models::*;
use self::diesel::prelude::*;
use self::rust_habit_server::*;

fn main() {
    use self::schema::users::dsl::*;

    let connection = establish_connection();
    let results = users
        .limit(5)
        .load::<User>(&connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{} {}", user.first_name, user.last_name);
    }
}