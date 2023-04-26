// https://github.com/rustwasm/wasm-bindgen/blob/main/examples/wasm-audio-worklet/src/worklet.js
registerProcessor("WasmProcessor", class WasmProcessor extends AudioWorkletProcessor {
  constructor(options) {
    super();
    let [module, memory, handle] = options.processorOptions;
    bindgen.initSync(module, memory);
    this.processor = bindgen.WasmAudioProcessor.unpack(handle);
    console.log("created")
  }
  process(inputs, outputs) {
    return this.processor.process(outputs[0][0]);
  }
});