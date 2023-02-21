use bevy::prelude::Plugin;

#[cfg(not(target_arch = "wasm32"))]
use web_audio_api::{
    context::{AudioContext, AudioContextOptions, BaseAudioContext},
    node::{AudioNode, AudioScheduledSourceNode},
};
#[cfg(target_arch = "wasm32")]
use web_sys::AudioContext;

const SAMPLE_RATE: u32 = 48_000;
const INV_SAMPLE_RATE: f32 = 1.0 / (SAMPLE_RATE as f32);
const BUFFER_SIZE: u32 = SAMPLE_RATE;

pub struct SoundPlugin;

pub type SoundFn = Box<dyn Fn(f32) -> f32 + Send + Sync>;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_non_send_resource(SoundControl { ctx: None });
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

        cfg_if::cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                let ctx = AudioContext::new().unwrap();
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
            } else {

                let ctx = AudioContext::new(AudioContextOptions {
                    latency_hint: web_audio_api::context::AudioContextLatencyCategory::Balanced,
                    sample_rate: Some(SAMPLE_RATE as f32),
                    sink_id: "".into(),
                    render_size_hint: web_audio_api::context::AudioContextRenderSizeCategory::Default,
                });
                let buffer_node = ctx.create_buffer_source();
                let gain = ctx.create_gain();
                gain.gain().set_value(0.2);
                buffer_node.set_loop(true);
                gain.connect(&ctx.destination());
                buffer_node.connect(&gain);
                buffer_node.start();
                let mut buffer = ctx
                    .create_buffer(1, BUFFER_SIZE as usize, SAMPLE_RATE as f32);
                buffer.copy_to_channel(data.as_slice(), 0);

                if let Some(old_ctx) = &self.ctx {
                    old_ctx.close_sync();
                }

                buffer_node.set_buffer(buffer);
            }
        }

        self.ctx = Some(ctx);
    }
}
