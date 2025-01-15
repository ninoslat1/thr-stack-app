use crate::models::User;
use sqlx::MySqlPool;
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use tracing::info;

impl User {
    pub async fn login(pool: &MySqlPool, username: &str, password: &str) -> Result<Option<String>, sqlx::Error> {
        let encoded_password = STANDARD
            .encode(
                password
                    .chars()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .join("\u{0000}"),
            )
            .trim_end_matches("=")
            .to_string();
        // Query for UserName based on UserCode and encoded Password
        let row = sqlx::query!(
            "SELECT UserName FROM myuser WHERE UserCode = ? AND Password RLIKE ?",
            username,
            encoded_password
        )
        .fetch_optional(pool)
        .await?;

        // If user is found, return UserName, otherwise None
        Ok(row.map(|r| r.UserName))
    }
}
