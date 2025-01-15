use axum::{extract::State, Form, response::Html};
use sqlx::MySqlPool;
use crate::models::{user::LoginResponse, User};
use tracing::info;

pub async fn authenticate(State(pool): State<MySqlPool>, Form(form): Form<LoginResponse>) -> Html<String> {
    match User::login(&pool, &form.UserCode, &form.Password).await {
        Ok(Some(user_name)) => {
            info!("User {} logged in", user_name);
            Html("<p>Login success</p>".to_string())
        }
        Ok(None) => {
            Html("<p>Invalid username or password</p>".to_string())
        }
        Err(_) => {
            Html("<p>Database error occurred</p>".to_string())
        }
    }
}
