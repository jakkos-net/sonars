use bevy::prelude::{NonSendMut, Plugin, ResMut, Resource};

use crossbeam_queue::SegQueue;
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
    mut sound_resources: NonSendMut<SoundResources>,
) {
    sound_resources.update(&sound_control.next_fn).unwrap();
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

struct SoundResources {
    pub ctx: AudioContext,
    pub gain: GainNode,
    pub buf: Option<AudioBufferSourceNode>,
    pub time_start: f64,
    pub next_buf: Option<AudioBufferSourceNode>,
}

impl Default for SoundResources {
    fn default() -> Self {
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

        let time_start = ctx.current_time();
        Self {
            ctx,
            gain,
            buf: None,
            next_buf: None,
            time_start: time_start,
        }
    }
}

const GAIN: f32 = 0.2;

impl SoundResources {
    pub fn update(&mut self, sound_fn: &SoundFn) -> anyhow::Result<()> {
        // have we been playing the current buffer for too long?
        let t = self.ctx.current_time();
        if (t - self.time_start) > BUFFER_TIME_LENGTH {
            cfg_if::cfg_if! {
                if #[cfg(target_arch = "wasm32")] {
                    let new_buf = self.ctx.create_buffer_source().unwrap();
                    new_buf.set_loop(true);
                    new_buf.connect_with_audio_node(&self.gain).unwrap();
                    // new_buf.start().unwrap();
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

                    // CURRENT SOUND
                    // #############

                    // if we were playing sound before, stop
                    if self.buf.is_some() {
                        let old_buf = self.buf.take().unwrap();
                        old_buf.stop();
                        old_buf.disconnect();
                        drop(old_buf);
                    }

                    // FOR NEXT TIME
                    // #############

                    // what time will the next buffer start playing
                    let next_t = self.time_start + BUFFER_TIME_LENGTH;
                    self.time_start = next_t;
                    let next_t_f32 = next_t as f32;
                    // generate the data for the next buffer
                    let data = (0..BUFFER_SIZE)
                    .into_iter()
                    .map(|x| (sound_fn)((x as f32) * INV_SAMPLE_RATE + next_t_f32))
                    .collect::<Vec<_>>();

                    // create the buffer_node
                    let new_buf = self.ctx.create_buffer_source();
                    // connect it to the sound ouput
                    new_buf.connect(&self.gain);

                    // create the buffer
                    let mut buf_data = self.ctx
                        .create_buffer(1, BUFFER_SIZE as usize, SAMPLE_RATE as f32);
                    // copy the data into the buffer
                    // todo_major why do we have to make a vec, then copy it? Seems inefficient
                    buf_data.copy_to_channel(data.as_slice(), 0);
                    // put the buffer in the buffer node
                    new_buf.set_buffer(buf_data);

                    new_buf.start_at(next_t);


                    // move the now playing buffer into buf, and the for-next-time buffer into next_buf
                    self.buf = self.next_buf.take();
                    self.next_buf = Some(new_buf);
                }
            }
        }
        Ok(())
    }
}


// https://github.com/reprimande/wasm-audioworklet-synth/


#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut f32 {
    let mut buf = Vec::<f32>::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr as *mut f32
}


#[no_mangle]
pub extern "C" fn process(out_ptr: *mut f32, size: usize) {
    // let mut synth = SYNTH.lock().unwrap();
    // synth.process(out_ptr, size);
}
