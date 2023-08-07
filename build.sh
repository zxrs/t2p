source ../../repo/emsdk/emsdk_env.sh

export EMCC_CFLAGS="-O3"
cargo build --release --package libtiff-sys --target wasm32-unknown-emscripten

export EMCC_CFLAGS="-O3 -o t2p.js -s ERROR_ON_UNDEFINED_SYMBOLS=0 -s EXPORTED_FUNCTIONS=['_generate_pdf','_malloc','_free'] -s ALLOW_MEMORY_GROWTH=1 -s EXPORTED_RUNTIME_METHODS=ccall"
cargo build --release --target wasm32-unknown-emscripten