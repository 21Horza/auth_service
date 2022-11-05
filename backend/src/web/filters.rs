#![allow(dead_code)]
#![allow(unused_variables)]

use std::sync::Arc;
use crate::db::Db;
use warp::Filter;
use crate::db::with_db_pool;
use super::handlers::*;
use crate::security;
// use common::Role;

pub fn user_filters(
    base_path: &'static str, db: Arc<Db>
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let user_path = warp::path(base_path)
        .and(warp::path("users"));

    let conn = with_db_pool(db.clone());

    // get all
    let get_all_users = user_path
        .and(warp::get())
        .and(warp::path::end())
        .and(conn.clone())
        // .and(security::with_auth(Role::Admin))
        .and_then(get_all_users);

    // get one
    let get_one_user = user_path
        .and(warp::get())
        .and(warp::path::param())
        .and(conn.clone())
        .and_then(get_one_user);

    // create
    let create_user = user_path
        .and(warp::path("create"))
        .and(warp::post())
        .and(warp::body::json())
        .and(conn.clone())
        .and_then(create_user);

    // delete
    let delete_user = user_path
        .and(warp::path("delete"))
        .and(warp::delete())
        .and(warp::body::json())
        .and(conn.clone())
        .and_then(delete_user);

    // delete all
    let delete_users = user_path
        .and(warp::path::end())
        .and(warp::delete())
        .and(conn.clone())
        .and_then(delete_all_users);

    // update
    let update_user = user_path
        .and(warp::path("update"))
        .and(warp::put())
        .and(warp::body::json())
        .and(conn.clone())
        .and_then(update_user);

    // update pwd
    let update_pwd = user_path
        .and(warp::path!("update-pwd"))
        .and(warp::put())
        .and(warp::body::json())
        .and(conn.clone())
        .and_then(update_user_pwd);
    
    // --- AUTH ROUTES ---
    
    // login
    let login_user = user_path
        .and(warp::path!("auth" / "login"))
        .and(warp::post())
        .and(warp::body::json())
        .and(conn.clone())
        .and_then(security::handlers::login_user_handler); // security handler

    // register
    let register_user = user_path
        .and(warp::path!("auth" / "register"))
        .and(warp::post())
        .and(warp::body::json())
        .and(conn.clone())
        .and_then(security::handlers::register_user_handler); // security handler
   
    get_all_users
        .or(get_one_user)
        .or(create_user)
        .or(delete_user)
        .or(delete_users)
        .or(update_user)
        .or(login_user)
        .or(register_user)
        .or(update_pwd)
}