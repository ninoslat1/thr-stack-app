use std::{env, net::SocketAddr};
use axum::Router;
use dotenv::dotenv;
use sqlx::{MySql, Pool};
use tracing::info;

mod libs;
mod models;
mod controllers;
mod routes;
mod templates;

#[tokio::main]
async fn main(){
    dotenv().ok();
    tracing_subscriber::fmt().init();
    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let app_port: String = env::var("APP_PORT").expect("APP_PORT must be set");
    let addr:SocketAddr = format!("0.0.0.0:{}", app_port).parse().expect("Invalid port format");

    let pool: Pool<MySql> = libs::get_db_pool(&database_url).await;

    let app: Router = routes::app_routes(pool);

    info!("Server is running at http://{}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}