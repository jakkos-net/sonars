// Taken from the wasm_bindgen audio worklet example: https://github.com/rustwasm/wasm-bindgen/tree/c5b073ae58cb3b6d44252108ea9862bf0d04f3b6/examples/wasm-audio-worklet 
registerProcessor("WasmProcessor", class WasmProcessor extends AudioWorkletProcessor {
  constructor(options) {
    super();
    let [module, memory, handle] = options.processorOptions;
    bindgen.initSync(module, memory);
    this.processor = bindgen.WasmAudioProcessor.unpack(handle);
  }
  process(inputs, outputs) {
    return this.processor.process(outputs[0][0]);
  }
});