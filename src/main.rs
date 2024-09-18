#![no_main]

use reth_primitives::B256;
use revm::InMemoryDB;
use reth_valida::primitives::db::InMemoryDBHelper;
use reth_valida::primitives::mpt::keccak;
use reth_valida::primitives::processor::EvmProcessor;
use reth_valida::primitives::ValidaRethInput;

use clap::Parser;
// use Valida_prover::{ValidaProver, ValidaStdin};
// use reth_valida::primitives::ValidaRethInput;
use reth_valida::utils::init::ValidaRethInputInitializer;
// use reth_valida::utils::db::RemoteDb;
use std::fs::File;

entrypoint::entrypoint!(main);

/// The CLI arguments for the Valida Reth program.
#[derive(Parser, Debug)]
pub struct ValidaRethArgs {
    #[arg(short, long)]
    rpc_url: String,

    #[arg(short, long)]
    block_number: u64,

    #[arg(short, long)]
    use_cache: bool,
}

#[tokio::main]
async fn main() {
    // Parse arguments.
    let args = ValidaRethArgs::parse();

    // Get input.
    let input: ValidaRethInput = if !args.use_cache {
        let input = ValidaRethInput::initialize(&args.rpc_url, args.block_number)
            .await
            .unwrap();
        let mut file =
            File::create(format!("{}.bin", args.block_number)).expect("unable to open file");
        bincode::serialize_into(&mut file, &input).expect("unable to serialize input");
        input
    } else {
        let file = File::open(format!("{}.bin", args.block_number)).expect("unable to open file");
        bincode::deserialize_from(file).expect("unable to deserialize input")
    };

    // let mut input = entrypoint::io::read::<ValidaRethInput>().unwrap();

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
    entrypoint::io::write(&hash).unwrap();
}
