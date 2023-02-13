use bevy::prelude::{Plugin};

use web_sys::{AudioBufferSourceNode, AudioContext, AudioBuffer};

const SAMPLE_RATE: u32 = 48_000;
const INV_SAMPLE_RATE: f32 = 1.0 / (SAMPLE_RATE as f32);
const BUFFER_SIZE: u32 = SAMPLE_RATE;

pub struct SoundPlugin;

pub type SoundFn = Box<dyn Fn(f32) -> f32 + Send + Sync>;

impl Plugin for SoundPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {

        let ctx = web_sys::AudioContext::new().unwrap();

        // Create our web audio objects.
        let gain = ctx.create_gain().unwrap();
    
        
    
        let buffer_node = ctx.create_buffer_source().unwrap();
        buffer_node.set_loop(true);
    
        // Some initial settings:
        gain.gain().set_value(0.2); // starts muted
        buffer_node.connect_with_audio_node(&gain).unwrap();
        gain.connect_with_audio_node(&ctx.destination()).unwrap();
        buffer_node.start().unwrap();
        let buffer = ctx.create_buffer(1, BUFFER_SIZE, SAMPLE_RATE as f32).unwrap();
        let data =(0..BUFFER_SIZE).into_iter().map(|x| ((x as f32) * INV_SAMPLE_RATE * 2.0 * 3.141592 * 440.0).sin()).collect::<Vec<_>>();
        buffer.copy_to_channel(data.as_slice(), 0).unwrap();
        buffer_node.set_buffer(Some(&buffer));
        app.insert_non_send_resource(SoundControl{ctx, buffer_node, buffer});
    }
}


pub struct SoundControl{
    ctx: AudioContext,
    buffer_node: AudioBufferSourceNode,
    buffer: AudioBuffer,
}

impl SoundControl{
    pub fn set(&self, sound_fn: SoundFn){
        let data =(0..BUFFER_SIZE).into_iter().map(|x| (sound_fn((x as f32) * INV_SAMPLE_RATE))).collect::<Vec<_>>();
        self.buffer.copy_to_channel(data.as_slice(), 0).unwrap();
    }
}