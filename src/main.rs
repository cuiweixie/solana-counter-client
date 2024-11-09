use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};

fn main() {
    let rpc_url = "http://127.0.0.1:8899";
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    // Path to the keypair.json file
    let path = Path::new("/Users/xiecui/.config/solana/id.json");

    // Read the keypair.json file
    let mut file = File::open(&path).expect("Failed to open keypair.json file");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Failed to read keypair.json file");

    // Deserialize the JSON data to a Keypair
    let keypair_vec: Vec<u8> = serde_json::from_str(&data).expect("Failed to deserialize data");
    let payer: Keypair = Keypair::from_bytes(&*keypair_vec).expect("Failed to deserialize keypair");


    let program_id = Pubkey::from_str("65Gb6MS5fcbc38vY9c8PHfCi1BxUjspusU4S1cVFi1Bc").unwrap();
    let counter_pubkey = Pubkey::from_str("4CjMGZKjRpvtyBHki7dfNGprXxU39xXKBFyRBqaQgND6").unwrap();

    println!("Counter public key: {}", counter_pubkey.to_string());

    let increment_by_n_ix = solana_sdk::instruction::Instruction::new_with_bincode(
        program_id,
        &[1, 5], // Instruction data: 1 for incrementByN, 5 is the increment value
        vec![solana_sdk::instruction::AccountMeta::new(counter_pubkey, false)],
    );

    let recent_blockhash = client.get_latest_blockhash().unwrap();
    let tx = Transaction::new_signed_with_payer(
        &[increment_by_n_ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    let signature = client.send_and_confirm_transaction(&tx).unwrap();
    println!("Transaction signature: {}", signature);
}