use serde::Serialize;

use serde_derive::{Deserialize};
use crate::models::{NewUser};
use crate::data_access::DBAccessManager;
use crate::AppError;

#[derive(Debug, Deserialize, Clone)]
pub struct AddUser {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct IdResponse {
    pub id: i64,
}

impl IdResponse {
    pub fn new(id: i64) -> IdResponse {
        IdResponse{id}
    }
}



impl AddUser {
    pub fn to_dto(&self) -> NewUser {
        NewUser{
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
        }
    }
}


fn respond<T: Serialize>(result: Result<T, AppError>, status: warp::http::StatusCode) -> Result<impl warp::Reply, warp::Rejection> {
    match result {
        Ok(response) => {
            Ok(warp::reply::with_status(warp::reply::json(&response), status))
        }
        Err(err) => {
            log::error!("Error while trying to respond: {}", err.to_string());
            Err(warp::reject::custom(err))
        }
    }
}

pub async fn add_user(
    db_manager: DBAccessManager,
    new_user: AddUser,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("handling add user");

        let create_user = new_user.to_dto();

    let id_response = db_manager.create_user(create_user).map(|user|
        { IdResponse::new(user.id) }
    );

    respond(id_response, warp::http::StatusCode::CREATED)
}

pub async fn delete_user(
    id: i64,
    db_manager: DBAccessManager,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("handling delete user");

    let result = db_manager.delete_user(id).map(|_| -> () {()});

    respond(result, warp::http::StatusCode::NO_CONTENT)
}

pub async fn list_users(
    db_manager: DBAccessManager,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("handling list users");

    let result = db_manager.list_users();

    respond(result, warp::http::StatusCode::OK)
}