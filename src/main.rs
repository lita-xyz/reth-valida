#![no_main]

use reth_primitives::B256;
use revm::InMemoryDB;
use reth_valida::primitives::ValidaRethInput;

entrypoint::entrypoint!(main);

pub fn main() {
    println!("reading input");
    let mut input = entrypoint::io::read_n(1).unwrap();
    println!("read input: {:?}", input);
}
