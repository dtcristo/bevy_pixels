run EXAMPLE_NAME:
    cargo run --release --package example_{{EXAMPLE_NAME}}

build EXAMPLE_NAME:
    cargo build --release --package example_{{EXAMPLE_NAME}}

serve-web EXAMPLE_NAME: (build-web EXAMPLE_NAME)
    miniserve --index index.html examples/public

build-web EXAMPLE_NAME:
    cargo build --release --package example_{{EXAMPLE_NAME}} --target wasm32-unknown-unknown
    wasm-bindgen --target web --no-typescript --out-dir examples/public/wasm --out-name example \
        target/wasm32-unknown-unknown/release/example_{{EXAMPLE_NAME}}.wasm
