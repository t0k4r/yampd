use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use serde::Serialize;

use crate::{
    database::{Song, DB},
    player::Player,
};

#[derive(Debug, Serialize)]
pub struct Queue {
    index: usize,
    songs: Vec<Song>,
}

#[derive(Debug, Serialize)]
pub struct Now {
    id: u32,
    pos: u128,
    dur: u128,
    pause: bool,
}

pub fn player() -> Router {
    Router::new()
        .route("/play", post(play))
        .route("/pause", post(pause))
        .route("/unpause", post(unpause))
        .route("/next", post(next))
        .route("/prev", post(prev))
        .route("/index/:idx", post(index).delete(delete))
        .route("/pos/seek/forw/:ms", post(pos_seek_forw))
        .route("/pos/seek/back/:ms", post(pos_seek_back))
        .route("/pos/set/:ms", post(pos_set))
        .route("/queue", get(queue))
        .route("/queue/song/:id", post(queue_song))
        .route("/queue/album/:id", post(queue_album))
        .route("/now", get(now))
}

pub async fn play(Extension(ply): Extension<Arc<Mutex<Player>>>) {
    ply.lock().unwrap().play()
}

pub async fn pause(Extension(ply): Extension<Arc<Mutex<Player>>>) {
    ply.lock().unwrap().set_pause(true)
}

pub async fn unpause(Extension(ply): Extension<Arc<Mutex<Player>>>) {
    ply.lock().unwrap().set_pause(false)
}

pub async fn next(Extension(ply): Extension<Arc<Mutex<Player>>>) {
    ply.lock().unwrap().next()
}

pub async fn prev(Extension(ply): Extension<Arc<Mutex<Player>>>) {
    ply.lock().unwrap().prev()
}

pub async fn index(Extension(ply): Extension<Arc<Mutex<Player>>>, Path(index): Path<usize>) {
    ply.lock().unwrap().index(index)
}

pub async fn delete(Extension(ply): Extension<Arc<Mutex<Player>>>, Path(index): Path<usize>) {
    ply.lock().unwrap().delete(index)
}

pub async fn pos_seek_forw(Extension(ply): Extension<Arc<Mutex<Player>>>, Path(ms): Path<u32>) {
    let lock = ply.lock().unwrap();
    let now = lock.position();
    lock.set_position(now + Duration::from_millis(ms.into()))
}

pub async fn pos_seek_back(Extension(ply): Extension<Arc<Mutex<Player>>>, Path(ms): Path<u32>) {
    let lock = ply.lock().unwrap();
    let now = lock.position();
    let dur = Duration::from_millis(ms.into());
    lock.set_position(match now < dur {
        true => Duration::from_secs(0),
        false => now - dur,
    })
}

pub async fn pos_set(Extension(ply): Extension<Arc<Mutex<Player>>>, Path(ms): Path<u32>) {
    ply.lock()
        .unwrap()
        .set_position(Duration::from_millis(ms.into()))
}

pub async fn queue(
    Extension(ply): Extension<Arc<Mutex<Player>>>,
    Extension(db): Extension<Arc<Mutex<DB>>>,
) -> impl IntoResponse {
    let q = ply.lock().unwrap().queue();
    Json(Queue {
        index: q.index,
        songs: q
            .songs
            .iter()
            .filter_map(|(id, _)| Song::by_id(&db.lock().unwrap(), id.clone()))
            .collect(),
    })
}

pub async fn queue_song(
    Extension(ply): Extension<Arc<Mutex<Player>>>,
    Extension(db): Extension<Arc<Mutex<DB>>>,
    Path(id): Path<u32>,
) {
    if let Some(song) = Song::by_id(&db.lock().unwrap(), id) {
        ply.lock().unwrap().push(song)
    }
}

pub async fn queue_album(
    Extension(ply): Extension<Arc<Mutex<Player>>>,
    Extension(db): Extension<Arc<Mutex<DB>>>,
    Path(id): Path<u32>,
) {
    for song in Song::by_album_id(&db.lock().unwrap(), id) {
        ply.lock().unwrap().push(song)
    }
}

pub async fn now(Extension(ply): Extension<Arc<Mutex<Player>>>) -> impl IntoResponse {
    let lock = ply.lock().unwrap();
    if let Some((id, _)) = lock.now() {
        (
            StatusCode::OK,
            Json(Now {
                id: id,
                pos: lock.position().as_millis(),
                dur: lock.duration().as_millis(),
                pause: lock.is_paused(),
            }),
        )
            .into_response()
    } else {
        (StatusCode::NOT_FOUND).into_response()
    }
}
