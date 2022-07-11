# Hi

This is a personal template repository for quickly spinning up a project with libraries I use often (bevy, kira, egui) and building for both native and WASM.

Once you clone this template, remember to replace-all "renameme" to whatever you want your project to be called, as well as renaming the "renameme" folder.

# useful commands
- install wasm deps: rustup target add wasm32-unknown-unknown
- install local wasm runner: cargo install wasm-server-runner
- run local wasm: cargo run --target wasm32-unknown-unknown
- install lld for faster compiling linux: sudo apt-get install lld
- install lld for faster compiling windows: cargo install -f cargo-binutils; rustup component add llvm-tools-preview
- install other deps linux: sudo apt install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev
- install github web builder: cargo install wasm-bindgen
- refresh github web builder: cargo build --release --target wasm32-unknown-unknown; wasm-bindgen --out-dir ./docs/ --target web ./target/wasm32-unknown-unknown/release/renameme.wasm
