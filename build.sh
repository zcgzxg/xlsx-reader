cargo build --release

wasm-bindgen \
    target/wasm32-unknown-unknown/release/xlsx_reader.wasm \
    --out-dir ./web/src/xlsx-reader \
    --out-name index \
    --target bundler \
    --reference-types \
    --weak-refs

static-compress \
    -c brotli -q 11 \
    ./web/src/xlsx-reader/*.wasm

ls -lah ./web/src/xlsx-reader
