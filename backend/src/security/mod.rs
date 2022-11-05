#![allow(dead_code)]
#![allow(unused_variables)]

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use scrypt::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Scrypt,
};
use warp::{
    filters::header::headers_cloned,
    http::header::{HeaderMap, HeaderValue, AUTHORIZATION},
    reject, Filter, Rejection,
};
use std::result::Result;
use log::debug;
use common::*;
use crate::error::*;

pub mod handlers;
pub mod models;

pub fn get_hashed_pwd(pwd: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);

    let pwd_hash = Scrypt
            .hash_password(pwd.as_bytes(), &salt)
            .unwrap()
            .to_string();
    pwd_hash
}

pub fn verify_pwd(pwd: &str, pwd_hash: &str) -> bool {
    println!("hash: {}", &pwd_hash);
    println!("pwd: {}", &pwd);
    let parsed_hash = PasswordHash::new(pwd_hash).unwrap();

    Scrypt
        .verify_password(pwd.as_bytes(), &parsed_hash)
        .is_ok()
}

fn get_secret_key() -> Vec<u8> {
    std::env::var("JWT_SECRET").unwrap().into_bytes()
}

pub fn get_jwt(user: User) -> String {
    let expiration_time = Utc::now()
        .checked_add_signed(Duration::seconds(60))
        .expect("invalid timestamp")
        .timestamp();
    
    let user_claims = models::Claims {
        sub: user.name.clone(),
        role: user.role.clone(),
        exp: expiration_time as usize
    };

    let token = match encode(
        &Header::default(),
        &user_claims,
        &EncodingKey::from_secret(&get_secret_key())
    ) {
        Ok(t) => t,
        Err(_) => panic!()
    };

    token
}

pub fn with_auth(role: Role ) -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
    headers_cloned()
        .map(move |headers: HeaderMap<HeaderValue>| (role.clone(), headers))
        .and_then(authorize)
}

async fn authorize((role, headers): (Role, HeaderMap<HeaderValue>)) -> Result<String, Rejection> {
    match jwt_from_header(&headers) {
        Ok(jwt) => {
            let decoded = decode::<models::Claims>(
                &jwt,
                &DecodingKey::from_secret(&get_secret_key()),
                &Validation::default(),
            )
            .map_err(|_| reject::custom(AuthError::JWTTokenError))?;

            debug!("decoded claims: {:?}", &decoded.claims);
            if !is_auth(role, &decoded.claims.role) {
                return Err(reject::custom(AuthError::NotAuthorizedError));
            }
            
            Ok(decoded.claims.sub)
        }
        Err(e) => return Err(reject::custom(e)),
    }
}

// helper 
fn is_auth(role: Role, claims_role: &str) -> bool {
    let claims_role = Role::from_str(claims_role);
    debug!("needed role: {}, user role: {}", role, claims_role);
    role == claims_role || claims_role == Role::Admin
}

fn jwt_from_header(headers: &HeaderMap<HeaderValue>) -> Result<String, AuthError> {
    let header = match headers.get(AUTHORIZATION) {
        Some(v) => v,
        None => return Err(AuthError::NoAuthHeaderError),
    };
    let auth_header = match std::str::from_utf8(header.as_bytes()) {
        Ok(v) => v,
        Err(_) => return Err(AuthError::NoAuthHeaderError),
    };
    if !auth_header.starts_with("Bearer ") {
        return Err(AuthError::InvalidAuthHeaderError);
    }
    Ok(auth_header.trim_start_matches("Bearer ").to_owned())
}