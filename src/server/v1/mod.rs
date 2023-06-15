use axum::Router;

mod library;
mod player;

pub fn v1_api() -> Router {
    Router::new()
        .nest("/ply", player::player())
        .nest("/lib", library::library())
}
