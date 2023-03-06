use crate::database::{Album, Song, DB};

mod database;
fn main() {
    let db = DB::open("tmp.db").unwrap();
    // db.update("/home/tokar/Music").unwrap();
    let s = Album::by_title(&db, "tw");
    println!("{:?}", s);
}
