use crate::database::DB;

mod database;
fn main() {
    let db = DB::open("tmp.db").unwrap();
    db.update("/home/tokar/Music").unwrap();
    println!("Hello, world!");
}
