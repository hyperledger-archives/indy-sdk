# Caching of data from ledger

Currently whenever credential definitions and/or schemas is needed, it is being fetched from the ledger.
This operation may last multiple seconds and is slowing down usage of credentials.
Caching also enables usage of anoncreds in areas where user do not have internet coverage (eg. Using passport credential on foreign airport).

## Goals and ideas

* Allow users to cache credential definitions and schemas.
  * Local wallet to be used because although this data is public, possession of some credential definition or schema reveals possession of respective credential. 
* Provide higher level api for fetching this data so it is easier to use.
  * Caching should be transparent to the user.
* Enable purging of old (not needed more) data.
  
## Public API

Note: In all calls `pool_handle` may be removed if did resolver is implemented.

```Rust
/// Gets credential definition json data for specified credential definition id.
/// If data is present inside of cache, cached data is returned.
/// Otherwise data is fetched from the ledger and stored inside of cache for future use.
/// 
/// #Params
/// command_handle: command handle to map callback to caller context.
/// pool_handle: pool handle (created by open_pool_ledger).
/// wallet_handle: wallet handle (created by open_wallet).
/// submitter_did: DID of the submitter stored in secured Wallet.
/// id: identifier of credential definition.
/// cb: Callback that takes command result as parameter.
#[no_mangle]
pub extern fn indy_get_cred_def(command_handle: IndyHandle,
                                pool_handle: IndyHandle,
                                wallet_handle: IndyHandle,
                                submitter_did: *const c_char,
                                id: *const c_char,
                                cb: Option<extern fn(command_handle_: IndyHandle,
                                                     err: ErrorCode,
                                                     cred_def_json: *const c_char)>) -> ErrorCode {
}

/// Gets schema json data for specified schema id.
/// If data is present inside of cache, cached data is returned.
/// Otherwise data is fetched from the ledger and stored inside of cache for future use.
/// 
/// #Params
/// command_handle: command handle to map callback to caller context.
/// pool_handle: pool handle (created by open_pool_ledger).
/// wallet_handle: wallet handle (created by open_wallet).
/// submitter_did: DID of the submitter stored in secured Wallet.
/// id: identifier of schema.
/// cb: Callback that takes command result as parameter.
#[no_mangle]
pub extern fn indy_get_schema(command_handle: IndyHandle,
                              pool_handle: IndyHandle,
                              wallet_handle: IndyHandle,
                              submitter_did: *const c_char,
                              id: *const c_char,
                              cb: Option<extern fn(command_handle_: IndyHandle,
                                                   err: ErrorCode,
                                                   schema_json: *const c_char)>) -> ErrorCode {
}
```

## Storing of the data into wallet

Data would be stored with specific cache type so that it is separated and easy to be managed.
Schema_id or cred_def_id would be used for id of wallet data.
This way data may be fetched very efficiently and also easy to be deleted when needed.

## Purging the cache

Several methods may be implemented for purging the cached data:

#### Purge all

Advantages:
* Very simple to implement.
* Only one method would be needed (eg. `indy_purge_cache`).

Disadvantages:
* Low selectivity of purging

#### Purge all of one type

Advantages:
* Very simple to implement.
* Only one method per cache type is needed (eg. `indy_purge_cred_def_cache`).

Disadvantages:
* Low selectivity of purging

#### LRU mechanism limited by size/number of cached data.

Advantages:
* Limited wallet data.
* Only useful data is being kept, older not needed data is being purged automatically.

Disadvantages:
* Complex to implement.
* Every fetching of data from cache would also introduce update of timestamp data (last used).

Also some data can be locked-in to be always present (useful for out of internet scenario).