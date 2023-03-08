use std::sync::{Arc, Mutex};

use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use serde::Deserialize;
use utoipa::ToSchema;

use crate::database::{Album, Song, DB};

#[derive(Debug, Deserialize, ToSchema)]
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
}

#[utoipa::path(
    post,
    path = "/lib/song",
    request_body = Query,
    responses(
        (status = 200, description = "Get array of Songs with title like", body = [Song]),
    )
)]
pub async fn song_by_title(
    Extension(db): Extension<Arc<Mutex<DB>>>,
    Json(payload): Json<Query>,
) -> impl IntoResponse {
    Json(Song::by_title(&db.lock().unwrap(), &payload.like))
}
#[utoipa::path(
    get,
    path = "/lib/song/{id}",
    responses(
        (status = 200, description = "Get Song by id", body = Song),
        (status = 404, description = "Song not found")
    )
)]
pub async fn song_by_id(
    Extension(db): Extension<Arc<Mutex<DB>>>,
    Path(id): Path<u32>,
) -> impl IntoResponse {
    match Song::by_id(&db.lock().unwrap(), id) {
        Some(song) => (StatusCode::OK, Json(song)).into_response(),
        None => (StatusCode::NOT_FOUND).into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/lib/song/album/{id}",
    responses(
        (status = 200, description = "Get array of Songs from album with id", body = [Song]),
    )
)]
pub async fn song_by_album_id(
    Extension(db): Extension<Arc<Mutex<DB>>>,
    Path(id): Path<u32>,
) -> impl IntoResponse {
    Json(Song::by_album_id(&db.lock().unwrap(), id))
}

#[utoipa::path(
    post,
    path = "/lib/album",
    request_body = Query,
    responses(
        (status = 200, description = "Get array of Albums with title like", body = [Album]),
    )
)]
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
#[utoipa::path(
    get,
    path = "/lib/album/{id}",
    responses(
        (status = 200, description = "Get Album by id", body = Album),
        (status = 404, description = "Album not found")
    )
)]
pub async fn album_by_id(
    Extension(db): Extension<Arc<Mutex<DB>>>,
    Path(id): Path<u32>,
) -> impl IntoResponse {
    match Album::by_id(&db.lock().unwrap(), id) {
        Some(album) => (StatusCode::OK, Json(album)).into_response(),
        None => (StatusCode::NOT_FOUND).into_response(),
    }
}
