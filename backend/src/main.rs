use std::{error::Error, sync::Arc};

use axum::Router;

use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, cors::CorsLayer};

use sqlx::{Executor, sqlite::SqlitePoolOptions};

mod state;
mod models;
mod routes;
mod cache;

use state::AppState;
use cache::cache_response;
use tower_http::services::ServeDir;

use crate::state::SharedState;

const LANGUAGES: [&str; 9] = [
    "de",
    "en",
    "es",
    "fr",
    "it",
    "ja",
    "pt",
    "zh_hans",
    "zh_hant"
];

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create SQLite connection pool
    let data_db = SqlitePoolOptions::new()
        .after_connect(|conn, _| {
            Box::pin(async move {
                for &language in LANGUAGES.iter() {
                    conn.execute(format!("ATTACH DATABASE 'databases/translations/{language}.sqlite' AS {language}").as_str()).await?;
                }
                conn.execute("ATTACH DATABASE 'databases/skills.sqlite' AS skills").await?;
                Ok(())
            })
        })
        .connect("sqlite:databases/characters.sqlite")
        .await?;

    let app_state: SharedState = Arc::new(AppState::new(data_db));

    let origins = [
        "http://localhost:5173".parse().unwrap(),               // My vite frontend
        "https://ievr-database.onrender.com/".parse().unwrap(),
    ];

    let app_clone = app_state.clone();

    let app = Router::new()
        .nest("/api", routes::router())
        .with_state(app_state)
        .layer(ServiceBuilder::new()
            .layer(CorsLayer::new().allow_origin(origins))
            .layer(axum::middleware::from_fn_with_state(app_clone, cache_response))
            .layer(CompressionLayer::new())
        )
        .fallback_service(ServeDir::new("assets"));

    let address = "0.0.0.0:".to_owned() + &std::env::var("PORT")
        .unwrap_or(String::from("3000"));

    axum::serve(
        tokio::net::TcpListener::bind(address).await?,
        app,
    )
    .await?;

    Ok(())
}
