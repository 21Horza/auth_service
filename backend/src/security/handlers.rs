use warp::reply::Json;
use crate::db::Db;
use std::sync::Arc;
use common::*;
use crate::web::service::*;
use crate::security::*;
use crate::error::SecurityError;
use serde_json::json;


pub async fn register_user_handler(mut user: UserCreateRequest, db: Arc<Db>) -> Result<Json, Rejection> {
    println!("Sign up user request ...");
    
    match check_user_email(&db, &user.email).await {
        Ok(None) => (),
        Ok(user) => {
            println!("User {} already exists", &user.unwrap().email);
            return Ok(warp::reply::json(&json!({"status":"error", "message":"Unable to complete registration, email already registered"})))
        },
        _ => (),
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
            println!("Registration successful: {:?}", &email);
            return Ok(warp::reply::json(&json!({"status": "success"})));
        }
    }
}

pub async fn login_user_handler(user: LoginUser, db: Arc<Db>) -> Result<Json, Rejection> {
    println!("Logging request ...");

    let candidate = match check_user_email(&db, &user.email).await {
        Ok(None) => {
            return Err(warp::reject::custom(UserError::FindError))
        }
        Ok(usr) => usr,
        _ => {
            println!("Error getting a user {}", &user.email);
            return Ok(warp::reply::json(&json!({"status":"error", "message":"Email or password unknown"})));
        },
    };

    let found_user = candidate.unwrap();

    if !verify_pwd(&user.password, &found_user.password) {
        println!("Error verifyin password for user {}", &user.email);
        return Err(warp::reject::custom(SecurityError::InvalidCredentials))
    }

    let token = get_jwt(found_user);
    Ok(warp::reply::json(&token))
}