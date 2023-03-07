use std::{net::SocketAddr, str::FromStr};

pub struct Config {
    pub db_path: String,
    pub music: Vec<String>,
}

impl Config {
    pub fn read() -> Config {
        Config {
            db_path: "tmp.db".into(),
            music: vec![],
        }
    }
    pub fn addr(&self) -> SocketAddr {
        SocketAddr::from_str("127.0.0.1:2137").unwrap()
    }
}
