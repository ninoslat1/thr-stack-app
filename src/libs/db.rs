use sqlx::mysql::MySqlPool;

pub async fn get_db_pool(database_url: &str) -> MySqlPool {
    MySqlPool::connect(database_url)
        .await
        .expect("Failed to create pool")
}