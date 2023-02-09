use rodio::{ Source, queue};
use rodio::{OutputStream, Sink};
use std::sync::mpsc::{channel, Receiver};
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
    

    
    
    let tau = 2.0 * PI;
    let hz = 440.0;
    let sample_func = Box::new(move |t:f32|{

        let outer_wave = (t * tau).sin().abs();
        let outer_wave2 = (t * tau + tau*0.25).sin().abs();
        let inner_wave =  |a:f32| (t * tau * a).sin().round();
        let detune = |func: &dyn Fn(f32) -> f32, input:f32, detune:f32|  (func(input - detune) + func(input) + func(input + detune))/3.0;
        let res = outer_wave * detune(&inner_wave, 440.0, 1.2) * outer_wave2;
        res.clamp(-1.0, 1.0)
    });

    let (tx,rx) = channel::<Box<dyn Fn(f32) -> f32 + Send>>();
    let source = SamplesSource {
        sample_func,
        new_sample_func: rx,
        t: 0,
    };
    // let source = SineWave::new(440.0);
    sink.append(source.amplify(0.3));

    sleep(Duration::from_secs(3));

    let sample_func = Box::new(move |t:f32|{

        let outer_wave = (t * tau).sin().abs();
        let outer_wave2 = (t * tau + tau*0.25).sin().abs();
        let inner_wave =  |a:f32| (t * tau * a).sin().round();
        let detune = |func: &dyn Fn(f32) -> f32, input:f32, detune:f32|  (func(input - detune) + func(input) + func(input + detune))/3.0;
        let res = outer_wave * detune(&inner_wave, 660.0, 1.0) * outer_wave2;
        res.clamp(-1.0, 1.0)
    });

    tx.send(sample_func).ok();

    loop{
        sleep(Duration::from_secs(1))
    }
}
pub struct SamplesSource {
    sample_func: Box<dyn Fn(f32) -> f32 + Send>,
    new_sample_func: Receiver<Box<dyn Fn(f32) -> f32 + Send>>,
    t: usize
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

        if let Ok(new_sample_func) = self.new_sample_func.try_recv(){
            self.sample_func = new_sample_func
        }
        let f = (self.sample_func)(self.t as f32  * INV_SAMPLE_RATE);
        self.t += 1;
        Some(f)
    }
}