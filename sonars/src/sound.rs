use bevy::prelude::{Plugin, Resource};

#[cfg(not(target_arch = "wasm32"))]
use web_audio_api::{
    context::{AudioContext, AudioContextOptions, BaseAudioContext},
    node::{AudioBufferSourceNode, AudioNode, AudioScheduledSourceNode, GainNode},
};
#[cfg(target_arch = "wasm32")]
use web_sys::{AudioBufferSourceNode, AudioContext, GainNode};

const SAMPLE_RATE: u32 = 48_000;
const INV_SAMPLE_RATE: f32 = 1.0 / (SAMPLE_RATE as f32);
const BUFFER_SIZE: u32 = SAMPLE_RATE * 2;

pub struct SoundPlugin;

pub type SoundFn = Box<dyn Fn(f32) -> f32>;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_non_send_resource(SoundControl::new());
    }
}

#[derive(Resource)]
pub struct SoundControl {
    ctx: AudioContext,
    gain: GainNode,
    buf: Option<AudioBufferSourceNode>,
}

const GAIN: f32 = 0.2;

impl SoundControl {
    pub fn new() -> Self {
        cfg_if::cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                let ctx = AudioContext::new().unwrap();
                let gain = ctx.create_gain().unwrap();
                gain.gain().set_value(GAIN);
                gain.connect_with_audio_node(&ctx.destination()).unwrap();
            } else {

                let ctx = AudioContext::new(AudioContextOptions {
                    latency_hint: web_audio_api::context::AudioContextLatencyCategory::Balanced,
                    sample_rate: Some(SAMPLE_RATE as f32),
                    sink_id: "".into(),
                    render_size_hint: web_audio_api::context::AudioContextRenderSizeCategory::Default,
                });

                let gain = ctx.create_gain();
                gain.connect(&ctx.destination());
                gain.gain().set_value(GAIN);
            }
        }

        Self {
            ctx,
            gain,
            buf: None,
        }
    }

    pub fn is_playing(&self) -> bool {
        self.buf.is_some()
    }

    pub fn set(&mut self, sound_fn: SoundFn) -> anyhow::Result<()> {
        let data = (0..BUFFER_SIZE)
            .into_iter()
            .map(|x| (sound_fn((x as f32) * INV_SAMPLE_RATE)))
            .collect::<Vec<_>>();

        cfg_if::cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                let new_buf = self.ctx.create_buffer_source().unwrap();
                new_buf.set_loop(true);
                new_buf.connect_with_audio_node(&self.gain).unwrap();
                new_buf.start().unwrap();
                let buf_data = &self.ctx
                    .create_buffer(1, BUFFER_SIZE, SAMPLE_RATE as f32)
                    .unwrap();
                buf_data.copy_to_channel(data.as_slice(), 0).unwrap();

                if self.buf.is_some() {
                    let old_buf = self.buf.take().unwrap();
                    old_buf.stop().ok();
                    old_buf.disconnect().ok();
                    drop(old_buf);
                }

                new_buf.set_buffer(Some(&buf_data));
            } else {
                let new_buf = self.ctx.create_buffer_source();
                new_buf.set_loop(true);
                new_buf.connect(&self.gain);
                new_buf.start();
                let mut buf_data = self.ctx
                    .create_buffer(1, BUFFER_SIZE as usize, SAMPLE_RATE as f32);
                buf_data.copy_to_channel(data.as_slice(), 0);

                if self.buf.is_some() {
                    let old_buf = self.buf.take().unwrap();
                    old_buf.stop();
                    old_buf.disconnect();
                    drop(old_buf);
                }

                new_buf.set_buffer(buf_data);
            }
        }

        self.buf = Some(new_buf);

        Ok(())
    }
}
