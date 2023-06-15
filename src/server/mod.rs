mod config;
mod v1;

use std::sync::{Arc, Mutex};

use axum::{Extension, Router};
pub use config::*;
use tower_http::cors::{Any, CorsLayer};

use crate::{database::DB, player::Player};

pub struct Server {
    conf: Config,
    router: Router,
}

impl Server {
    pub fn new(conf: Config) -> Server {
        let cors = CorsLayer::new().allow_origin(Any);
        let db = DB::open(&conf.db_path).unwrap();
        conf.music.iter().for_each(|path| db.update(path).unwrap());
        let router = Router::new()
            .nest("/v1", v1::v1_api())
            .layer(Extension(Arc::new(Mutex::new(db))))
            .layer(Extension(Arc::new(Mutex::new(Player::new()))))
            .layer(cors);

        Server { conf, router }
    }
    pub async fn run(self) {
        let _ = axum::Server::bind(&self.conf.addr())
            .serve(self.router.into_make_service())
            .await;
    }
}
