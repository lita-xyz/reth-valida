#![no_main]

use alloy_primitives::B256;
use revm::InMemoryDB;
use reth_valida::primitives::db::InMemoryDBHelper;
use reth_valida::primitives::mpt::keccak;
use reth_valida::primitives::processor::EvmProcessor;
use reth_valida::primitives::ValidaRethInput;

valida_rs::entrypoint!(main);

pub fn main() {
    let mut input = match valida_rs::io::read_and_deserialize::<ValidaRethInput>() {
        Ok(val) => val,
        Err(e) => {
            valida_rs::io::println(&format!("Error reading/deserializing input: {}", e));
            return;
        }
    };
    // Initialize the database.
    let db = match InMemoryDB::initialize(&mut input) {
        Ok(val) => val,
        Err(e) => {
            valida_rs::io::println(&format!("Error initializing database: {}", e));
            return;
        }
    };

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
    let header = match executor.header {
        Some(val) => val,
        None => {
            valida_rs::io::println("Error: executor header is None");
            return;
        }
    };
    let hash = B256::from(keccak(alloy_rlp::encode(header)));
    if let Err(e) = valida_rs::io::write(&hash) {
        valida_rs::io::println(&format!("Error writing hash: {}", e));
    }
}
