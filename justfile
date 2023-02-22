run:
    cargo run -j6 --profile mid

web:
    cargo run -j-1 --release --target wasm32-unknown-unknown