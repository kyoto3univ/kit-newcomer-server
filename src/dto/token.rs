use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaim {
    pub exp: i64,
    pub iat: i64,
    pub iss: String,
    pub sub: String,
}
