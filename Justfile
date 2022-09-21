serve example_name: (build_web example_name)
    miniserve --index index.html examples/wasm

build_web example_name:
    cargo build --release --example {{example_name}} --target wasm32-unknown-unknown
    wasm-bindgen --target web --out-dir examples/wasm/target --out-name wasm_example \
        target/wasm32-unknown-unknown/release/examples/{{example_name}}.wasm
