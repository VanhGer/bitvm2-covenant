#![no_std]
#![no_main]

use revm::{
    db::CacheState,
    primitives::{calc_excess_blob_gas, Bytecode, Env, SpecId, TransactTo, keccak256},
    Evm,
    primitives::{b256, U256},
};

//use zkm2_zkvm::lib::hasher::Hasher;

extern crate libc;

//use models::*;
use covenant_lib::{
    recover_address,
    read_suite,
    check_withdraw,
    execute_test_suite,
};

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use alloc::boxed::Box;
use revm::primitives::Address;
zkm2_zkvm::entrypoint!(main);

pub fn main() {
    // all private inputs
    // size: 32bytes
    let goat_withdraw_txid: Vec<u8> = zkm2_zkvm::io::read(); 
    //assert!(goat_withdraw_txid.len() == 32);
    // size: 20bytes
    let withdraw_contract_address: Vec<u8> = zkm2_zkvm::io::read(); 
    //assert!(withdraw_contract_address.len() == 20);
    // size: 20bytes

    let withdraw_map_base_key = zkm2_zkvm::io::read();  
    let withdraw_map_index = zkm2_zkvm::io::read(); 
    let peg_in_txid: Vec<u8> = zkm2_zkvm::io::read(); 
    let tx_list: Vec<u8> = zkm2_zkvm::io::read(); 
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

    // public inputs
    zkm2_zkvm::io::commit(&goat_withdraw_txid);
    zkm2_zkvm::io::commit(&withdraw_contract_address);
}