use std::sync::{Arc, Mutex};

use axum::{
    body::{Body, BoxBody, HttpBody},
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Extension, Json, Router,
};
use serde::Deserialize;

use crate::database::{Album, Cover, Song, DB};

#[derive(Debug, Deserialize)]
pub struct Query {
    like: String,
}

pub fn library() -> Router {
    Router::new()
        .route("/song", post(song_by_title))
        .route("/song/:id", get(song_by_id))
        .route("/song/album/:id", get(song_by_album_id))
        .route("/album", post(album_by_title))
        .route("/album/:id", get(album_by_id))
        .route("/cover/:id", get(cover_by_id))
}

pub async fn song_by_title(
    Extension(db): Extension<Arc<Mutex<DB>>>,
    Json(payload): Json<Query>,
) -> impl IntoResponse {
    Json(Song::by_title(&db.lock().unwrap(), &payload.like))
}

pub async fn song_by_id(
    Extension(db): Extension<Arc<Mutex<DB>>>,
    Path(id): Path<u32>,
) -> impl IntoResponse {
    match Song::by_id(&db.lock().unwrap(), id) {
        Some(song) => (StatusCode::OK, Json(song)).into_response(),
        None => (StatusCode::NOT_FOUND).into_response(),
    }
}

pub async fn song_by_album_id(
    Extension(db): Extension<Arc<Mutex<DB>>>,
    Path(id): Path<u32>,
) -> impl IntoResponse {
    Json(Song::by_album_id(&db.lock().unwrap(), id))
}

pub async fn album_by_title(
    Extension(db): Extension<Arc<Mutex<DB>>>,
    Json(payload): Json<Query>,
) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(Album::by_title(&db.lock().unwrap(), &payload.like)),
    )
        .into_response()
}

pub async fn album_by_id(
    Extension(db): Extension<Arc<Mutex<DB>>>,
    Path(id): Path<u32>,
) -> impl IntoResponse {
    match Album::by_id(&db.lock().unwrap(), id) {
        Some(album) => (StatusCode::OK, Json(album)).into_response(),
        None => (StatusCode::NOT_FOUND).into_response(),
    }
}

pub async fn cover_by_id(
    Extension(db): Extension<Arc<Mutex<DB>>>,
    Path(id): Path<u32>,
) -> impl IntoResponse {
    match Cover::by_album_id(&db.lock().unwrap(), id) {
        Some(c) => Response::builder()
            .header("Content-Type", "image/jpeg")
            .status(StatusCode::OK)
            .body(Body::from(c))
            .unwrap(),
        None => Response::builder()
            .header("Content-Type", "image/jpeg")
            .status(StatusCode::OK)
            .body(Body::from(vec![]))
            .unwrap(),
    }
}
