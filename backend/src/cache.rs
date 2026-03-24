use std::{sync::Arc, usize};

use axum::{ 
    body::{Body, Bytes}, 
    extract::{Request, State}, 
    http::response::Parts, 
    middleware::Next, 
    response::Response
};

use crate::state::{SharedState};

pub async fn cache_response(
    State(state): State<SharedState>,
    request: Request,
    next: Next
) -> Response {
    if !request.uri().path().starts_with("/api") { // Bypass the middleware entirely
        return next.run(request).await
    }

    let key: Arc::<str> = request.uri()
        .path_and_query()
        .map(|pq| pq.as_str())
        .unwrap()
        .into();

    let headers = request
        .headers()
        .get(axum::http::header::ACCEPT_ENCODING);

    let headers = match headers {
        Some(header_value) => header_value.to_str().unwrap(),
        None => "identity"
    };

    let encodings = headers.split(",")
            .map(|encoding| Encoding::from(encoding.trim()));

    for encoding in encodings {
        let hash = (key.clone(), encoding);
        if state.cache().contains_key(&hash) {
            let cached_response = state.cache().get(&hash).await.unwrap(); // We know the key is in the cache
            return cached_response.build_response()
        }
    }

    let response = next.run(request).await;

    let encoding = response.headers()
        .get(axum::http::header::CONTENT_ENCODING)
        .and_then(|v| v.to_str().ok())
        .map(Encoding::from)
        .unwrap_or(Encoding::Identity);
        
    let (parts, body) = response.into_parts();
    let bytes = axum::body::to_bytes(body, usize::MAX).await.unwrap();

    let cached_response = Arc::new(CachedResponse {
        parts,
        body: bytes
    });

    state.cache().insert((key, encoding), cached_response.clone()).await;

    return cached_response.build_response()
}

#[derive(Debug)]
pub struct CachedResponse {
    parts: Parts,
    body: Bytes
}

impl CachedResponse {
    fn build_response(&self) -> Response {
        Response::from_parts(self.parts.clone(), Body::from(self.body.clone()))
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Encoding {
    Gzip,
    Deflate,
    Br,
    Zstd,
    Brotli,
    Identity
}

impl From<&str> for Encoding {
    fn from(value: &str) -> Self {
        match value {
            "gzip" => Self::Gzip,
            "deflate" => Self::Deflate,
            "br" => Self::Br,
            "Zstd" => Self::Zstd,
            "Brotli" => Self::Brotli,
            _ => Self::Identity
        }
    }
}