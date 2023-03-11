use std::{fs::File, net::SocketAddr, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub db_path: String,
    pub music: Vec<String>,
    pub addr: String,
}

impl Config {
    pub fn read() -> Config {
        dirs::config_dir().unwrap().as_path().to_str().unwrap();
        if let Ok(file) = File::open(format!(
            "{}/yampd.json",
            dirs::config_dir().unwrap().as_path().to_str().unwrap()
        )) {
            serde_json::from_reader(file).unwrap()
        } else {
            let conf = Config::default();
            serde_json::to_writer_pretty(
                File::create(format!(
                    "{}/yampd.json",
                    dirs::config_dir().unwrap().as_path().to_str().unwrap()
                ))
                .unwrap(),
                &conf,
            )
            .unwrap();
            conf
        }
    }
    fn default() -> Config {
        Config {
            db_path: format!(
                "{}/yampd.db",
                dirs::cache_dir().unwrap().as_path().to_str().unwrap()
            ),
            music: dirs::audio_dir()
                .unwrap()
                .as_path()
                .to_str()
                .into_iter()
                .map(|s| s.into())
                .collect(),
            addr: "127.0.0.1:2137".into(),
        }
    }
    pub fn addr(&self) -> SocketAddr {
        SocketAddr::from_str(&self.addr).unwrap()
    }
}
