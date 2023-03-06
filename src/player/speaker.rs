use std::sync::{
    mpsc::{channel, Sender},
    Arc, Mutex,
};

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    OutputCallbackInfo, SampleRate, Stream, StreamConfig,
};

use super::source::Source;

pub struct Speaker {
    stream: Stream,
    cmd: Sender<Arc<Mutex<Source>>>,
}

impl Speaker {
    pub fn new(channels: u8, sample_rate: u32, on_end: Box<dyn Fn() + Send>) -> Speaker {
        let (snd, cmd) = channel();
        let mut src: Option<Arc<Mutex<Source>>> = None;
        let mut on_end_done = true;
        let stream = cpal::default_host()
            .default_output_device()
            .unwrap()
            .build_output_stream(
                &StreamConfig {
                    channels: channels as u16,
                    sample_rate: SampleRate(sample_rate),
                    buffer_size: cpal::BufferSize::Default,
                },
                move |data: &mut [f32], _: &OutputCallbackInfo| {
                    if let Ok(rec) = cmd.try_recv() {
                        src.replace(rec);
                        on_end_done = false;
                    }
                    if let Some(src) = &src {
                        let mut lock = src.lock().unwrap();
                        lock.stream(data);
                        if !on_end_done && lock.ended() {
                            on_end()
                        }
                    } else {
                        for i in data {
                            *i = 0.
                        }
                    }
                },
                |err| panic!("{err}"),
                None,
            )
            .unwrap();
        stream.play().unwrap();
        Speaker {
            stream: stream,
            cmd: snd,
        }
    }
    pub fn play(&self, src: Arc<Mutex<Source>>) {
        self.cmd.send(src).unwrap()
    }
}
