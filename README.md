Embedded WASM Test
=================

The goal of this repo is to see if we can compile some source code to WASM, then
WASM to a native binary than can run on an MCU (ideally ARM Cortex-M, but RISC-V
would also be acceptable). This is primarily to test ahead of time compilation.

What you will need:
 - Cranelift built and added to your path (https://github.com/CraneStation/cranelift)
 - WASM binary tools build and added to your path (https://github.com/WebAssembly/wabt)
 - Rust nightly

Steps:
 - Build the example rust program

```
$ cd rust-test-app
$ cargo build --release
```

 - Compile the resulting WASM to a target. Currently cranelift only supports X86_64 and riscv32.

```
$ clif-util wasm --target riscv32 -- target/wasm32-unknown-unknown/rust-test-app.wasm > rust-test-app.bin
``` 

or

```
$ clif-util wasm --target x86_64 -- target/wasm32-unknown-unknown/rust-test-app.wasm > rust-test-app.bin
``` 

Note that with no system interface these aren't very useful. There is a need to explore WASI (which implements
the proposed WASM system interface) and how it could be ported to run on MCUs. 

Even for AoT code there would also still need to be a runtime. So getting this actually working is still pretty
far away, but AoT compiler supports seems like a necessary first step. JIT would also work, but (at least in cranelift)
supporting JIT is the same as AoT. The WAMR runtime is a JIT written in C, which seems to work at least a bit but must
be horribly inefficient. I'm pretty skeptical about putting any time into it for this reason.
