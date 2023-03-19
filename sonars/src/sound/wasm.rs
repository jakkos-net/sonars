const SAMPLE_TICK: usize = 128;

pub struct SoundResources {
    ctx: Option<AudioContext>,
}

impl Default for SoundResources {
    fn default() -> Self {
        spawn_local(wasm_audio());

        Self { ctx: None }
    }
}
// https://github.com/rustwasm/wasm-bindgen/blob/main/examples/wasm-audio-worklet/src/wasm_audio.rs

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{AudioContext, AudioWorkletNode, AudioWorkletNodeOptions};

use crate::sound::CURRENT_SOUND_FN;

//https://github.com/rustwasm/wasm-bindgen/blob/main/examples/wasm-audio-worklet/src/dependent_module.rs
#[wasm_bindgen]
pub struct WasmAudioProcessor;

#[wasm_bindgen]
impl WasmAudioProcessor {
    pub fn process(&mut self, buf: &mut [f32]) -> bool {
        // self.sample_idx += SAMPLE_TICK;
        // let idx = self.sample_idx;
        // let func = CURRENT_SOUND_FN.lock().unwrap();
        // for (i, v) in buf.iter_mut().enumerate() {
        //     *v = func((idx + i) as f32);
        // }
        true
    }
    pub fn pack(self) -> usize {
        Box::into_raw(Box::new(self)) as usize
    }
    pub unsafe fn unpack(val: usize) -> Self {
        *Box::from_raw(val as *mut _)
    }
}

// Use wasm_audio if you have a single wasm audio processor in your application
// whose samples should be played directly. Ideally, call wasm_audio based on
// user interaction. Otherwise, resume the context on user interaction, so
// playback starts reliably on all browsers.
pub async fn wasm_audio() {
    let ctx = AudioContext::new().unwrap();
    prepare_wasm_audio(&ctx).await.unwrap();
    let node = wasm_audio_node(&ctx).unwrap();
    node.connect_with_audio_node(&ctx.destination()).unwrap();
}

// wasm_audio_node creates an AudioWorkletNode running a wasm audio processor.
// Remember to call prepare_wasm_audio once on your context before calling
// this function.
pub fn wasm_audio_node(ctx: &AudioContext) -> Result<AudioWorkletNode, JsValue> {
    AudioWorkletNode::new_with_options(
        &ctx,
        "WasmProcessor",
        &AudioWorkletNodeOptions::new().processor_options(Some(&js_sys::Array::of3(
            &wasm_bindgen::module(),
            &wasm_bindgen::memory(),
            &WasmAudioProcessor {}.pack().into(),
        ))),
    )
}

pub async fn prepare_wasm_audio(ctx: &AudioContext) -> Result<(), JsValue> {
    JsFuture::from(ctx.audio_worklet()?.add_module("docs/worklet.js")?).await?;
    Ok(())
}
