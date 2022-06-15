use crate::errors::{AppError, ErrorType};
use crate::models::{NewUser, User};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};

type PooledPg = PooledConnection<ConnectionManager<PgConnection>>;

pub struct DBAccessManager {
    connection: PooledPg,
}

impl DBAccessManager {
    pub fn new(connection: PooledPg) -> DBAccessManager {
        DBAccessManager { connection }
    }

    pub fn create_user(&self, dto: NewUser) -> Result<User, AppError> {
        use super::schema::users;

        diesel::insert_into(users::table) // insert into users table
            .values(&dto) // use values from NewUser
            .get_result(&self.connection) // execute query
            .map_err(|err| AppError::from_diesel_err(err, "while creating user"))
        // if error occurred map it to AppError
    }

    pub fn list_users(&self) -> Result<Vec<User>, AppError> {
        use super::schema::users::dsl::*;

        users
            .load(&self.connection)
            .map_err(|err| AppError::from_diesel_err(err, "while listing users"))
    }
    pub fn delete_user(&self, id: i64) -> Result<usize, AppError> {
        use super::schema::users::dsl::*;

        let deleted = diesel::delete(users.filter(id.eq(id)))
            .execute(&self.connection)
            .map_err(|err| AppError::from_diesel_err(err, "while deleting user"))?;

        if deleted == 0 {
            return Err(AppError::new("User not found", ErrorType::NotFound));
        }
        return Ok(deleted);
    }
}
