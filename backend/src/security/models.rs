use serde::{Deserialize, Serialize};

// for JWT token
#[derive(Deserialize, Serialize, Debug)]
pub struct Claims {
    pub sub: String, 
    pub role: String,
    pub exp: usize
}