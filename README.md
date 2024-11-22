# A Rust warm-build benchmark

This repository is just intended as a benchmark for warm build times of Rust code with a substantial
number of large dependencies.

To run the benchmark, run:

```sh
./run-benchmark
```

The accompanying blog post is [Speeding up Rust edit-build-run
cycle](https://davidlattimore.github.io/working-on-rust-iteration-time.html)

## Pre-requisites

In order to run this benchmark suite you will need to be able to build and run `wild` 
(see [Wild's README.md](https://github.com/davidlattimore/wild)) plus these installed:

* `hyperfine` - install on linux with `sudo apt install hyperfine`
* `x86_64-unknown-linux-musl target` - install with `rustup target add x86_64-unknown-linux-musl`
* `mold` linker - install on linux with `sudo apt install mold`