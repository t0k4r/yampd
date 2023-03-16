use std::{
    fs::File,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread::spawn,
    time::Duration,
};

use crate::database::Song;

use self::{queue::Queue, source::Source, speaker::Speaker};
mod queue;
pub mod source;
mod speaker;

enum Cmd {
    Play,
    Push(Song),
    Next,
    Prev,
    Index(usize),
    Delete(usize),
    Pause,
    SetPause(bool),
    Duration,
    Position,
    SetPosition(Duration),
    Queue,
    Ended,
    Now,
    Die,
}
enum Rpl {
    Is(bool),
    Time(Duration),
    Queue(Queue),
    Now(Option<(u32, String)>),
}
pub struct Player {
    cmd: Sender<Cmd>,
    rpl: Receiver<Rpl>,
}
impl Player {
    pub fn new() -> Player {
        let (cmd1, cmd2) = channel();
        let (rpl1, rpl2) = channel();
        Player::run(cmd1.clone(), cmd2, rpl1);
        Player {
            cmd: cmd1,
            rpl: rpl2,
        }
    }
    fn run(cmd_snd: Sender<Cmd>, cmd: Receiver<Cmd>, rpl: Sender<Rpl>) {
        spawn(move || {
            let play = || {
                cmd_snd.clone().send(Cmd::Play).unwrap();
            };
            let end = cmd_snd.clone();
            let on_end = Box::new(move || {
                end.send(Cmd::Ended).unwrap();
            });

            let mut queue = Queue::new();
            let mut source: Option<Arc<Mutex<Source>>> = None;
            let mut channels = 2;
            let mut sample_rate = 48_000;
            let mut speaker = Speaker::new(channels, sample_rate, on_end.clone());
            loop {
                match cmd.recv().unwrap() {
                    Cmd::Play => {
                        if let Some(now) = queue.now() {
                            if let Ok(file) = File::open(now.1) {
                                if let Ok(src) = Source::new(file) {
                                    if src.channels() != channels
                                        || src.sample_rate() != sample_rate
                                    {
                                        channels = src.channels();
                                        sample_rate = src.sample_rate();
                                        speaker =
                                            Speaker::new(channels, sample_rate, on_end.clone())
                                    }
                                    let src = Arc::new(Mutex::new(src));
                                    speaker.play(src.clone());
                                    source.replace(src);
                                }
                            }
                        } else {
                            speaker = Speaker::new(channels, sample_rate, on_end.clone())
                        }
                    }
                    Cmd::Push(song) => {
                        queue.push(song);
                    }
                    Cmd::Next => {
                        queue.next();
                        play()
                    }
                    Cmd::Prev => {
                        queue.prev();
                        play()
                    }
                    Cmd::Index(index) => {
                        queue.index(index);
                        play()
                    }
                    Cmd::Delete(index) => {
                        let is_now = index == queue.index;
                        queue.delete(index);
                        if is_now {
                            play()
                        }
                    }
                    Cmd::Pause => rpl
                        .send(Rpl::Is(match &source {
                            Some(src) => src.lock().unwrap().paused(),
                            None => false,
                        }))
                        .unwrap(),
                    Cmd::SetPause(pause) => {
                        if let Some(src) = &source {
                            let mut lock = src.lock().unwrap();
                            lock.set_pause(pause)
                        }
                    }
                    Cmd::Duration => rpl
                        .send(Rpl::Time(match &source {
                            Some(src) => src.lock().unwrap().duration(),
                            None => Duration::from_secs(0),
                        }))
                        .unwrap(),
                    Cmd::Position => rpl
                        .send(Rpl::Time(match &source {
                            Some(src) => src.lock().unwrap().position(),
                            None => Duration::from_secs(0),
                        }))
                        .unwrap(),
                    Cmd::SetPosition(pos) => {
                        if let Some(src) = &source {
                            let mut lock = src.lock().unwrap();
                            lock.set_position(pos)
                        }
                    }
                    Cmd::Queue => rpl.send(Rpl::Queue(queue.clone())).unwrap(),
                    Cmd::Ended => {
                        queue.remove();
                        play();
                    }
                    Cmd::Die => {
                        break;
                    }
                    Cmd::Now => rpl.send(Rpl::Now(queue.now())).unwrap(),
                }
            }
        });
    }
    pub fn play(&self) {
        self.cmd.send(Cmd::Play).unwrap()
    }
    pub fn push(&self, song: Song) {
        self.cmd.send(Cmd::Push(song)).unwrap();
    }
    pub fn index(&self, index: usize) {
        self.cmd.send(Cmd::Index(index)).unwrap();
    }
    pub fn next(&self) {
        self.cmd.send(Cmd::Next).unwrap();
    }
    pub fn prev(&self) {
        self.cmd.send(Cmd::Prev).unwrap();
    }
    pub fn delete(&self, index: usize) {
        self.cmd.send(Cmd::Delete(index)).unwrap();
    }
    pub fn set_pause(&self, pause: bool) {
        self.cmd.send(Cmd::SetPause(pause)).unwrap()
    }
    pub fn set_position(&self, pos: Duration) {
        self.cmd.send(Cmd::SetPosition(pos)).unwrap();
    }
    pub fn position(&self) -> Duration {
        self.cmd.send(Cmd::Position).unwrap();
        match self.rpl.recv().unwrap() {
            Rpl::Time(dur) => dur,
            _ => unreachable!(),
        }
    }
    pub fn duration(&self) -> Duration {
        self.cmd.send(Cmd::Duration).unwrap();
        match self.rpl.recv().unwrap() {
            Rpl::Time(dur) => dur,
            _ => unreachable!(),
        }
    }
    pub fn is_paused(&self) -> bool {
        self.cmd.send(Cmd::Pause).unwrap();
        match self.rpl.recv().unwrap() {
            Rpl::Is(is) => is,
            _ => unreachable!(),
        }
    }
    pub fn queue(&self) -> Queue {
        self.cmd.send(Cmd::Queue).unwrap();
        match self.rpl.recv().unwrap() {
            Rpl::Queue(que) => que,
            _ => unreachable!(),
        }
    }
    pub fn now(&self) -> Option<(u32, String)> {
        self.cmd.send(Cmd::Now).unwrap();
        match self.rpl.recv().unwrap() {
            Rpl::Now(now) => now,
            _ => unreachable!(),
        }
    }
}

impl Drop for Player {
    fn drop(&mut self) {
        self.cmd.send(Cmd::Die).unwrap()
    }
}
