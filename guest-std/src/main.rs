use covenant_lib::{check_withdraw, execute_test_suite, read_suite};

extern crate libc;

use std::fs::File;
use std::io::Read;

extern crate alloc;
use alloc::string::ToString;
use alloc::vec::Vec;

pub fn main() {
    // all private inputs
    // size: 32bytes
    let goat_withdraw_txid: Vec<u8> =
        hex::decode(std::env::var("GOAT_WITHDRAW_TXID").unwrap_or("32bc8a6c5b3649f92812c461083bab5e8f3fe4516d792bb9a67054ba040b7988".to_string())).unwrap();
    //assert!(goat_withdraw_txid.len() == 32);
    // size: 20bytes
    let withdraw_contract_address: Vec<u8> =
        hex::decode(std::env::var("WITHDRAW_CONTRACT_ADDRESS").unwrap_or("86a77bdfcaff7435e1f1df06a95304d35b112ba8".to_string()))
            .unwrap();
    //assert!(withdraw_contract_address.len() == 20);
    // size: 20bytes
    //let operator_address: Vec<u8> =
    //    hex::decode(std::env::var("OPERATOR_ADDRESS").unwrap_or("86a77bdfcaff7435e1f1df06a95304d35b112ba8".to_string())).unwrap();
    //assert!(operator_address.len() == 20);

    let withdraw_map_base_key = 
        hex::decode(std::env::var("WITHDRAW_MAP_BASE_KEY").unwrap_or("32bc8a6c5b3649f92812c461083bab5e8f3fe4516d792bb9a67054ba040b7988".to_string())).unwrap();
    let withdraw_map_index = 
        hex::decode(std::env::var("WITHDRAW_MAP_INDEX").unwrap_or("32bc8a6c5b3649f92812c461083bab5e8f3fe4516d792bb9a67054ba040b7988".to_string())).unwrap();
    let peg_in_txid: Vec<u8> =
        hex::decode(std::env::var("PEG_IN_TXID").unwrap_or("32bc8a6c5b3649f92812c461083bab5e8f3fe4516d792bb9a67054ba040b7988".to_string())).unwrap();

    //let tx_list: Vec<u8> = zkm2_zkvm::io::read();
    let manifest_path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let json_path = std::env::var("JSON_PATH")
        .unwrap_or(format!("{}/../test-vectors/3168249.json", manifest_path));
    let mut f = File::open(json_path).unwrap();
    let mut tx_list = vec![];
    f.read_to_end(&mut tx_list).unwrap();

    let suite = read_suite(&tx_list);

    assert!(check_withdraw(
        &withdraw_contract_address,
        &goat_withdraw_txid,
        &withdraw_map_base_key,
        &withdraw_map_index,
        &peg_in_txid,
        &suite
    )
    .is_ok());
    assert!(execute_test_suite(suite).is_ok());
    println!("finish");

    // public inputs
    //zkm2_zkvm::io::commit(&goat_withdraw_txid);
    //zkm2_zkvm::io::commit(&withdraw_contract_address);
}
