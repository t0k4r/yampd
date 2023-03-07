use server::{Config, Server};

mod database;
mod player;
mod server;
#[tokio::main]
async fn main() {
    Server::new(Config::read()).run().await
}
