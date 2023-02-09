use std::{sync::mpsc::{SyncSender, sync_channel, Receiver}};
use bevy::prelude::Plugin;

use rodio::{Sink, OutputStream, Source};

const SAMPLE_RATE: u32 = 48_000;
const INV_SAMPLE_RATE: f32 = 1.0 / (SAMPLE_RATE as f32);

pub struct SoundPlugin;

// If the stream or sink are dropped, we lose sound output.
// So we need to keep them, however, they aren't currently used
pub struct SoundSystemResources{
    _stream: OutputStream,
    _sink: Sink
}

pub type SoundFn = Box<dyn Fn(f32) -> f32 + Send + Sync>;

impl Plugin for SoundPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        let (sender,receiver) = sync_channel::<SoundFn>(100);
        let source = SamplesSource {
            sample_func: Box::new(|t| (t * 2.0 * 3.141592 * 440.0).sin()),
            receiver,
            t: 0,
        };
        sink.append(source);
        app.insert_non_send_resource(SoundSystemResources{
            _stream: stream,
            _sink: sink
        });
        app.insert_resource(SoundControl{sender});
    }
}

pub struct SoundControl{
    sender: SyncSender<SoundFn>
}

impl SoundControl{
    pub fn set(&self, f:SoundFn)
    {
        self.sender.send(f).ok();
    }
}

pub struct SamplesSource {
    sample_func: SoundFn,
    receiver: Receiver<SoundFn>,
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