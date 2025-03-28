# Bitvm2-covenant

## Run the CI

Generate the test suites via [zkMIPS/remve](https://github.com/zkMIPS/revme).
```
cargo run -r  --bin bitvm2-covenant-guest
```

## Generate the Proof
```
export CARGO_NET_GIT_FETCH_WITH_CLI=true

ZKM_DEV=true cargo run -r  --bin bitvm2-covenant-host
```
