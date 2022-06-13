#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}

use super::schema::users;

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
}