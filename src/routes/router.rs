use std::env::current_dir;

use axum::{routing::{get, post}, Router};
use sqlx::MySqlPool;
use tower_http::services::ServeDir;

use crate::{controllers::authenticate, templates::login_html};

pub fn app_routes(pool: MySqlPool) -> Router {
    let assets_path = current_dir().unwrap();
    Router::new()
            .route("/login", post(authenticate))
            .route("/", get(login_html))
            .nest_service(
                "/assets",
                ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
            )
            .with_state(pool)
}