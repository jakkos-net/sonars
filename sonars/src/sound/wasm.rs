const SAMPLE_TICK: usize = 128;

pub struct SoundResources {
    ctx: Option<AudioContext>,
}

impl Default for SoundResources {
    fn default() -> Self {
        spawn_local(web_main());
        Self { ctx: None }
    }
}
// https://github.com/rustwasm/wasm-bindgen/blob/main/examples/wasm-audio-worklet/src/wasm_audio.rs

use js_sys::Array;
use js_sys::JsString;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_futures::JsFuture;
use web_sys::{AudioContext, AudioWorkletNode, AudioWorkletNodeOptions};

use crate::sound::CURRENT_SOUND_FN;

// https://github.com/rustwasm/wasm-bindgen/blob/153a6aa9c7a989c1865df7f93b2ddbca1113a175/examples/wasm-audio-worklet/src/lib.rs#L12
#[wasm_bindgen]
pub async fn web_main() {
    wasm_audio().await.unwrap();
}

//https://github.com/rustwasm/wasm-bindgen/blob/main/examples/wasm-audio-worklet/src/dependent_module.rs
#[wasm_bindgen]
pub struct WasmAudioProcessor(Box<dyn FnMut(&mut [f32]) -> bool>);

#[wasm_bindgen]
impl WasmAudioProcessor {
    pub fn process(&mut self, buf: &mut [f32]) -> bool {
        // self.sample_idx += SAMPLE_TICK;
        // let idx = self.sample_idx;
        // let func = CURRENT_SOUND_FN.lock().unwrap();
        // for (i, v) in buf.iter_mut().enumerate() {
        //     *v = func((idx + i) as f32);
        // }
        self.0(buf)
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
pub async fn wasm_audio() -> Result<AudioContext, JsValue> {
    let ctx = AudioContext::new()?;
    prepare_wasm_audio(&ctx).await?;

    let process = Box::new(|buf: &mut [f32]| {
        for (i, f) in buf.iter_mut().enumerate() {
            *f = (((i as f32) / 44_000.0) * 2.0 * 3.141 * 440.0).sin();
        }
        true
    });

    let node = wasm_audio_node(&ctx, process)?;
    node.connect_with_audio_node(&ctx.destination()).unwrap();
    Ok(ctx)
}

// wasm_audio_node creates an AudioWorkletNode running a wasm audio processor.
// Remember to call prepare_wasm_audio once on your context before calling
// this function.
pub fn wasm_audio_node(
    ctx: &AudioContext,
    process: Box<dyn FnMut(&mut [f32]) -> bool>,
) -> Result<AudioWorkletNode, JsValue> {
    AudioWorkletNode::new_with_options(
        &ctx,
        "WasmProcessor",
        &AudioWorkletNodeOptions::new().processor_options(Some(&js_sys::Array::of3(
            &wasm_bindgen::module(),
            &wasm_bindgen::memory(),
            &WasmAudioProcessor(process).pack().into(),
        ))),
    )
}

pub async fn prepare_wasm_audio(ctx: &AudioContext) -> Result<(), JsValue> {
    let mod_url = on_the_fly(include_str!("worklet.js"))?;
    JsFuture::from(ctx.audio_worklet()?.add_module(&mod_url)?).await?;
    Ok(())
}

// https://github.com/rustwasm/wasm-bindgen/blob/153a6aa9c7a989c1865df7f93b2ddbca1113a175/examples/wasm-audio-worklet/src/dependent_module.rsuse js_sys::{Array, JsString};
use web_sys::{Blob, BlobPropertyBag, Url};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    type ImportMeta;

    #[wasm_bindgen(method, getter)]
    fn url(this: &ImportMeta) -> JsString;

    #[wasm_bindgen(js_namespace = import, js_name = meta)]
    static IMPORT_META: ImportMeta;
}

pub fn on_the_fly(code: &str) -> Result<String, JsValue> {
    // Generate the import of the bindgen ES module, assuming `--target web`,
    // preluded by the TextEncoder/TextDecoder polyfill needed inside worklets.
    let header = format!(
        "import '{}';\n\
        import init, * as bindgen from '{}';\n\n",
        wasm_bindgen::link_to!(module = "/src/sound/polyfill.js"),
        IMPORT_META.url(),
    );

    Url::create_object_url_with_blob(&Blob::new_with_str_sequence_and_options(
        &Array::of2(&JsValue::from(header.as_str()), &JsValue::from(code)),
        &BlobPropertyBag::new().type_("text/javascript"),
    )?)
}
