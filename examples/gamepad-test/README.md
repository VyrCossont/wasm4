# gamepad-test

A gamepad tester written in Rust for the [WASM-4](https://wasm4.org) fantasy console.

## Building

Build the cart by running:

```shell
cargo build --release
```

## Running

Then run it with:

```shell
w4 run target/wasm32-unknown-unknown/release/gamepad_test.wasm
```

Press gamepad buttons for any one of the 4 players, and you should see the corresponding on-screen buttons light up.
