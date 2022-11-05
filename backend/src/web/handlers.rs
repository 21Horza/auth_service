use crate::db::Db;
use std::sync::Arc;
use warp::{reply::Json, Rejection};
use super::service::*;
use uuid::Uuid;
use common::*;
use serde_json::json;
use crate::error::{UserError, SecurityError};
use crate::security::*;

pub async fn get_all_users(db: Arc<Db>) -> Result<Json, Rejection> {
    println!("Getting all users from db ...");
    let users = get_all(&db).await?;
    Ok(warp::reply::json(&users))
}

pub async fn get_one_user(user_id: String, db: Arc<Db>) -> Result<Json, Rejection> {
    println!("Getting a user by id from db ...");
    let id = Uuid::parse_str(&user_id).unwrap();
    let user = get_one(&db, id).await?;
    Ok(warp::reply::json(&user))
}

pub async fn create_user(mut user: UserCreateRequest, db: Arc<Db>) -> Result<Json, Rejection> {
    println!("Creating a user ...");
    
    match check_user_email(&db, &user.email).await {
        Ok(None) => (),
        Ok(user) => {
            println!("User with {} already exists", user.unwrap().email);
            return Ok(warp::reply::json(&json!({"status": "error", "message": "You can't create a user as this email already exists"})))
        },
        _ => ()
    }

    let pwd = get_hashed_pwd(&user.password);
    user.password = pwd;
    user.role = Some(Role::User);
    let email = user.email.clone();

    match create(&db, user).await {
        Err(e) => {
            println!("Error creating user {}: {:?}", &email, e.message);
            return Err(warp::reject::custom(UserError::CreateError))
        },
        _ => {
            println!("User with {:?} created", &email);
            return Ok(warp::reply::json(&json!({"status": "success"})));
        }
    }
}

pub async fn delete_user(user: UserDeleteRequest, db: Arc<Db>) -> Result<Json, Rejection> {
    println!("Deleting a user ...");
    
    match check_user_email(&db, &user.email).await {
        Ok(None) => {
            println!("User with {} doesn't exists", user.email);
            return Ok(warp::reply::json(&json!({"status": "error", "message": "You can't create a user as this email already exists"})))
        },
        Ok(_user) => (),
        _ => ()
    }

    delete(&db, user.email).await.map(|_e| UserError::DeleteError);
    Ok(warp::reply::json(&json!({"status":"success", "message":"User deleted"})))
}

pub async fn delete_all_users(db: Arc<Db>) -> Result<Json, Rejection> {
    println!("Deleting all users ...");

    delete_all(&db).await;
    Ok(warp::reply::json(&json!({"status":"success", "message":"All users deleted"})))
}

pub async fn update_user(user: UserUpdateRequest, db: Arc<Db>) -> Result<Json, Rejection> {
    println!("Updating a user ...");
    
    update(&db, user).await.map(|_e| UserError::UpdateError);
    Ok(warp::reply::json(&json!({"status":"success", "message":"User updated"})))
}

pub async fn update_user_pwd(pwd_req: PasswordUpdateRequest, db: Arc<Db>) -> Result<Json, Rejection> {
    println!("Updating a user's password ...");
    
    // // reject non-admins changing password of other users
    // if pwd_req.role != Role::Admin {
    //     return Err(warp::reject::custom(UserError::UpdateError))
    // };

    let result = get_one(&db, pwd_req.id).await?;

    let mut candidate = match result {
        Some(_) => result.unwrap(),
        _none => return Err(warp::reject::custom(UserError::UpdateError))
    };

    // if (user.id != candidate.id.to_string() && user.role != Role::Admin) || user.id == candidate.id.to_string() {
        if !verify_pwd(&pwd_req.current_password, &candidate.password) {
            return Err(warp::reject::custom(SecurityError::InvalidCredentials))
        }
    // }

    let hash = get_hashed_pwd(&pwd_req.new_password);

    candidate.password = hash;
    update_pwd(&db, candidate).await.map(|_e| UserError::UpdateError);

    Ok(warp::reply::json(&json!({"status":"success", "message":"Password updated"})))
}