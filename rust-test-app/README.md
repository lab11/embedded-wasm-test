Example Rust Application
========================

A very simple rust application build with no\_std. The cargo config sets
the default target to "wasm32-unknown-unknown"

We can't use std because we are currently compiling to WASM with no system interface (which std depends on heavily).

To build just run:

```
$ cargo build --release
``` 
