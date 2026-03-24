use axum::Router;

mod common;
mod characters;
mod skills;

pub fn router() -> Router<crate::state::SharedState> {
    Router::new()
        .nest("/characters", characters::router())
        .nest("/skills", skills::router())
}