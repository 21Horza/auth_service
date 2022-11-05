#![allow(dead_code)]
#![allow(unused_variables)]

use warp::Rejection;
use common::*;
use sqlx::{query_as_unchecked, query_unchecked};
use crate::db::Db;
use uuid;
use chrono::Utc;
use crate::error::{SecurityError, DatabaseError};

pub async fn check_user_email(db: &Db, email: &str) -> Result<Option<User>, Rejection> {
    let user = query_as_unchecked!(
        User,
        r#"SELECT id, email, name, password, role, created_at, updated_at FROM users WHERE email = $1"#,
        email
    )
        .fetch_one(db)
        .await
        .map_err(|_e| {
            SecurityError::InvalidCredentials
        })
        .ok();

    Ok(user)
}

pub async fn create(db: &Db, user: UserCreateRequest) -> Result<u64, DatabaseError> {

    let candidate = query_unchecked!(
        r#"INSERT INTO users (id, email, name, password, role, created_at) VALUES ($1, $2, $3, $4, $5, $6)"#,
        uuid::Uuid::new_v4(),
        user.email,
        user.name,
        user.password,
        user.role.unwrap().to_string(),
        Utc::now(),
    )
        .execute(db)
        .await
        .map(|_| 0)
        .map_err(|_e| {
            let _reply = match _e.as_database_error() {
                None => println!("ERR"),
                Some(err) => {
                    println!("ERR {:?}", err.message().to_string());
                    return DatabaseError {  message: err.message().to_string() };
                }
            };
            return DatabaseError{ message: String::from("DB Error")};
        });

    return candidate;
}

pub async fn update(db: &Db, user: UserUpdateRequest) -> Option<Rejection> {
    query_unchecked!(
        r#"UPDATE users SET email=$1, name=$2, role=$3, updated_at=$4 WHERE id=$5"#,
        user.email,
        user.name,
        user.role,
        Utc::now(),
        user.id  
    )
        .execute(db)
        .await
        .unwrap();

    None
}

pub async fn delete(db: &Db, email: String) -> Option<Rejection> {
    query_unchecked!(
        r#"DELETE FROM users WHERE email=$1"#,
        email
    )
        .execute(db)
        .await
        .unwrap();
    None
}

pub async fn delete_all(db: &Db) -> Option<Rejection> {
    query_unchecked!(
        r#"DELETE FROM users"#,
    )
        .execute(db)
        .await
        .unwrap();
    None
}

pub async fn update_pwd(db: &Db, user: User) -> Option<Rejection> {
    query_unchecked!(
        r#"UPDATE users SET password=$1, updated_at=$2 WHERE id=$3"#,
        user.password,
        Utc::now(),
        user.id
    )
        .execute(db)
        .await
        .unwrap();
    None
}

pub async fn get_one(db: &Db, id: uuid::Uuid) -> Result<Option<User>, Rejection> {
    let user = query_as_unchecked!(
        User,
        r#"SELECT id, email, name, password, role, created_at, updated_at FROM users WHERE id = $1"#,
        id
    )
        .fetch_one(db)
        .await
        .map_err(|_e| {
            SecurityError::InvalidCredentials
        })
        .ok();
    Ok(user)
}

pub async fn get_all(db: &Db) -> Result<Option<Vec<User>>, Rejection> {
    let users = query_as_unchecked!(
        User,
        r#"select id, email, name, password, role, created_at, updated_at from users"#
    )
        .fetch_all(db)
        .await
        .map_err(|_e| { anyhow::Error::new(_e) })
        .ok();
    
    Ok(users)
}
