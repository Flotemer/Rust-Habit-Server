
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use crate::data_access::DBAccessManager;
use crate::errors::{AppError, ErrorType};

use std::env;
use warp::{Filter, reject};
use log::{info};

use serde::de::DeserializeOwned;
use crate::api::{AddUser};

#[macro_use]
extern crate diesel;

mod data_access;
mod errors;
mod models;
mod schema;
mod api;

type PgPool = Pool<ConnectionManager<PgConnection>>;

fn pg_pool(db_url: &str) -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::new(manager).expect("Postgres connection pool could not be created")
}

fn with_db_access_manager(pool: PgPool) -> impl Filter<Extract = (DBAccessManager,), Error = warp::Rejection> + Clone {
    warp::any()
        .map(move || pool.clone())
        .and_then(|pool: PgPool| async move {  match pool.get() {
            Ok(conn) => Ok(DBAccessManager::new(conn)),
            Err(err) => Err(reject::custom(
                AppError::new(format!("Error getting connection from pool: {}", err.to_string()).as_str(), ErrorType::Internal))
            ),
        }})
}

#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    let database_url = "postgres://postgres:postgres@habit-db.catjyboeofqs.us-east-1.rds.amazonaws.com/habit-db";

    let pg_pool = pg_pool(database_url);

    let routes = api_filters(pg_pool)
        .recover(errors::handle_rejection);

    info!("Starting server on port 3030...");

    // Start up the server...
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn with_json_body<T: DeserializeOwned + Send>(
) -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

/// POST /user
fn add_user(
    pool: PgPool
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("users")                    // Match /books path
        .and(warp::post())                  // Match POST method
        .and(with_db_access_manager(pool))  // Add DBAccessManager to params tuple
        .and(with_json_body::<AddUser>())   // Try to deserialize JSON body to AddBook
        .and_then(api::add_user)            // Pass the params touple to the handler function
}

/// DELETE /delete/:id
fn delete_user(
    pool: PgPool
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("users" / i64 )
        .and(warp::delete())
        .and(with_db_access_manager(pool))
        .and_then(api::delete_user)
}

fn api_filters(
    pool: PgPool
) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone  {
    warp::path!("api" / "v1" / ..)   // Add path prefix /api/v1 to all our routes
        .and(
            add_user(pool.clone())
                .or(delete_user(pool.clone()))
                .or(list_users(pool))
        )
}

/// GET /users
fn list_users(
    pool: PgPool
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("users")
        .and(warp::get())
        .and(with_db_access_manager(pool))
        .and_then(api::list_users)
}