use crate::{
    database::{Album, Song, DB},
    player::Player,
};

mod database;
mod player;
fn main() {
    let db = DB::open("tmp.db").unwrap();
    // sndchk();
    // db.update("/home/tokar/Music").unwrap();
    let ply = Player::new();
    // let p = Player::new();
    // loop {}
    let s = Song::by_title(&db, "");
    for z in &s {
        ply.push(z.clone());
    }

    // loop {}
    // std::mem::drop(s);
    println!("done");
    // ply.index(0);
    ply.next();
    // loop {
    std::thread::sleep_ms(1000000_000)
    // }
    // println!("{:?}", s);
}
