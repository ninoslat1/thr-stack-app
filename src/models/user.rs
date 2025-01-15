use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub ID: i32,
    pub UserCode: String,
    pub UserName: String,
    pub Password: String,
    pub Position: Option<String>,
    pub Telephone: Option<String>,
    pub Email: Option<String>,
    pub Handphone: Option<String>,
    pub GroupID: i32,
    pub LogIn: i8,
    pub SecurityCode: String,
    pub IDCard: Option<String>,
    pub Status: i8,
    pub UserID: i32
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct LoginResponse {
    pub Password: String,
    pub UserCode: String
}