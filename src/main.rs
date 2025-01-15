use std::env;
use dotenv::dotenv;

mod libs;
mod models;
mod controllers;
mod routes;
mod templates;

#[tokio::main]
async fn main(){
    dotenv().ok();
    tracing_subscriber::fmt().init();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = libs::get_db_pool(&database_url).await;

    let app = routes::app_routes(pool);

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}