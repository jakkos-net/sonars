use std::f32::consts::PI;

use bevy::prelude::Plugin;

use web_sys::{AudioBufferSourceNode, AudioContext};

const SAMPLE_RATE: u32 = 48_000;
const INV_SAMPLE_RATE: f32 = 1.0 / (SAMPLE_RATE as f32);
const BUFFER_SIZE: u32 = SAMPLE_RATE;

pub struct SoundPlugin;

pub type SoundFn = Box<dyn Fn(f32) -> f32 + Send + Sync>;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let mut sound_control = SoundControl { ctx: None };
        sound_control.set(Box::new(move |t| (t * 440.0 * 2.0 * PI).sin()));
        app.insert_non_send_resource(sound_control);
    }
}

pub struct SoundControl {
    ctx: Option<AudioContext>,
}

impl SoundControl {
    pub fn set(&mut self, sound_fn: SoundFn) {
        let data = (0..BUFFER_SIZE)
            .into_iter()
            .map(|x| (sound_fn((x as f32) * INV_SAMPLE_RATE)))
            .collect::<Vec<_>>();

        let ctx = web_sys::AudioContext::new().unwrap();
        let gain = ctx.create_gain().unwrap();
        let buffer_node = ctx.create_buffer_source().unwrap();
        buffer_node.set_loop(true);
        gain.gain().set_value(0.2);
        buffer_node.connect_with_audio_node(&gain).unwrap();
        gain.connect_with_audio_node(&ctx.destination()).unwrap();
        buffer_node.start().unwrap();
        let buffer = ctx
            .create_buffer(1, BUFFER_SIZE, SAMPLE_RATE as f32)
            .unwrap();
        buffer.copy_to_channel(data.as_slice(), 0).unwrap();

        if let Some(old_ctx) = &self.ctx {
            old_ctx.close().ok();
        }

        buffer_node.set_buffer(Some(&buffer));

        self.ctx = Some(ctx);
    }
}
