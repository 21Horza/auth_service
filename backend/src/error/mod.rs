#![allow(dead_code)]
#![allow(unused_variables)]

use thiserror::Error;
use serde::{Deserialize, Serialize};

pub mod handler;

#[derive(Error, Debug)]
pub enum SecurityError {
    #[error("invalid credentials")]
    InvalidCredentials,
    #[error("could not hash password")]
    HashError,
}
impl warp::reject::Reject for SecurityError {}

#[derive(Error, Debug, Serialize)]
pub enum AuthError {
    #[error("invalid credentials")]
    InvalidCredentialsError,
    #[error("jwt token not valid")]
    JWTTokenError,
    #[error("jwt token creation error")]
    JWTTokenCreationError,
    #[error("authorization header required")]
    NoAuthHeaderError,
    #[error("invalid auth header")]
    InvalidAuthHeaderError,
    #[error("permission denied")]
    NotAuthorizedError,
}
impl warp::reject::Reject for AuthError {}

impl From<sqlx::error::Error> for AuthError {
    fn from(_err: sqlx::error::Error) -> Self {
        AuthError::InvalidCredentialsError
    }
}

#[derive(Serialize, Debug)]
struct ErrorResponse {
    message: String,
    status: String,
}

#[derive(Error, Debug)]
pub enum UserError {
    #[error("could not create user")]
    CreateError,
    #[error("could not find user")]
    FindError,
    #[error("could not update user")]
    UpdateError,
    #[error("could not delete user")]
    DeleteError
}
impl warp::reject::Reject for UserError {}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DatabaseError {
    pub message: String,
}
impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            _ => write!(f, "Database error"),
        }
    }
}