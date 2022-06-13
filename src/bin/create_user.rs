extern crate rust_habit_server;
extern crate diesel;


use self::rust_habit_server::*;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();

    println!("What would you like your first name to be?");
    let mut first_name = String::new();
    stdin().read_line(&mut first_name).unwrap();
    println!("What would you like your first name to be?");
    let mut last_name = String::new();
    stdin().read_line(&mut last_name).unwrap();


    let user = create_user(&connection, &first_name, &last_name);
    println!("\nSaved user  {} {}", first_name, last_name);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";