# Shmair secret API (indy-crypto and indy-sdk)
**Objective**: `indy-crypto` exposes the low level API for generating and reconstructing secrets. `indy-sdk` uses the underlying `indy-crypto` and exposes an API to shard a JSON message, store the shards and reconstitute the secret.

## Indy-crypto
1. `shard_secret(secret: bytes, m: u8, n: u8, sign_shares: Option<bool>) -> Result<Vec<Share>, IndyCryptoError>`.  
Splits the bytes of the secret `secret` in `n` different shares and `m-of-n` shares are required to reconstitute the secret. `sign_shares` if provided, all shards are signed.  
1. `recover_secret(shards: Vec<Share>, verify_signatures: Option<bool>) -> Result<Vec<u8>, IndyCryptoError>`.  
Recover the secret from the given `shards`. `verify_signatures` if given verifies the signatures.

## Indy-sdk
1. `shard_JSON(msg: String, m: u8, n: u8, sign_shares: Option<bool>) -> Result<Vec<String>, IndyError>`  
Takes the message as a JSON string and serialises it to bytes and passes it to `shard_secret` of `indy-crypto`. The serialisation has to be deterministic, i.e the same JSON should always serialise to same bytes everytime. The resulting `Share` given by `indy-crypto` is converted to JSON before returning. 
1. `shard_JSON_with_wallet_data(wallet_handle: i32, msg: String, wallet_keys:Vec<&str>, m: u8, n: u8, sign_shares: Option<bool>) -> Result<Vec<String>, IndyError>`  
Takes the message as a JSON string, updates the JSON with key-values from wallet given by handle `wallet_handle`, keys present in the vector `wallet_keys` and passes the resulting JSON to `shard_JSON`. 
1. `recover_secret(shards: Vec<String>, verify_signatures: Option<bool>) -> Result<String, IndyError>`  
Takes a collection of shards each encoded as JSON, deserialises them into `Share`s and passes them to `recover_secret` from `indy-crypto`. It converts the resulting secret back to JSON before returning it.
1. `shard_JSON_and_store_shards(wallet_handle: i32, msg: String, m: u8, n: u8, sign_shares: Option<bool>) -> Result<String, IndyError>`  
Shards the given JSON using `shard_JSON` and store shards as a JSON array (each shard is an object in itself) in the wallet given by `wallet_handle`. Returns the wallet key used to store the shards.
