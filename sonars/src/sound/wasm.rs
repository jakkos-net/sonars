// https://github.com/rustwasm/wasm-bindgen/blob/main/examples/wasm-audio-worklet/src/wasm_audio.rs

use crate::dependent_module;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{AudioContext, AudioWorkletNode, AudioWorkletNodeOptions};

#[wasm_bindgen]
pub struct WasmAudioProcessor;

#[wasm_bindgen]
impl WasmAudioProcessor {
    pub fn process(&mut self, buf: &mut [f32]) -> bool {
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
pub async fn wasm_audio(
    process: Box<dyn FnMut(&mut [f32]) -> bool>,
) -> Result<AudioContext, JsValue> {
    let ctx = AudioContext::new()?;
    prepare_wasm_audio(&ctx).await?;
    let node = wasm_audio_node(&ctx, process)?;
    node.connect_with_audio_node(&ctx.destination())?;
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
    let mod_url = dependent_module!("worklet.js")?;
    JsFuture::from(ctx.audio_worklet()?.add_module(&mod_url)?).await?;
    Ok(())
}

//https://github.com/rustwasm/wasm-bindgen/blob/main/examples/wasm-audio-worklet/src/dependent_module.rs
use js_sys::{Array, JsString};
use wasm_bindgen::prelude::*;
use web_sys::{Blob, BlobPropertyBag, Url};

// This is a not-so-clean approach to get the current bindgen ES module URL
// in Rust. This will fail at run time on bindgen targets not using ES modules.
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
    // Generate the import of the bindgen ES module, assuming `--target web`.
    let header = format!(
        "import init, * as bindgen from '{}';\n\n",
        IMPORT_META.url(),
    );

    Url::create_object_url_with_blob(&Blob::new_with_str_sequence_and_options(
        &Array::of2(&JsValue::from(header.as_str()), &JsValue::from(code)),
        &BlobPropertyBag::new().type_("text/javascript"),
    )?)
}

// dependent_module! takes a local file name to a JS module as input and
// returns a URL to a slightly modified module in run time. This modified module
// has an additional import statement in the header that imports the current
// bindgen JS module under the `bindgen` alias, and the separate init function.
// How this URL is produced does not matter for the macro user. on_the_fly
// creates a blob URL in run time. A better, more sophisticated solution
// would add wasm_bindgen support to put such a module in pkg/ during build time
// and return a URL to this file instead (described in #3019).
#[macro_export]
macro_rules! dependent_module {
    ($file_name:expr) => {
        crate::dependent_module::on_the_fly(include_str!($file_name))
    };
}
