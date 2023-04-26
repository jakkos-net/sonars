run:
    cargo run -j6 --profile mid

web:
    cargo run -j-1 --release --target wasm32-unknown-unknown -Zbuild-std=std,panic_abort

web_build:
    cargo build --release --target wasm32-unknown-unknown -Zbuild-std=std,panic_abort
    wasm-bindgen --out-dir ./docs/ --target web ./target/wasm32-unknown-unknown/release/sonars.wasm --split-linked-modules