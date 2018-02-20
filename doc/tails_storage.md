# Goals
* Provide default implementation on indy-sdk level for Tails routine
* Need to be able to extend and customize tails handler in applications
  * For example allow client to not downloads whole large tails

# Tails
It's an ordered sequence of elements used for non-revocation proof. "As is" tails is static (once generated) array of BigIntegers
* may require quite huge amount of data (up to 1/2GB per Issuer revocation registry);
* are created and uploaded by Issuers;
* are required (so must be available for download) for both Provers and Issuers;
* can be cached and can be downloaded only once;
* it's may not be enough just to store tails on Issuer side, as Issuer may be offline

## Required Tails Functionality
* generate_tails
* init_tails_storage
* get_tail

## Indy Crypto Rust API for tails operate
* Some calls in Anoncreds API are require access to tails for read.
  For these calls `trait RevocationTailsAccessor` should be implemented and passed as parameter.
  This trait assume the function `access_tail(&self, tail_id: u32, accessor: &mut FnMut(&Tail)) -> Result<(), IndyCryptoError>`
* There is ability to generate Revocation Registry (including tails) by IndyCrypto.
  The previous version API has `new_revocation_registry` method returning public and private part of the registry.
  And whole tails are included into public part.
  The new one version of API should return public part of revocation registry without Tails. 
  And also it returns `RevocationTailsGenerator` object which should be used to generate all tails one by one and store it somehow
  * `count() -> u32` count of rest Tails in the generator
  * `next() -> Result<Tail, IndyCryptoError>` - generate next tail
* Tail::from_bytes
* Tail::to_bytes   

## Tails API in SDK
### TailsReader
* tails_reader_verify_consistency(type, URI, hash) -> bool
* tails_reader_open(type, URI, hash) -> handle
* tails_reader_close(handle)
* tails_reader_calc_tail_offset()
* tails_reader_tail_from_bytes(tail_bytes)
* tails_reader_register_type(type,
    (access_tail)(handle, idx) -> Tail
    (verify_consistency)(URI) -> hash
  )

`handle` will be used for calls like `indy_prover_create_proof` or `indy_issuer_revoke_claim`

### TailsWriter
* tails_writer_init_config(type, config) -> config_handle
* tails_writer_register_type(type,
    (append_tail)(tail_bytes)
    (finalize_tail)() -> hash
  )

`config_handle` will be used in `indy_issuer_create_and_store_revoc_reg` (multiply times)

## Default VS implementation in SDK
TODO
