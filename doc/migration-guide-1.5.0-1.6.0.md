<!-- markdownlint-disable MD033 -->

# Libindy 1.5 to 1.6 migration Guide

This document is written for developers using Libindy to provide necessary information and
to simplify their transition to Libindy 1.6 from Libindy 1.5. If you are using older Libindy
version you can check migration guides history:

* [Libindy 1.3 to 1.4 migration](#libindy-1.3-to-1.4-migration)
* [Libindy 1.4 to 1.5 migration](#libindy-1.4-to-1.5-migration)

## Table of contents

* [Notes](#notes)
* [Wallet API](#wallet-api)
* [Anoncreds API](#anoncreds-api)

## Notes

Migration information is organized in tables, there are mappings for each Libindy API part of how older version functionality maps to a newer one.
Functions from older version are listed in the left column, and the equivalent newer version function is placed in the right column:

* If some function had been added, the word 'NEW' would be placed in the left column.
* If some function had been deleted, the word 'DELETED' would be placed in the right column.
* If some function had been deprecated, the word 'DEPRECATED' would be placed in the right column.
* If some function had been changed, the current format would be placed in the right column.
* If some function had not been changed, the symbol '=' would be placed in the right column.
* To get more details about current format of a function click on the description above it.
* Bellow are signatures of functions in Libindy C API.
  The params of ```cb``` (except command_handle and err) will be result values of the similar function in any Libindy wrapper.

### Wallet API

The main goal of changes performed in Wallet API is to avoid maintaining created wallet list on libindy side.
It allows to access wallets from a cluster and solves some problems on mobile platforms.
A significant part of Wallet APIs has been updated to accept wallet configuration as a single json 
which provides wallet configuration, wallet storage configuration, and placement inside of storage.
This wallet configuration json has the following format:
```
 {
   "id": string, Identifier of the wallet.
         Configured storage uses this identifier to lookup exact wallet data placement.
   "storage_type": optional<string>, Type of the wallet storage. Defaults to 'default'.
                  'Default' storage type allows to store wallet data in the local file.
                  Custom storage types can be registered with indy_register_wallet_storage call.
   "storage_config": optional<object>, Storage configuration json. Storage type defines set of supported keys.
                     Can be optional if storage supports default configuration.
                     For 'default' storage type configuration is:
   {
     "path": optional<string>, Path to the directory with wallet files.
             Defaults to $HOME/.indy_client/wallets.
             Wallet will be stored in the file {path}/{id}/sqlite.db
   }
 }
```

<table>
  <tr>  
    <th>v1.5.0 - Wallet API</th>
    <th>v1.6.0 - Wallet API</th>
  </tr>
    <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/wallet.rs#L142">
            Create a new secure wallet.
        </a>
    </th>
  <tr>
    <td>
      <pre>
indy_create_wallet(command_handle: i32,
                   pool_name: *const c_char,
                   name: *const c_char,
                   xtype: *const c_char,
                   config: *const c_char,
                   credentials: *const c_char,
                   cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode)>)
      </pre>
    </td>
    <td>
      <pre>
indy_create_wallet(command_handle: i32,
                   config: *const c_char,
                   credentials: *const c_char,
                   cb: Option<extern fn(xcommand_handle: i32,
                                        err: ErrorCode)>)
      </pre>
      <b>Note:</b> Format of <i>config</i> parameter was changed. Current format is described above.
    </td>
  </tr>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/wallet.rs#L208">
            Open the wallet.
        </a>
    </th>
  <tr>
    <td>
      <pre>
indy_open_wallet(command_handle: i32,
                 name: *const c_char,
                 runtime_config: *const c_char,
                 credentials_json: *const c_char,
                 cb: Option<extern fn(xcommand_handle: i32,
                                      err: ErrorCode,
                                      handle: i32)>)
      </pre>
    </td>
    <td>
      <pre>
indy_open_wallet(command_handle: i32,
                 config: *const c_char,
                 credentials: *const c_char,
                 cb: Option<extern fn(xcommand_handle: i32,
                                      err: ErrorCode,
                                      handle: i32)>)
      </pre>
      <b>Note:</b> Format of <i>config</i> parameter was changed. Current format is described above.
    </td>
  </tr>
    <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/wallet.rs#L449">
            Deletes created wallet
        </a>
    </th>
  <tr>
    <td>
      <pre>
indy_delete_wallet(command_handle: i32,
                   name: *const c_char,
                   credentials: *const c_char,
                   cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode)>)
      </pre>
    </td>
    <td>
      <pre>
indy_delete_wallet(command_handle: i32,
                   config: *const c_char,
                   credentials: *const c_char,
                   cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode)>)
      </pre>
      <b>Note:</b> Format of <i>config</i> parameter was changed. Current format is described above.
    </td>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/wallet.rs#L359">
            Import wallet
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_import_wallet(
            command_handle: i32,
            pool_name: *const c_char,
            name: *const c_char,
            storage_type: *const c_char,
            config: *const c_char,
            credentials: *const c_char,
            import_config_json: *const c_char,
            cb: fn(xcommand_handle: i32, 
                   err: ErrorCode))
        </pre>
    </td>
    <td>
      <pre>
indy_import_wallet(
            command_handle: i32,
            config: *const c_char,
            credentials: *const c_char,
            import_config_json: *const c_char,
            cb: fn(xcommand_handle: i32, 
                   err: ErrorCode))
        </pre>
      <b>Note:</b> Format of <i>config</i> parameter was changed. Current format is described above.
    </td>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/rc/libindy/src/api/wallet.rs#L271">
            Lists created wallets
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_list_wallets(command_handle: i32,
                  cb: fn(xcommand_handle: i32,
                         err: ErrorCode,
                         wallets: *const c_char))
        </pre>
    </td>
    <td>
      <b>DELETED</b>
    </td>
  </tr>
</table>

### Anoncreds API

The main goal of changes performed in Anoncreds API is integration tags based search to Anoncreds workflow 
as it has done in Non-Secrets API.
 * Create tags for a stored object.
 * Provide efficient and flexible search for entities using WQL.
 * Avoid immediately returning all matched records.
 * Provide ability to fetch records by small batches.

* Updated behavior of `indy_prover_store_credential` API function to create tags for a stored credential object.
* ```indy_prover_get_credentials``` endpoint is DEPRECATED and will be removed in the next release because immediately returns all fetched credentials.
* ```indy_prover_get_credentials_for_proof_req``` endpoint is DEPRECATED and will be removed in the next release because immediately returns all fetched credentials.
* Added two chains of APIs related to credentials search that allows fetching records by batches:
     * Simple credentials search - `indy_prover_search_credentials` -> `indy_prover_fetch_credentials` -> `indy_prover_close_credentials_search`
     * Search credentials for proof request - `indy_prover_search_credentials_for_proof_req` -> `indy_prover_fetch_credentials_for_proof_req` -> `indy_prover_close_credentials_search_for_proof_req`
* ```indy_prover_get_credential``` was added to Libindy Anoncreds API to allow getting human readable credential by the given id.

References:

* [Wallet Query Language](https://github.com/hyperledger/indy-node/blob/master/docs/design/011-wallet-query-language/README.md)

<table>
  <tr>
    <th>v1.5.0 - Anoncreds API</th>
    <th>v1.6.0 - Anoncreds API</th>
  </tr>  
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/anoncreds.rs#L798">
            Gets human readable credential by the given id
        </a>
    </th>
  </tr>
  <tr>
    <td><b>NEW</b></td>
    <td>
      <pre>
indy_prover_get_credential(
            command_handle: i32,
            wallet_handle: i32,
            cred_id: *const c_char,
            cb: fn(xcommand_handle: i32, 
                   err: ErrorCode,
                   credential_json: *const c_char))
        </pre>
    </td>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/anoncreds.rs#L855">
            Gets human readable credentials according to the filter
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_prover_get_credentials(
        command_handle: i32,
        wallet_handle: i32,
        filter_json: *const c_char,
        cb: fn(xcommand_handle: i32, 
               err: ErrorCode,
               credentials_json: *const c_char))
        </pre>
    </td>
    <td><b>DEPRECATED</b></td>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/anoncreds.rs#L925">
            Search for credentials stored in wallet
        </a>
    </th>
  </tr>
  <tr>
    <td><b>NEW</b></td>
    <td colspan="2">
      <pre>
indy_prover_search_credentials(
            command_handle: i32,
            wallet_handle: i32,
            query_json: *const c_char,
            cb: fn(xcommand_handle: i32, 
                   err: ErrorCode,
                   search_handle: i32,
                   total_count: usize))
        </pre>
    </td>
  </tr>  
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/anoncreds.rs#L980">
            Fetch next credentials for search
        </a>
    </th>
  </tr>
  <tr>
    <td><b>NEW</b></td>
    <td colspan="2">
      <pre>
indy_prover_fetch_credentials(
            command_handle: i32,
            search_handle: i32,
            count: usize,
            cb: fn(command_handle_: i32, 
                   err: ErrorCode,
                   credentials_json: *const c_char))
        </pre>
    </td>
  </tr> 
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/anoncreds.rs#L1036">
            Close credentials search
        </a>
    </th>
  </tr>
  <tr>
    <td><b>NEW</b></td>
    <td colspan="2">
      <pre>
indy_prover_close_credentials_search(
            command_handle: i32,
            search_handle: i32,
            cb: fn(command_handle_: i32, 
                   err: ErrorCode))
        </pre>
    </td>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/anoncreds.rs#L1074">
            Gets human readable credentials matching the given proof request
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_prover_get_credentials_for_proof_req(
        command_handle: i32,
        wallet_handle: i32,
        proof_request_json: *const c_char,
        cb: fn(xcommand_handle: i32, 
               err: ErrorCode,
               credentials_json: *const c_char))
        </pre>
    </td>
    <td><b>DEPRECATED</b></td>
  </tr>
  <tr>
    <th colspan="2">
      <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/anoncreds.rs#L1191">
        Search for credentials matching the given proof request.
      </a>
    </th>
  </tr>
  <tr>
    <td><b>NEW</b></td>
    <td>
      <pre>
indy_prover_search_credentials_for_proof_req(
            command_handle: i32,
            wallet_handle: i32,
            proof_request_json: *const c_char,
            extra_query_json: *const c_char,
            cb: fn(xcommand_handle: i32, 
                   err: ErrorCode,
                   search_handle: i32))
      </pre>
    </td>
  </tr>
  <tr>
    <th colspan="2">
      <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/anoncreds.rs#L1270">
        Fetch next credentials for the requested item using proof request search handle
      </a>
    </th>
  </tr>
  <tr>
    <td><b>NEW</b></td>
    <td>
      <pre>
indy_prover_fetch_credentials_for_proof_req(
            command_handle: i32,
            search_handle: i32,
            item_referent: *const c_char,
            count: usize,
            cb: fn(command_handle_: i32, 
                   err: ErrorCode,
                   credentials_json: *const c_char))
      </pre>
    </td>
  </tr>
  <tr>
    <th colspan="2">
      <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/anoncreds.rs#L1343">
        Close credentials search for proof request
      </a>
    </th>
  </tr>
  <tr>
    <td><b>NEW</b></td>
    <td>
      <pre>
indy_prover_close_credentials_search_for_proof_req(
            command_handle: i32,
            search_handle: i32,
            cb: fn(command_handle_: i32, 
                   err: ErrorCode))
      </pre>
    </td>
  </tr>
</table>