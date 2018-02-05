# Goals
* Provide default implementation on indy-sdk level for tails routine
* Need to be able to extend and customize tails handler in applications
* * For example allow client to not downloads whole large tails

# Tails
Expected size is about 1/2 GB per Issuer revocation registry.
"As is" tails is static (once generated) array of BigIntegers

## Required Tails Functionality
* generate_tails
* init_tails_storage
* deinit_tails_storage
* get_tail

## Indy Crypto API for tails operate
TODO

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
