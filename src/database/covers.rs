use rusqlite::params;

use super::DB;

pub struct Cover {}

impl Cover {
    pub fn by_album_id(db: &DB, id: u32) -> Option<Vec<u8>> {
        match db.conn.query_row(
            "SELECT a.album_cover FROM Albums a WHERE a.album_id = ?",
            params![id],
            |row| row.get(0),
        ) {
            Ok(c) => Some(c),
            Err(_) => None,
        }
    }
}
