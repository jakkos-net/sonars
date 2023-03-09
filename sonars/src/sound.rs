use std::sync::Mutex;

use bevy::prelude::{NonSendMut, Plugin, ResMut, Resource};

use crossbeam_queue::SegQueue;
use once_cell::sync::Lazy;
use web_audio_api::{render::{AudioRenderQuantum, RenderScope, AudioParamValues, AudioProcessor}, node::ChannelConfig, context::AudioContextRegistration};
#[cfg(not(target_arch = "wasm32"))]
use web_audio_api::{
    context::{AudioContext, AudioContextOptions, BaseAudioContext},
    node::{AudioBufferSourceNode, AudioNode, AudioScheduledSourceNode, GainNode},
};
#[cfg(target_arch = "wasm32")]
use web_sys::{AudioBufferSourceNode, AudioContext, GainNode};

const SAMPLE_RATE: u32 = 48_000;
const INV_SAMPLE_RATE: f32 = (1.0 / (SAMPLE_RATE as f64)) as f32;
const BUFFER_INV_TIME_LENGTH: u32 = 1; // 1/N seconds per buffer, keep this a factor of SAMPLE_RATE
const BUFFER_SIZE: u32 = SAMPLE_RATE / BUFFER_INV_TIME_LENGTH;
// const BUFFER_TIME_LENGTH: f64 = BUFFER_SIZE as f64 / SAMPLE_RATE as f64; // it will take up to 2x TIME_LENGTH for changes to be heard
const BUFFER_TIME_LENGTH: f64 = 1.0;

pub struct SoundPlugin;


pub type SoundFn = Box<dyn Fn(f32) -> f32 + Send + Sync>;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<SoundControl>();
        app.init_non_send_resource::<SoundResources>();
        app.add_system(update_sound);
    }
}

fn update_sound(
    mut sound_control: ResMut<SoundControl>,
) {
    sound_control.update();
}

#[derive(Resource)]
pub struct SoundControl {
    queue: SegQueue<SoundFn>,
    next_fn: SoundFn,
}

impl Default for SoundControl {
    fn default() -> Self {
        Self {
            queue: Default::default(),
            next_fn: Box::new(|_| 0.0),
        }
    }
}

impl SoundControl {
    pub fn push(&self, new_fn: SoundFn) {
        self.queue.push(new_fn);
    }

    fn update(&mut self) {
        while !self.queue.is_empty() {
            self.next_fn = self.queue.pop().unwrap();
        }
    }

    pub fn current(&self) -> &SoundFn {
        &self.next_fn
    }
}

pub fn push_sound(new_fn: SoundFn){
    *CURRENT_SOUND_FN.lock().unwrap() = new_fn;
}

struct SoundResources {
    pub ctx: AudioContext,
    pub time_start: f64,
}

impl Default for SoundResources {
    fn default() -> Self {
        cfg_if::cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                let ctx = AudioContext::new().unwrap();
            } else {

                let ctx = AudioContext::new(AudioContextOptions {
                    latency_hint: web_audio_api::context::AudioContextLatencyCategory::Balanced,
                    sample_rate: Some(SAMPLE_RATE as f32),
                    sink_id: "".into(),
                    render_size_hint: web_audio_api::context::AudioContextRenderSizeCategory::Default,
                });
            }
        }

        setup_worklet(&ctx);

        let time_start = ctx.current_time();
        Self {
            ctx,
            time_start: time_start,
        }
    }
}


// desktop worklet

static CURRENT_SOUND_FN: Lazy<Mutex<SoundFn>> = Lazy::new(|| {
    Mutex::new(Box::new(|_| 0.0))
});

fn setup_worklet(context: &AudioContext){
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
struct MyProcessor{
    sample_idx: u64,
}

impl AudioProcessor for MyProcessor {
    fn process(
        &mut self,
        _inputs: &[AudioRenderQuantum],
        outputs: &mut [AudioRenderQuantum],
        params: AudioParamValues,
        _scope: &RenderScope,
    ) -> bool {
        let output = &mut outputs[0];
        output.set_number_of_channels(1);
        self.sample_idx += 128;
        let sound_fn_guard = CURRENT_SOUND_FN.lock().unwrap();
        let sound_fn = sound_fn_guard.as_ref();
        output.channels_mut().iter_mut().for_each(|buf| {
            buf.iter_mut()
            .enumerate()
                .for_each(|(i,output_sample)| {
                    *output_sample = sound_fn(((self.sample_idx + i as u64) as f32 * INV_SAMPLE_RATE));
                })
        });

        true 
    }
}
