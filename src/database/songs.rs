use rusqlite::params;
use serde::Serialize;
use utoipa::ToSchema;

use super::{DBObject, DB};

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct Song {
    pub song_id: u32,
    pub artist_id: u32,
    pub album_id: u32,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub ms: u32,
    #[serde(skip)]
    pub file: String,
}
impl DBObject for Song {
    fn from_row(row: &rusqlite::Row) -> Result<Self, rusqlite::Error>
    where
        Self: Sized,
    {
        Ok(Song {
            song_id: row.get(0)?,
            artist_id: row.get(1)?,
            album_id: row.get(2)?,
            title: row.get(3)?,
            artist: row.get(4)?,
            album: row.get(5)?,
            ms: row.get(6)?,
            file: row.get(7)?,
        })
    }
}

impl Song {
    pub fn by_id(db: &DB, id: u32) -> Option<Song> {
        db.query(
            r#"
        SELECT s.song_id, s.artist_id, s.album_id, s.song_title, ar.artist_name, al.album_title, s.song_ms, s.song_file
        FROM Songs s
        JOIN Artists ar ON s.artist_id = ar.artist_id
        JOIN Albums al ON s.album_id = al.album_id
        WHERE s.song_id = ?1
        "#,
            params![id],
        )
        .get(0)
        .cloned()
    }
    pub fn by_title(db: &DB, title: &str) -> Vec<Song> {
        db.query(
            r#"
        SELECT s.song_id, s.artist_id, s.album_id, s.song_title, ar.artist_name, al.album_title, s.song_ms, s.song_file
        FROM Songs s
        JOIN Artists ar ON s.artist_id = ar.artist_id
        JOIN Albums al ON s.album_id = al.album_id
        WHERE s.song_title LIKE ?1
        ORDER BY s.song_title
        "#,
            params![format!("%{title}%")],
        )
    }
    pub fn by_album_id(db: &DB, id: u32) -> Vec<Song> {
        db.query(
            r#"
        SELECT s.song_id, s.artist_id, s.album_id, s.song_title, ar.artist_name, al.album_title, s.song_ms, s.song_file
        FROM Songs s
        JOIN Artists ar ON s.artist_id = ar.artist_id
        JOIN Albums al ON s.album_id = al.album_id
        WHERE s.album_id = ?1
        ORDER BY s.song_index
        "#,
            params![id],
        )
    }
}
