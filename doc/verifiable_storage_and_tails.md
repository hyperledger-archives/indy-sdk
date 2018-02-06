# Goals
* Provide default implementation on indy-sdk level for Tails routine
* Need to be able to extend and customize tails handler in applications
  * For example allow client to not downloads whole large tails

# Tails
It's an ordered sequence of elements used for non-revocation proof. "As is" tails is static (once generated) array of BigIntegers
* may require quite huge amount of data (up to 1/2GB per Issuer revocation registry);
* are created and uploaded by Issuers;
* are required (so must be available for download) for both Provers and Verifiers;
* can be cached and can be downloaded only once;
* it's may not be enough just to store tails on Issuer side, as Issuer may be offline

## Required Tails Functionality
* generate_tails
* init_tails_storage
* get_tail

## Indy Crypto API for tails operate
* Some calls in Anoncreds API are require access to tails for read.
  For these calls `fetch_tail(context, index)` should be passed.
* There is ability to generate Revocation Registry (including tails) by IndyCrypto.
  The previous version API has `new_revocation_registry` method returning public and private part of the registry.
  And whole tails are included into public part.
  The new one version of API should return public part of revocation registry without Tails. And one more method should be added `generate_tails(rev_reg_private, from_idx, to_idx, *buf_ptr, buf_sz)`.
  * `rev_reg_private` private part of `new_revocation_registry` result
  * `from_idx..to_idx` - range of tails to generate (include `from`, exclude `to`)
  * `buf_ptr` is allocated by caller buffer to store generated tails and `buf_sz` is size in bytes of this buffer   

# Verifiable Storage (VS)
Indy SDK can implement some generic storage with verify ability.
It may be organised as Merkle Tree above raw binary data.
TODO research MT modifications for binary data.
This storage can be used to handle tails.

## VS API in SDK
TODO

## Default VS implementation in SDK
TODO
* init_storage(init_URI) -> handle
* append/insert/update data (handle, position, data)
* finalize(handle) -> ID, rootHash, final_URI
* get_data(from, to) -> data, proof
And one more call to register implementation of the interface
register_storage_type(type, callbacks)

# Indy Crypto and Indy SDK workflow
Indy SDK will contain helpers to work with VS as Tails Storage.
