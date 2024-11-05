use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenDetails {
    pub token: Option<String>,
    pub token_uuid: uuid::Uuid,
    pub user_id: String,
    pub main_user_id: String,
    pub expires_in: Option<i64>,
    pub user_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub token_uuid: String,
    pub exp: i64,
    pub iat: i64,
    pub nbf: i64,
    pub user_type: String,
    pub main_user_id: String,
}
