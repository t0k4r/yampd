use std::{fs::File, time::Duration};

use symphonia::{
    core::{
        audio::SampleBuffer,
        codecs::Decoder,
        formats::{FormatReader, Packet, SeekMode, SeekTo},
        io::MediaSourceStream,
        units::{Time, TimeBase},
    },
    default::{get_codecs, get_probe},
};

pub struct Source {
    fmt: Box<dyn FormatReader>,
    dec: Box<dyn Decoder>,
    channels: u8,
    sample_rate: u32,
    dur: Duration,
    pause: bool,
    end: bool,
    ts: u64,
    buf: Vec<f32>,
    tb: TimeBase,
}
impl Source {
    pub fn new(file: File) -> Result<Source, symphonia::core::errors::Error> {
        let fmt = get_probe()
            .format(
                &Default::default(),
                MediaSourceStream::new(Box::new(file), Default::default()),
                &Default::default(),
                &Default::default(),
            )?
            .format;
        let dec = get_codecs().make(
            &fmt.default_track().unwrap().codec_params,
            &Default::default(),
        )?;
        let tb = fmt.default_track().unwrap().codec_params.time_base.unwrap();
        let dur = tb.calc_time(fmt.default_track().unwrap().codec_params.n_frames.unwrap());
        let dur = Duration::from_secs_f64(dur.seconds as f64 + dur.frac);
        let channels = fmt
            .default_track()
            .unwrap()
            .codec_params
            .channels
            .unwrap()
            .count() as u8;
        let sample_rate = fmt
            .default_track()
            .unwrap()
            .codec_params
            .sample_rate
            .unwrap();

        Ok(Source {
            fmt,
            dec,
            dur,
            pause: false,
            end: false,
            ts: 0,
            buf: vec![],
            channels,
            sample_rate,
            tb,
        })
    }
    pub fn channels(&self) -> u8 {
        self.channels
    }
    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
    pub fn duration(&self) -> Duration {
        self.dur
    }
    pub fn position(&self) -> Duration {
        let pos = self.tb.calc_time(self.ts);
        Duration::from_secs_f64(pos.seconds as f64 + pos.frac)
    }
    pub fn set_position(&mut self, mut pos: Duration) {
        self.buf.clear();
        if pos > self.duration() {
            pos = self.duration()
        }
        let secs = (pos.as_millis() / 1_000) as u64;
        let mut frac = (pos.as_millis() % 1_000) as f64;
        while frac > 0. {
            frac /= 10.;
        }
        if secs == 0 && frac < 0.1 {
            frac = 0.1
        }
        self.fmt
            .seek(
                SeekMode::Coarse,
                SeekTo::Time {
                    time: Time {
                        seconds: secs,
                        frac: frac,
                    },
                    track_id: None,
                },
            )
            .unwrap();
        self.next()
    }
    pub fn ended(&self) -> bool {
        self.end
    }
    pub fn paused(&self) -> bool {
        self.pause
    }
    pub fn set_pause(&mut self, pause: bool) {
        self.pause = pause
    }
    fn packet(&mut self) -> Option<Packet> {
        match self.fmt.next_packet() {
            Ok(packet) => {
                self.ts = packet.ts();
                Some(packet)
            }
            Err(_) => None,
        }
    }
    fn decode(&mut self, packet: Packet) -> Vec<f32> {
        match self.dec.decode(&packet) {
            Ok(decoded) => {
                let mut sample_buf =
                    SampleBuffer::<f32>::new(decoded.capacity() as u64, *decoded.spec());
                sample_buf.copy_interleaved_ref(decoded);
                let samples = sample_buf.samples();
                return samples.to_owned();
            }
            Err(err) => {
                panic!("{}", err);
            }
        }
    }

    fn next(&mut self) {
        if let Some(packet) = self.packet() {
            let mut vec = self.decode(packet);
            self.buf.append(vec.as_mut())
        } else {
            self.end = true
        }
    }

    pub fn stream(&mut self, data: &mut [f32]) {
        for i in data {
            if !self.pause && !self.end {
                if self.buf.len() == 0 {
                    self.next();
                };
                if self.buf.len() != 0 {
                    *i = self.buf.remove(0)
                }
            } else {
                *i = 0.
            }
        }
    }
}
