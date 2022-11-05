use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub name: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UserResponse {
    pub id: uuid::Uuid,
    pub email: String,
    pub name: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct UserCreateRequest {
    pub email: String,
    pub name: String,
    pub password: String,
    pub role: Option<Role>,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct UserCreateResponse {
    pub status: String
}

#[derive(Deserialize)]
pub struct UserDeleteRequest {
    pub email: String,
    pub password: String
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct UserUpdateRequest {
    pub id: uuid::Uuid,
    pub email: String,
    pub name: String,
    pub role: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct PasswordUpdateRequest {
    pub id: uuid::Uuid,
    pub role: Role,
    pub current_password: String,
    pub new_password: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AuthUser {
    pub id: String,
    pub role: Role,
    pub login_at: DateTime<Utc>,
}

impl AuthUser {
    pub fn new(id: String, role: String) -> AuthUser {
        AuthUser {
            id: id,
            role: Role::from_str(&role),
            login_at: Utc::now(),
        }
    }
}

impl std::fmt::Display for AuthUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            _ => write!(f, "{}", &self.id)
        }
    }
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq)]
pub enum Role {
    User,
    Admin,
}

impl Role {
    pub fn from_str(role: &str) -> Role {
        match role.to_lowercase().as_str() {
            "admin" => Role::Admin,
            _ => Role::User,
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::User => write!(f, "User"),
            Role::Admin => write!(f, "Admin"),
        }
    }
}