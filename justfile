run:
    cargo run -j6 --profile mid

web:
    cargo build --release --target wasm32-unknown-unknown -Zbuild-std=std,panic_abort
    wasm-bindgen --out-dir ./docs/ --target web ./target/wasm32-unknown-unknown/release/sonars.wasm --split-linked-modules
    (cd docs && ./server.py)