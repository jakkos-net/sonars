use bevy::ecs::system::Resource;
use itertools::izip;
use web_audio_api::{
    context::{AudioContext, AudioContextOptions, AudioContextRegistration, BaseAudioContext},
    node::{AudioNode, ChannelConfig},
    render::{AudioParamValues, AudioProcessor, AudioRenderQuantum, RenderScope},
};

use std::sync::Arc;

use crate::sound::SAMPLE_RATE;

use super::{empty_sound_fn, FloatOut, SoundFn, CURRENT_SOUND_FN, INV_SAMPLE_RATE, SAMPLE_INDEX};

pub fn setup_worklet(context: &AudioContext) {
    let noise = MyNode::new(context);
    noise.connect(&context.destination());
}

struct MyNode {
    registration: AudioContextRegistration,
    channel_config: ChannelConfig,
}

// implement required methods for AudioNode trait
impl AudioNode for MyNode {
    fn registration(&self) -> &AudioContextRegistration {
        &self.registration
    }

    fn channel_config(&self) -> &ChannelConfig {
        &self.channel_config
    }

    fn number_of_inputs(&self) -> usize {
        0
    }

    fn number_of_outputs(&self) -> usize {
        2
    }
}

impl MyNode {
    fn new<C: BaseAudioContext>(context: &C) -> Self {
        context.register(move |registration| {
            let render = MyProcessor::default();

            let node = MyNode {
                registration,
                channel_config: ChannelConfig::default(),
            };

            (node, Box::new(render))
        })
    }
}

struct MyProcessor {
    _sound_fn: Arc<SoundFn>,
}

impl Default for MyProcessor {
    fn default() -> Self {
        Self {
            _sound_fn: Arc::new(empty_sound_fn()),
        }
    }
}

impl AudioProcessor for MyProcessor {
    fn process(
        &mut self,
        _inputs: &[AudioRenderQuantum],
        outputs: &mut [AudioRenderQuantum],
        _params: AudioParamValues,
        _scope: &RenderScope,
    ) -> bool {
        let sample_idx = SAMPLE_INDEX.load(std::sync::atomic::Ordering::Relaxed);
        let output = &mut outputs[0];
        output.set_number_of_channels(2);
        // todo_major we should store a local copy of this and try lock instead, we don't want to be waiting on the lock while we should be processing audio
        let sound_fn_guard = CURRENT_SOUND_FN.lock().unwrap();
        let sound_fn = sound_fn_guard.as_ref();

        let channels = output.channels_mut();
        let (left, right) = channels.split_at_mut(1);
        let channel_0 = &mut left[0];
        let channel_1 = &mut right[0];

        izip!(channel_0.iter_mut(), channel_1.iter_mut())
            .enumerate()
            .for_each(|(i, (f0, f1))| {
                let idx = sample_idx + i;
                let t = idx as f64 * INV_SAMPLE_RATE;
                let [f0_n, f1_n] = sound_fn(t);
                *f0 = f0_n as FloatOut;
                *f1 = f1_n as FloatOut;
            });

        SAMPLE_INDEX.store(
            sample_idx + channel_0.len(),
            std::sync::atomic::Ordering::Relaxed,
        );

        true
    }
}

#[derive(Resource)]
pub struct SoundResources {
    pub ctx: AudioContext,
    pub time_start: f64,
}

impl Default for SoundResources {
    fn default() -> Self {
        let ctx = AudioContext::new(AudioContextOptions {
            latency_hint: web_audio_api::context::AudioContextLatencyCategory::Balanced,
            sample_rate: Some(SAMPLE_RATE as f32),
            sink_id: "".into(),
            render_size_hint: web_audio_api::context::AudioContextRenderSizeCategory::Default,
        });

        setup_worklet(&ctx);

        let time_start = ctx.current_time();
        Self { ctx, time_start }
    }
}