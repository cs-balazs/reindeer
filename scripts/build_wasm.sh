cargo build --target wasm32-unknown-unknown --features webgl --release --example hello_world 
cargo install -f wasm-bindgen-cli

rm -rf ./public/wasm

wasm-bindgen \
    --target web \
    --out-dir ./public/wasm \
    ./target/wasm32-unknown-unknown/release/examples/hello_world.wasm

cd public

npm i
npm run build