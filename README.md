This crate is a modified version of [`reth`](https://github.com/paradigmxyz/reth), to be run on the [Valida](https://github.com/lita-xyz/valida-toolchain) zkVM.

It is to be used in conjunction with [`prepare_block`](https://github.com/marty-ai/prepare_block). It reads a specific serialized block prepared by `prepare_block`, and prove the block on the Ethereum engine, using Valida.

## Compilation
To compile this to run on the Valida zkVM, make sure you have the [Valida Compiler](https://github.com/lita-xyz/valida-toolchain) installed. Follow its README to install. Make sure there is no error in the compilation process.

After the Valida compiler is successfully installed, run the following command in the `reth-valida` directory:

```bash
cargo +valida build --release
```

This is in active development. If you get compilation errors, run `git pull`, `cargo clean`, `cargo update`and try again.

If you get the following error:

```bash
error: toolchain 'valida' is not installable
```

Run [this script](https://github.com/lita-xyz/valida-toolchain/blob/main/install-rust) from the Valida repo to add the Valida compiler to `rustc`.

## Proving a block
After successful compilation, the binary will be in `target/delendum-unknown-baremetal-gnu/release/reth-valida`. One can prove a block by running the binary in Valida, with the block data input `input.bin` (obtained from running `prepare_block`, more details to follow). To do this, run this command in the `valida-toolchain` directory:

```bash
./valida/target/release/valida run ../reth-valida/target/delendum-unknown-baremetal-gnu/release/reth-valida log ../prepare_block/input.bin
```

Alternatively, run the built binary in a `valida-shell` with the block data input file:

```bash
>valida run <path to reth-valida binary> log <path to input.bin>
```

## Preparing block data
To prepare the block data, clone the [prepare_block](https://github.com/lita-xyz/prepare_block) repository and run the following command (see also its [README](https://github.com/lita-xyz/prepare_block/blob/main/README.md)):

Note that `prepare_block` is not intended to be compiled with the Valida compiler because it has features that are not supported by the Valida compiler. Use the standard Rust toolchain to compile and run `prepare_block`.

```bash
cargo run -- --rpc-url "<RPC URL>" --block-number <block number>
```

The output will be in the current directory as `input.bin`.

This is in active development. If you get compilation errors, run `git pull`, `cargo clean`, `cargo update`and try again.