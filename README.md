This crate is a modified version of [`reth`](https://github.com/paradigmxyz/reth), to be run on the [Valida](https://github.com/lita-xyz/valida-toolchain) zkVM.

It is to be used in conjunction with [`prepare_block`](https://github.com/marty-ai/prepare_block). It reads a specific serialized block prepared by `prepare_block`, and prove the block on the Ethereum engine, using Valida.

## Compilation
To compile this to run on the Valida zkVM, make sure you have the [Valida Compiler](https://github.com/lita-xyz/valida-toolchain) installed. Follow its README to install. Make sure there is no error in the compilation process.

After the Valida compiler is successfully installed, run the following command in the `reth-valida` directory:

```bash
CC_delendum_unknown_baremetal_gnu=/valida-toolchain/bin/clang CFLAGS_delendum_unknown_baremetal_gnu="--sysroot=/valida-toolchain/ -isystem /valida-toolchain/include" RUSTFLAGS="-C linker=/valida-toolchain/bin/ld.lld -C link-args=/valida-toolchain/DelendumEntryPoint.o -C link-args=--script=/valida-toolchain/valida.ld -C link-args=/valida-toolchain/lib/delendum-unknown-baremetal-gnu/libc.a -C link-args=/valida-toolchain/lib/delendum-unknown-baremetal-gnu/libm.a -C link-args=--noinhibit-exec" cargo +delendum build --target=delendum-unknown-baremetal-gnu --verbose
```

If you get the following error:

```bash
error: toolchain 'delendum' is not installable
```

Run [this script](https://github.com/lita-xyz/valida-toolchain/blob/main/install-rust) from the Valida repo to add the Valida compiler to `rustc`.

## Proving a block
After successful compilation, the binary will be in `target/delendum-unknown-baremetal-gnu/release/reth-valida`. One can prove a block by running the binary in Valida, with the block data input `input.bin` (obtained from running `prepare_block`, more details to follow). To do this, run the following commands in the `valida-toolchain` directory:

```bash
./valida/target/release/valida run ../reth-valida/target/delendum-unknown-baremetal-gnu/debug/reth-valida log ../prepare_block/input.bin
```

## Preparing block data
To prepare the block data, clone the [prepare_block](https://github.com/lita-xyz/prepare_block) repository and run the following command (see also its [README](https://github.com/lita-xyz/prepare_block/blob/main/README.md)):

```bash
cargo run -- --rpc-url "<RPC URL>" --block-number <block number>
```
