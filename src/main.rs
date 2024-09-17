#![no_main]

use reth_valida::primitives::mpt;
use reth_primitives::B256;
use revm::InMemoryDB;
use reth_valida::primitives::db::InMemoryDBHelper;
use reth_valida::primitives::mpt::keccak;
use reth_valida::primitives::processor::EvmProcessor;
use reth_valida::primitives::ValidaRethInput;

entrypoint::entrypoint!(main);

pub fn main() {
    let mut input = entrypoint::io::read::<ValidaRethInput>().unwrap();

    // Initialize the database.
    let db = InMemoryDB::initialize(&mut input).unwrap();

    // Execute the block.
    let mut executor = EvmProcessor::<InMemoryDB> {
        input,
        db: Some(db),
        header: None,
    };
    executor.initialize();
    executor.execute();
    executor.finalize();

    // Print the resulting block hash.
    let hash = B256::from(keccak(alloy_rlp::encode(executor.header.unwrap())));
}
