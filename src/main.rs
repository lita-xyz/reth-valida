#![no_main]

use reth_primitives::B256;
use revm::InMemoryDB;
use reth_valida::primitives::ValidaRethInput;

entrypoint::entrypoint!(main);

pub fn main() {
    let mut input = entrypoint::io::read::<ValidaRethInput>().unwrap();

}
