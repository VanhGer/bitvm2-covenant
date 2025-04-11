use std::env;
use std::fs::{read, File};
use std::io::Read;
use std::path::Path;
use std::time::Instant;
use zkm_sdk::prover::{ClientCfg, ProverInput};
use zkm_sdk::ProverClient;
use common::file;

const ELF: &[u8] = include_bytes!(env!("ZKM_ELF_bitvm2-covenant"));
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::try_init().unwrap_or_default();

    // Set up
    let output_dir = env::var("OUTPUT_DIR").unwrap_or(String::from("./output"));
    let seg_size = env::var("SEG_SIZE").unwrap_or("262144".to_string());
    let seg_size = seg_size.parse::<_>().unwrap_or(262144);
    let execute_only = env::var("EXECUTE_ONLY").unwrap_or("false".to_string());
    let execute_only = execute_only.parse::<bool>().unwrap_or(false);

    // network proving
    let endpoint = env::var("ENDPOINT").map_or(None, |endpoint| Some(endpoint.to_string()));
    let ca_cert_path = env::var("CA_CERT_PATH").map_or(None, |path| Some(path.to_string()));
    let cert_path = env::var("CERT_PATH").map_or(None, |x| Some(x.to_string()));
    let key_path = env::var("KEY_PATH").map_or(None, |x| Some(x.to_string()));
    let domain_name = Some(env::var("DOMAIN_NAME").unwrap_or("stage".to_string()));
    let proof_network_privkey =
        env::var("PROOF_NETWORK_PRVKEY").map_or(None, |x| Some(x.to_string()));

    let prover_cfg = ClientCfg {
        zkm_prover_type: "network".to_string(),
        endpoint,
        ca_cert_path,
        cert_path,
        key_path,
        domain_name,
        proof_network_privkey,
    };
    let prover_client = ProverClient::new(&prover_cfg).await;

    let mut prover_input = ProverInput {
        elf: Vec::from(ELF),
        seg_size,
        execute_only,
        ..Default::default()
    };
    //If the guest program doesn't have inputs, it doesn't need the setting.
    set_guest_input(&mut prover_input);

    let start = Instant::now();
    let proving_result = prover_client.prover.prove(&prover_input, None).await;
    match proving_result {
        Ok(Some(prover_result)) => {
            if !execute_only {
                if prover_result.proof_with_public_inputs.is_empty() {
                    log::info!(
                        "Fail: snark_proof_with_public_inputs.len() is : {}.Please try setting SEG_SIZE={}",
                        prover_result.proof_with_public_inputs.len(), seg_size/2
                    );
                }
                let tmp = prover_result.proof_with_public_inputs.clone();
                let output_path = Path::new(&output_dir);
                let proof_result_path =
                    output_path.join("snark_proof_with_public_inputs.json");
                let mut f = file::new(&proof_result_path.to_string_lossy());
                match f.write(prover_result.proof_with_public_inputs.as_slice()) {
                    Ok(bytes_written) => {
                        log::info!("Proof: successfully written {} bytes.", bytes_written);
                    }
                    Err(e) => {
                        log::info!("Proof: failed to write to file: {}", e);
                    }
                }
                log::info!("Generating proof successfully.");
            } else {
                log::info!("Generating proof successfully .The proof is not saved.");
            }
        }
        Ok(None) => {
            log::info!("Failed to generate proof.The result is None.");
        }
        Err(e) => {
            log::info!("Failed to generate proof. error: {}", e);
        }
    }

    let end = Instant::now();
    let elapsed = end.duration_since(start);
    log::info!(
        "Elapsed time: {:?} secs",
        elapsed.as_secs(),
    );
    Ok(())
}

fn set_guest_input(prover_input: &mut ProverInput) {
    let goat_withdraw_txid: Vec<u8> =
        hex::decode(std::env::var("GOAT_WITHDRAW_TXID").unwrap_or("32bc8a6c5b3649f92812c461083bab5e8f3fe4516d792bb9a67054ba040b7988".to_string())).unwrap();
    write_to_guest_public_input(prover_input, &goat_withdraw_txid);

    let withdraw_contract_address: Vec<u8> =
        hex::decode(std::env::var("WITHDRAW_CONTRACT_ADDRESS").unwrap_or("86a77bdfcaff7435e1f1df06a95304d35b112ba8".to_string()))
            .unwrap();
    write_to_guest_public_input(prover_input, &withdraw_contract_address);

    let withdraw_map_base_key =
        hex::decode(std::env::var("WITHDRAW_MAP_BASE_KEY").unwrap_or("32bc8a6c5b3649f92812c461083bab5e8f3fe4516d792bb9a67054ba040b7988".to_string())).unwrap();
    write_to_guest_public_input(prover_input, &withdraw_map_base_key);
    let withdraw_map_index =
        hex::decode(std::env::var("WITHDRAW_MAP_INDEX").unwrap_or("32bc8a6c5b3649f92812c461083bab5e8f3fe4516d792bb9a67054ba040b7988".to_string())).unwrap();
    write_to_guest_public_input(prover_input, &withdraw_map_index);
    let peg_in_txid: Vec<u8> =
        hex::decode(std::env::var("PEG_IN_TXID").unwrap_or("32bc8a6c5b3649f92812c461083bab5e8f3fe4516d792bb9a67054ba040b7988".to_string())).unwrap();
    write_to_guest_public_input(prover_input, &peg_in_txid);

    let manifest_path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let json_path =
        env::var("JSON_PATH").unwrap_or(format!("{}/../test-vectors/3168249.json", manifest_path));
    let mut f = File::open(json_path).unwrap();
    let mut data = vec![];
    f.read_to_end(&mut data).unwrap();

    let encoded = guest_std::cbor_serialize(&data).unwrap();
    write_to_guest_public_input(prover_input, &encoded);
}
fn write_to_guest_public_input(prover_input: &mut ProverInput, data: &[u8]) {
    let mut tmp = Vec::new();
    bincode::serialize_into(&mut tmp, data).expect("serialization failed");
    prover_input.public_inputstream.extend(tmp);
}
