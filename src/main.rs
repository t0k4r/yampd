use crate::{
    database::{Album, Song, DB},
    player::sndchk,
};

mod database;
mod player;
fn main() {
    let db = DB::open("tmp.db").unwrap();
    sndchk();
    // db.update("/home/tokar/Music").unwrap();
    let s = Album::by_title(&db, "tw");
    println!("{:?}", s);
}
