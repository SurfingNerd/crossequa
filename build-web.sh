cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-name crossequa \
   --out-dir web/target/wasm \
   --target web target/wasm32-unknown-unknown/release/crossequa.wasm
   
#cp target/wasm32-unknown-unknown/release/crossequa.wasm web/target/
basic-http-server web
