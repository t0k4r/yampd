use crate::database::Song;

#[derive(Debug, Clone)]
pub struct Queue {
    pub index: usize,
    pub songs: Vec<(u32, String)>,
}
impl Queue {
    pub fn new() -> Queue {
        Queue {
            index: 0,
            songs: vec![],
        }
    }
    pub fn push(&mut self, song: Song) {
        self.songs.push((song.song_id, song.file));
    }
    pub fn delete(&mut self, index: usize) {
        if index < self.songs.len() {
            self.songs.remove(index);
        }
    }
    pub fn index(&mut self, index: usize) {
        self.index = index
    }
    pub fn next(&mut self) {
        if self.index < self.songs.len() - 1 {
            self.index += 1
        }
    }
    pub fn prev(&mut self) {
        if self.index != 0 {
            self.index -= 1
        }
    }
    pub fn remove(&mut self) {
        if self.index < self.songs.len() {
            self.songs.remove(self.index);
        }
    }
    pub fn now(&self) -> Option<(u32, String)> {
        self.songs.get(self.index).cloned()
    }
}
