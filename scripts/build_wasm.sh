cargo build --target wasm32-unknown-unknown --features webgl
cargo install -f wasm-bindgen-cli
cargo update

rm -rf ./public/wasm

wasm-bindgen \
    --target web \
    --out-dir ./public/wasm \
    ./target/wasm32-unknown-unknown/debug/reindeer.wasm

cd public

npm i
npm run build