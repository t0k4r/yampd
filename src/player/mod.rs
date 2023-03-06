use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use self::{source::Source, speaker::Speaker};

mod source;
mod speaker;

pub struct Player {}
impl Player {
    pub fn new() {}
    fn run() {}
}

pub fn sndchk() {
    let src = Source::new(
        std::fs::File::open(
            "/home/tokar/Music/The Strokes The New Abnormal 8 Not the Same Anymore.ogg",
        )
        .unwrap(),
    )
    .unwrap();
    let spk = Speaker::new(
        src.channels(),
        src.sample_rate(),
        Box::new(|| println!("emd")),
    );
    let s = Arc::new(Mutex::new(src));
    spk.play(s.clone());
    std::thread::sleep_ms(1_000);
    s.lock().unwrap().set_pause(true);
    std::thread::sleep_ms(1_000);
    s.lock().unwrap().set_pause(false);
    println!("{:?}", s.lock().unwrap().position());
    std::thread::sleep_ms(1_000);
    s.lock().unwrap().set_position(Duration::from_secs(20));
    println!("{:?}", s.lock().unwrap().position());
    s.lock().unwrap().set_position(Duration::from_millis(10));
    println!("{:?}", s.lock().unwrap().position());
    loop {}
}
