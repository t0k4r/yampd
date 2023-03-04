use crate::database::DB;

mod database;
fn main() {
    let db = DB::open("tmp.db").unwrap();
    println!("Hello, world!");
}
