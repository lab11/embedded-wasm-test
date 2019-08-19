Embedded WASM Test
=================

The goal of this repo is to see if we can compile some source code to WASM, then
WASM to a native binary than can run on an MCU (ideally ARM Cortex-M, but RISC-V
would also be acceptable). This is primarily to test ahead of time compilation.

What you will need:
 - Cranelift built and added to your path (https://github.com/CraneStation/cranelift)
 - WASM binary tools build and added to your path (https://github.com/WebAssembly/wabt)
 - Rust nightly
