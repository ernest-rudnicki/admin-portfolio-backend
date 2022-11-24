use serde::{Deserialize, Serialize};
use diesel::prelude::*;

#[derive(Queryable, Deserialize, Serialize)]
pub struct CreateUserData {
    pub username: String,
    pub email: String,
    pub password: String,
    pub created: String,
}