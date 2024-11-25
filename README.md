# A Rust warm-build benchmark

This repository is just intended as a benchmark for warm build times of Rust code with a substantial
number of large dependencies.

To run the benchmark, run:

```sh
./run-benchmark
```

The accompanying blog post is [Speeding up Rust edit-build-run cycle](https://davidlattimore.github.io/working-on-rust-iteration-time.html).

## Pre-requisites

In order to run this benchmark suite you will need these installed:

* `hyperfine` - install on linux with `sudo apt install hyperfine` or on mac with `brew install hyperfine`
* `x86_64-unknown-linux-musl target` - install with `rustup target add x86_64-unknown-linux-musl`

## Benchmarking mold linker (default)
To be able to benchmark the build times using the `mold` linker, you will need:
* `mold` installed - install on linux with `sudo apt install mold`

## Benchmarking wild linker
To be able to benchmark the build times using the `wild` linker, you will need to:
* Build and run `wild` (see [Wild's README.md](https://github.com/davidlattimore/wild))
* Have it in your `PATH`
* Modify the `.cargo/config.toml` file to use `wild` as the linker instead of `mold`,
an example `.cargo/config.toml` file's contents would be:
```toml
[build]
target = "x86_64-unknown-linux-musl"

[target.x86_64-unknown-linux-musl]
linker = "/usr/bin/clang"
#rustflags = [ "-C", "relocation-model=static", "-C", "link-arg=--ld-path=mold" ]
rustflags = [ "-C", "relocation-model=static", "-C", "link-arg=--ld-path=wild" ]

[target.x86_64-unknown-linux-gnu]
linker = "/usr/bin/clang"
#rustflags = ["-C", "link-arg=--ld-path=mold"]
rustflags = ["-C", "link-arg=--ld-path=wild"]
```
