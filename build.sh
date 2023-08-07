source ../../repo/emsdk/emsdk_env.sh

export EMCC_CFLAGS="-O3"
cargo build --release --package libtiff-sys --target wasm32-unknown-emscripten

export EMCC_CFLAGS="-O3 -o t2p.js -s EXPORTED_FUNCTIONS=['_generate_pdf','_buf_len','_free_buf','_malloc','_free'] -s ALLOW_MEMORY_GROWTH=1 -s EXPORTED_RUNTIME_METHODS=ccall"
cargo build --release --target wasm32-unknown-emscripten