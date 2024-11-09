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
    system_program,
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
    let counter_keypair = Keypair::new();
    let counter_pubkey = counter_keypair.pubkey();

    // Create the counter account
    let create_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &counter_pubkey,
        1_000_000, // lamports
        1, // space for the counter (1 byte)
        &program_id,
    );

    let recent_blockhash = client.get_latest_blockhash().unwrap();
    let tx = Transaction::new_signed_with_payer(
        &[create_account_ix],
        Some(&payer.pubkey()),
        &[&payer, &counter_keypair],
        recent_blockhash,
    );

    let signature = client.send_and_confirm_transaction(&tx).unwrap();
    println!("Transaction signature: {}", signature);
    println!("Counter account public key: {}", counter_pubkey);
}