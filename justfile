build:
  cargo build --all

run MODE:
  cargo run --bin={{MODE}}

host MODE:
    cargo build --bin={{MODE}} --target wasm32-unknown-unknown
    wasm-bindgen --out-dir public/ --target web target/wasm32-unknown-unknown/debug/{{MODE}}.wasm
    basic-http-server