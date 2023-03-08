use std::{fs::File, io::Read, net::SocketAddr, str::FromStr};

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub db_path: String,
    pub music: Vec<String>,
    pub addr: String,
}

impl Config {
    pub fn read() -> Config {
        let mut buf = String::new();
        File::open("yampd.json")
            .unwrap()
            .read_to_string(&mut buf)
            .unwrap();
        serde_json::from_str(&buf).unwrap()
    }
    pub fn addr(&self) -> SocketAddr {
        SocketAddr::from_str(&self.addr).unwrap()
    }
}
