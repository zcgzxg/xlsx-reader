cargo +nightly build --release

wasm-opt -O3 \
    --enable-nontrapping-float-to-int \
    --enable-bulk-memory \
    --enable-simd \
    --strip-debug \
    --strip-producers \
    target/wasm32-unknown-unknown/release/xlsx_reader.wasm \
    -o target/wasm32-unknown-unknown/release/xlsx_reader.wasm

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
