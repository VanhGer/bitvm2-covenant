extern crate libc;

use models::{Test, TestSuite, TestUnit, SpecName};

pub use guest::{execute_test_suite, read_suite, verify_revm_tx};

extern crate alloc;
use alloc::format;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use revm::primitives::Address;


/// Check withraw txid is included in current suite
pub fn check_withdraw(
    contract_address: &Vec<u8>,
    withdraw_txid: &Vec<u8>,
    withdraw_map_base_key: &Vec<u8>,
    withdraw_map_index: &Vec<u8>,
    peg_in_txid: &Vec<u8>,
    suite: &TestSuite,
) -> Result<(), String> {
    // calc slot id: slot=keccak256(key.base_slot)
    assert!(withdraw_map_base_key.len() == 32);
    assert!(withdraw_map_index.len() == 32);

    // TODO: check federation signature

    let contract_address = Address::from_slice(contract_address.as_slice());
    let hex_withdraw_txid = format!("0x{}", hex::encode(withdraw_txid));
    let tx_info: &TestUnit = suite.0.get(&hex_withdraw_txid).unwrap();
    let post: &Vec<Test> = &tx_info.post.get(&SpecName::Cancun).unwrap();

    let mut inputs = withdraw_map_base_key.clone();
    inputs.extend_from_slice(&withdraw_map_index);
    for t in post {
        // BUG: skip all below checks if you want to generate the proof only
        if let Some(acc) = &t.post_state.get(&contract_address) {
           //let slot: B256 = keccak256(&inputs);
           ////println!("slots: {}, {:?}", slot, acc.storage);
           //let actual_peg_in_txid = acc.storage.get::<U256>(&slot.into()).unwrap();

           //// NOTE: BE
           //let expected = U256::from_be_slice(&peg_in_txid);

           //assert_eq!(expected, *actual_peg_in_txid);

           //let slot =  <FixedBytes<32> as Into<U256>>::into(slot) + U256::from(1);
           //let acutal_one = acc.storage.get(&slot).unwrap(); 
           //assert_eq!(*acutal_one, U256::from(1));
           return Ok(());
        } 
    }

    return Err("Contract is not called in current transaction".to_string());
}
