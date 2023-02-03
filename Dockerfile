FROM scratch

COPY target/wasm32-wasi/release/hello_world.wasm ./plugin.wasm