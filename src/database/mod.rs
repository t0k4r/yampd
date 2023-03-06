mod albums;
mod files;
mod songs;
pub use albums::*;
use rusqlite::{params, Connection, Params, Row};
pub use songs::*;

pub struct DB {
    conn: Connection,
}

impl DB {
    pub fn open(path: &str) -> Result<DB, rusqlite::Error> {
        let db = DB {
            conn: Connection::open(path)?,
        };
        db.init()?;
        Ok(db)
    }
    pub fn update(&self, path: &str) -> Result<(), rusqlite::Error> {
        for path in files::get_paths(path) {
            if let Some(file) = files::AudioFile::open(&path) {
                file.insert(self)?
            }
        }
        Ok(())
    }
    fn init(&self) -> Result<(), rusqlite::Error> {
        self.conn.execute(
            r#"
            CREATE TABLE IF NOT EXISTS Artists (
                artist_id INTEGER,
                artist_name TEXT,
                CONSTRAINT Artists_PK PRIMARY KEY (artist_id),
                CONSTRAINT Artists_UN UNIQUE (artist_name)
            );        
        "#,
            params![],
        )?;
        self.conn.execute(
            r#"
            CREATE TABLE IF NOT EXISTS Albums (
                album_id INTEGER,
                album_title TEXT,
                album_year INTEGER,
                artist_id INTEGER,
                album_songs INTEGER,
                album_cover BLOB,
                CONSTRAINT Albums_PK PRIMARY KEY (album_id),
                CONSTRAINT Albums_UN UNIQUE (album_title,album_year,artist_id),
                CONSTRAINT Albums_FK FOREIGN KEY (artist_id) REFERENCES Artists(artist_id)
            );
        "#,
            params![],
        )?;
        self.conn.execute(
            r#"
            CREATE TABLE IF NOT EXISTS Songs (
                song_id INTEGER,
                song_title TEXT,
                album_id INTEGER,
                artist_id INTEGER,
                song_index INTEGER,
                song_ms INTEGER,
                song_file TEXT,
                CONSTRAINT Songs_PK PRIMARY KEY (song_id),
                CONSTRAINT Songs_UN UNIQUE (song_file),
                CONSTRAINT Songs_UN UNIQUE (song_title,album_id,artist_id),
                CONSTRAINT Songs_FK FOREIGN KEY (artist_id) REFERENCES Artists(artist_id),
                CONSTRAINT Songs_FK_1 FOREIGN KEY (album_id) REFERENCES Albums(album_id)
            );
        "#,
            params![],
        )?;
        Ok(())
    }
    fn query<T: DBObject>(&self, sql: &str, params: impl Params) -> Vec<T> {
        self.conn
            .prepare(sql)
            .unwrap()
            .query_map(params, |row| T::from_row(row))
            .unwrap()
            .filter_map(|object| match object {
                Ok(object) => Some(object),
                Err(_) => None,
            })
            .collect()
    }
}

pub trait DBObject {
    fn from_row(row: &Row) -> Result<Self, rusqlite::Error>
    where
        Self: Sized;
}
