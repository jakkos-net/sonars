use std::{sync::mpsc::{SyncSender, sync_channel, Receiver}};
use bevy::prelude::Plugin;

use once_cell::sync::OnceCell;
use rodio::{Sink, OutputStream, Source};

const SAMPLE_RATE: u32 = 48_000;
const INV_SAMPLE_RATE: f32 = 1.0 / (SAMPLE_RATE as f32);

// To keep sound playing, I need to stop the Stream
// and the Sink from being dropped.
static STREAM: OnceCell<Stream> = OnceCell::new();
static SINK: OnceCell<Sink> = OnceCell::new();

// todo_major come up with a better solution
// However, OutputStream is not sent...
// BIG SCARY UNSAFE MAKES ME SAD :'(
// this should be fine as you can't 
// actually access the OutputStream inside
unsafe impl Sync for Stream{}
unsafe impl Send for Stream{}
struct Stream(OutputStream);

pub struct SoundPlugin;

impl Plugin for SoundPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {

        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        STREAM.set(Stream(stream)).ok();
        SINK.set(Sink::try_new(&stream_handle).unwrap()).ok();

        let (sender,receiver) = sync_channel::<Box<dyn Fn(f32) -> f32 + Send + Sync>>(100);
        let source = SamplesSource {
            sample_func: Box::new(|t| (t * 2.0 * 3.141592 * 440.0).sin()),
            receiver,
            t: 0,
        };
        SINK.get().unwrap().append(source);

        app.insert_resource(SoundControl{sender});
    }
}

pub struct SoundControl{
    sender: SyncSender<Box<dyn Fn(f32) -> f32 + Send + Sync>>
}

impl SoundControl{
    pub fn set(&self, f:Box<dyn Fn(f32) -> f32 + Send + Sync>)
    {
        self.sender.send(f).ok();
    }
}

pub struct SamplesSource {
    sample_func: Box<dyn Fn(f32) -> f32 + Send + Sync>,
    receiver: Receiver<Box<dyn Fn(f32) -> f32 + Send + Sync>>,
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

        if let Ok(new_sample_func) = self.receiver.try_recv(){
            self.sample_func = new_sample_func
        }
        let f = (self.sample_func)(self.t as f32  * INV_SAMPLE_RATE);
        self.t += 1;
        Some(f)
    }
}