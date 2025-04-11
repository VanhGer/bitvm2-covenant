#!/bin/bash -e

BASEDIR=$(cd $(dirname $0); pwd)
export ZKM_PROVER=${ZKM_PROVER-"network"}
export RUST_LOG=${RUST_LOG-info}
export SEG_SIZE=${SEG_SIZE-262144}
export OUTPUT_DIR=${BASEDIR}/output
export EXECUTE_ONLY=false

##network proving
export CA_CERT_PATH=${BASEDIR}/tool/ca.pem
#export CERT_PATH=
#export KEY_PATH=
##The private key corresponding to the public key when registering in the https://www.zkm.io/apply
#export PROOF_NETWORK_PRVKEY=
export ENDPOINT=https://152.32.186.45:20002    ##the test entry of zkm proof network
#export DOMAIN_NAME=

#export GOAT_WITHDRAW_TXID=
#export WITHDRAW_CONTRACT_ADDRESS=
#export WITHDRAW_MAP_BASE_KEY=
#export WITHDRAW_MAP_INDEX=
#export PEG_IN_TXID=
#export JSON_PATH=

cargo run --release