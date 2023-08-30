## Run the project

Before running anything, make sure you have installed `clippy` and `fmt`:

```sh
rustup component add clippy-preview --toolchain=nightly
rustup component add rustfmt-preview --toolchain nightly
```

Run:

```sh
cargo run
```

## Running clippy

`cargo +nightly clippy --all -- -D clippy::pedantic`
