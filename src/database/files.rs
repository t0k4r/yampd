use std::fs::read_dir;

use lofty::{Accessor, Probe, TaggedFileExt};
use rusqlite::params;

use super::DB;

pub struct AudioFile {
    album_artist: String,
    album_title: String,
    album_year: u32,
    album_songs: u32,
    album_cover: Vec<u8>,
    song_artist: String,
    song_title: String,
    song_flie: String,
    song_index: u32,
    song_ms: u32,
}

impl AudioFile {
    pub fn open(path: &str) -> Option<AudioFile> {
        match Probe::open(path).unwrap().read() {
            Ok(tag) => tag.primary_tag().and_then(|tag| {
                Some(AudioFile {
                    album_artist: tag.get_string(&lofty::ItemKey::AlbumArtist)?.into(),
                    album_title: tag.album()?.into(),
                    album_year: tag.year()?,
                    album_songs: tag.track_total()?,
                    album_cover: tag.pictures().get(0)?.data().to_owned(),
                    song_artist: tag.artist()?.into(),
                    song_title: tag.title()?.into(),
                    song_flie: path.into(),
                    song_index: tag.track()?,
                    song_ms: 0, // TODO: REDO WHEN AUDIO SOURCE IS DONE
                })
            }),
            Err(_) => None,
        }
    }
    pub fn insert(&self, db: &DB) -> Result<(), rusqlite::Error> {
        db.conn.execute(
            r#"
            INSERT OR IGNORE INTO Artists(artist_name) 
            VALUES (?1), (?2);"#,
            params![self.song_artist, self.album_artist],
        )?;
        let mut artist_query = db.conn.prepare(
            r#"
            SELECT a.artist_id 
            FROM Artists a 
            WHERE a.artist_name = ?1;"#,
        )?;
        let album_artist_id: u32 =
            artist_query.query_row(params![self.album_artist], |row| Ok(row.get(0)?))?;
        let artist_id: u32 =
            artist_query.query_row(params![self.song_artist], |row| Ok(row.get(0)?))?;

        db.conn.execute(
            r#"
            INSERT OR IGNORE INTO Albums(album_title, album_year, artist_id, album_songs, album_cover)
            VALUES (?1,?2,?3,?4,?5);"#,
            params![
                self.album_title,
                self.album_year,
                album_artist_id,
                self.album_songs,
                self.album_cover,
            ],
        )?;
        let album_id: u32 = db.conn.query_row(
            r#"
            SELECT a.album_id
            FROM Albums a
            WHERE a.album_title = ?1;"#,
            params![self.album_title],
            |row| Ok(row.get(0)?),
        )?;

        db.conn.execute(
            r#"
            INSERT OR IGNORE INTO Songs(song_title, album_id, artist_id, song_file, song_index, song_ms)
            VALUES (?1,?2,?3,?4,?5,?6)"#,
            params![
                self.song_title,
                album_id,
                artist_id,
                self.song_flie,
                self.song_index,
                self.song_ms
            ],
        )?;
        Ok(())
    }
}

pub fn get_paths(path: &str) -> Vec<String> {
    let mut paths = vec![];
    read_dir(path).unwrap().for_each(|entry| {
        if let Ok(entry) = entry {
            let meta = entry.metadata().unwrap();
            let path = entry.path();
            let path = path.as_path().to_str().unwrap();
            let is_music = path.ends_with(".ogg")
                || path.ends_with(".mp3")
                || path.ends_with(".m4a")
                || path.ends_with(".wav")
                || path.ends_with(".flac");
            if meta.is_file() && is_music {
                paths.push(path.to_owned())
            } else if meta.is_dir() {
                paths.append(&mut get_paths(path))
            }
        }
    });
    paths
}
