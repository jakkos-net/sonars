use rodio::{ Source};
use rodio::{OutputStream, Sink};
use std::{

    f32::consts::PI,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex, MutexGuard, TryLockError,
    },
    thread::sleep,
    time::{Duration, Instant},
};

const SAMPLE_RATE: u32 = 48_000;
const INV_SAMPLE_RATE: f32 = 1.0 / (SAMPLE_RATE as f32);

enum MExpr{
    Sin(E),
    Add(E, E),
    Mul(E, E),
    Div(E, E),
    Sub(E, E),
    Mod(E, E),
    Pow(E,E),
    Num(f32),
    Invoke(String),
    Time
}


enum MStmt{
    Assign(String, Vec<String>, MExpr)
}


type E = Box<MExpr>;

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Add a dummy source of the sake of the example.
    let start_time = Instant::now();
    let samples: Arc<Samples> = Default::default();
    let source = SamplesSource {
        samples: samples.clone(),
        t: (Instant::now() - start_time).as_secs_f32(),
    };
    // let source = SineWave::new(440.0);
    sink.append(source.amplify(0.3));

    
    let tau = 2.0 * PI;
    let hz = 440.0;
    samples
        .try_set(Box::new(move |t|{

            let outer_wave = (t * tau).sin().abs();
            let outer_wave2 = (t * tau + tau*0.25).sin().abs();
            let inner_wave =  |a:f32| (t * tau * a).sin().round();
            let detune = |func: &dyn Fn(f32) -> f32, input:f32, detune:f32|  (func(input - detune) + func(input) + func(input + detune))/3.0;
            let res = outer_wave * detune(&inner_wave, 440.0, 1.2) * outer_wave2;
            res.clamp(-1.0, 1.0)
        })).unwrap();

    loop{
        sleep(Duration::from_secs(1))
    }
}
pub struct Samples {
    buffer_a: Mutex<Box<dyn Fn(f32) -> f32 + Send>>,
    buffer_b: Mutex<Box<dyn Fn(f32) -> f32 + Send>>,
    a_is_read: AtomicBool,
    pending_switch: AtomicBool,
}

impl Default for Samples {
    fn default() -> Self {
        Self {
            buffer_a: Mutex::new(Box::new(|_| 0.0)),
            buffer_b: Mutex::new(Box::new(|_| 0.0)),
            a_is_read: AtomicBool::new(true),
            pending_switch: AtomicBool::new(false),
        }
    }
}

impl Samples {
    pub fn read(&self, t: f32) -> f32 {
        if self.pending_switch.load(Ordering::Relaxed) {
            self.a_is_read.fetch_xor(true, Ordering::Relaxed);
            self.pending_switch.store(false, Ordering::Relaxed);
        }

        let buffer = if self.a_is_read.load(Ordering::Relaxed) {
            self.buffer_a.lock().unwrap()
        } else {
            self.buffer_b.lock().unwrap()
        };

        buffer(t)
    }

    pub fn try_set(
        &self,
        f: Box<dyn Fn(f32) -> f32 + Send>,
    ) -> Result<(), TryLockError<MutexGuard<dyn Fn(f32) -> f32 + Send>>> {
        if !self.pending_switch.load(Ordering::Relaxed) {
            if self.a_is_read.load(Ordering::Relaxed) {
                if let Ok(mut buffer) = self.buffer_b.try_lock() {
                    self.pending_switch.store(true, Ordering::Relaxed);
                    *buffer = f;
                    return Ok(())
                }
            } else {
                if let Ok(mut buffer) = self.buffer_a.try_lock() {
                    self.pending_switch.store(true, Ordering::Relaxed);
                    *buffer = f;
                    return Ok(())
                }
            }
        }

        Err(TryLockError::WouldBlock)
    }
}

pub struct SamplesSource {
    samples: Arc<Samples>,
    t: f32,
}

impl Source for SamplesSource {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        SAMPLE_RATE
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}

impl Iterator for SamplesSource {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let f = self.samples.read(self.t);
        self.t += INV_SAMPLE_RATE;
        Some(f)
    }
}