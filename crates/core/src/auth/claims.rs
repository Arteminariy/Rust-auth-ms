//! JWT claims for access and refresh tokens.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccessClaim {
    pub id: Uuid,
    pub name: String,
    pub role_id: Option<Uuid>,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RefreshClaim {
    pub sub: String, // user name (matches the original Rocket code)
    pub exp: usize,
}
