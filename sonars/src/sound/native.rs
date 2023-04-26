use web_audio_api::{
    context::{AudioContext, AudioContextOptions, AudioContextRegistration, BaseAudioContext},
    node::{AudioNode, ChannelConfig},
    render::{AudioParamValues, AudioProcessor, AudioRenderQuantum, RenderScope},
};

use crate::sound::SAMPLE_RATE;

use super::{CURRENT_SOUND_FN, INV_SAMPLE_RATE};

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
        1
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

#[derive(Default)]
struct MyProcessor {
    sample_idx: u64,
}

impl AudioProcessor for MyProcessor {
    fn process(
        &mut self,
        _inputs: &[AudioRenderQuantum],
        outputs: &mut [AudioRenderQuantum],
        _params: AudioParamValues,
        _scope: &RenderScope,
    ) -> bool {
        let output = &mut outputs[0];
        output.set_number_of_channels(1);
        self.sample_idx += 128;
        let sound_fn_guard = CURRENT_SOUND_FN.lock().unwrap();
        let sound_fn = sound_fn_guard.as_ref();
        output.channels_mut().iter_mut().for_each(|buf| {
            buf.iter_mut().enumerate().for_each(|(i, output_sample)| {
                *output_sample = sound_fn((self.sample_idx + i as u64) as f32 * INV_SAMPLE_RATE);
            })
        });

        true
    }
}

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
        Self {
            ctx,
            time_start: time_start,
        }
    }
}
