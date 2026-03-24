use std::sync::Arc;

use sqlx::SqlitePool;
use moka::future::Cache;

use crate::cache::{CachedResponse, Encoding};

pub struct AppState {
    data_db: SqlitePool,
    cache: Cache<(Arc<str>, Encoding), Arc<CachedResponse>>,
}

impl AppState {
    pub fn new(data_db: SqlitePool) -> AppState {
        AppState { 
            data_db,
            cache: Cache::builder()
                .max_capacity(50 * 1024 * 1024) // 50 MB cache
                .build()
        }
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.data_db
    }

    pub fn cache(&self) -> &Cache<(Arc<str>, Encoding), Arc<CachedResponse>> {
        &self.cache
    }
}

pub type SharedState = Arc<AppState>;