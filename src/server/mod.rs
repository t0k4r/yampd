mod config;
mod library;
mod player;
use std::sync::{Arc, Mutex};

use axum::{Extension, Router};
pub use config::*;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{database::DB, player::Player};

#[derive(Debug, OpenApi)]
#[openapi(
    paths(
        library::song_by_id,
        library::song_by_title,
        library::song_by_album_id,
        library::album_by_title,
        library::album_by_id,
        player::play,
        player::pause,
        player::unpause,
        player::next,
        player::prev,
        player::index,
        player::delete,
        player::pos_seek_forw,
        player::pos_seek_back,
        player::pos_set,
        player::queue,
        player::queue_song,
        player::queue_album,
        player::now
    ),
    components(schemas(
        crate::database::Album,
        crate::database::Song,
        library::Query,
        player::Queue,
        player::Now
    ))
)]
struct ApiDoc;

pub struct Server {
    conf: Config,
    router: Router,
}

impl Server {
    pub fn new(conf: Config) -> Server {
        let db = DB::open(&conf.db_path).unwrap();
        conf.music.iter().for_each(|path| db.update(path).unwrap());
        let router = Router::new()
            .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()))
            .nest("/ply", player::player())
            .nest("/lib", library::library())
            .layer(Extension(Arc::new(Mutex::new(db))))
            .layer(Extension(Arc::new(Mutex::new(Player::new()))));

        Server { conf, router }
    }
    pub async fn run(self) {
        let srv = axum::Server::bind(&self.conf.addr())
            .serve(self.router.into_make_service())
            .await;
    }
}
