run EXAMPLE_NAME:
    cargo run --release --example {{EXAMPLE_NAME}}

build EXAMPLE_NAME:
    cargo build --release --example {{EXAMPLE_NAME}}

serve EXAMPLE_NAME: (build-web EXAMPLE_NAME)
    miniserve --index index.html examples/wasm

build-web EXAMPLE_NAME:
    cargo build --release --example {{EXAMPLE_NAME}} --target wasm32-unknown-unknown
    wasm-bindgen --target web --no-typescript --out-dir examples/wasm/target --out-name wasm_example \
        target/wasm32-unknown-unknown/release/examples/{{EXAMPLE_NAME}}.wasm
