use rusqlite::params;

use super::{DBObject, DB};

#[derive(Debug, Clone)]
pub struct Album {
    pub album_id: u32,
    pub artist_id: u32,
    pub title: String,
    pub artist: String,
    pub year: u32,
    pub songs: u32,
}
impl DBObject for Album {
    fn from_row(row: &rusqlite::Row) -> Result<Self, rusqlite::Error>
    where
        Self: Sized,
    {
        Ok(Album {
            album_id: row.get(0)?,
            artist_id: row.get(1)?,
            title: row.get(2)?,
            artist: row.get(3)?,
            year: row.get(4)?,
            songs: row.get(5)?,
        })
    }
}

impl Album {
    pub fn by_id(db: &DB, id: u32) -> Option<Album> {
        db.query(
            r#"
        SELECT al.album_id, al.artist_id, al.album_title, ar.artist_name, al.album_year, al.album_songs
        FROM Albums al
        JOIN Artists ar ON al.artist_id = ar.artist_id
        WHERE al.album_id = ?1
        "#,
            params![id],
        )
        .get(0)
        .cloned()
    }
    pub fn by_title(db: &DB, title: &str) -> Vec<Album> {
        db.query(
            r#"
        SELECT al.album_id, al.artist_id, al.album_title, ar.artist_name, al.album_year, al.album_songs
        FROM Albums al
        JOIN Artists ar ON al.artist_id = ar.artist_id
        WHERE al.album_title LIKE ?1
        ORDER BY al.album_title
        "#,
            params![format!("%{title}%")],
        )
    }
}
