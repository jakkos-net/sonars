pub struct SoundResources {
    ctx: Option<AudioContext>,
}

impl Default for SoundResources {
    fn default() -> Self {
        let ctx = get_ctx();
        Self { ctx }
    }
}

fn get_ctx() -> Option<AudioContext> {
    // todo_major: actually return the audio context
    async fn inner() {
        web_main().await.unwrap();
    }
    spawn_local(inner());
    None
}

// editted from the wasm_bindgen audio worklet example: https://github.com/rustwasm/wasm-bindgen/tree/c5b073ae58cb3b6d44252108ea9862bf0d04f3b6/examples/wasm-audio-worklet

use js_sys::Array;
use js_sys::JsString;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_futures::JsFuture;
use web_sys::AudioContextOptions;
use web_sys::{AudioContext, AudioWorkletNode, AudioWorkletNodeOptions};

use crate::sound::CURRENT_SOUND_FN;

#[wasm_bindgen]
pub async fn web_main() -> Result<AudioContext, JsValue> {
    wasm_audio().await
}

#[wasm_bindgen]
pub struct WasmAudioProcessor(Box<dyn FnMut(&mut [f32]) -> bool>);

#[wasm_bindgen]
impl WasmAudioProcessor {
    pub fn process(&mut self, buf: &mut [f32]) -> bool {
        self.0(buf)
    }
    pub fn pack(self) -> usize {
        Box::into_raw(Box::new(self)) as usize
    }
    pub unsafe fn unpack(val: usize) -> Self {
        *Box::from_raw(val as *mut _)
    }
}

pub async fn wasm_audio() -> Result<AudioContext, JsValue> {
    let mut options = AudioContextOptions::new();
    options.sample_rate(SAMPLE_RATE as f32);
    let ctx = AudioContext::new_with_context_options(&options)?;
    prepare_wasm_audio(&ctx).await?;
    let process = make_process_function();
    let node = wasm_audio_node(&ctx, process)?;
    node.connect_with_audio_node(&ctx.destination()).unwrap();
    Ok(ctx)
}

fn make_process_function() -> Box<dyn FnMut(&mut [f32]) -> bool> {
    let mut idx: usize = 0;

    Box::new(move |buf: &mut [f32]| {
        let sound_fn_guard = CURRENT_SOUND_FN.lock().unwrap();
        let sound_fn = sound_fn_guard.as_ref();
        for (i, f) in buf.iter_mut().enumerate() {
            let t = (idx + i) as f32 / SAMPLE_RATE as f32;
            *f = sound_fn(t);
        }
        idx += buf.len();
        true
    })
}

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

use web_sys::{Blob, BlobPropertyBag, Url};

use super::SAMPLE_RATE;

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
