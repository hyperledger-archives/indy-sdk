<!-- markdownlint-disable MD033 -->

# Libindy 1.5 to 1.6 migration Guide

This document is written for developers using Libindy to provide necessary information and
to simplify their transition to Libindy 1.6 from Libindy 1.5. If you are using older Libindy
version you can check migration guides history:

* [Libindy 1.3 to 1.4 migration](https://github.com/hyperledger/indy-sdk/blob/v1.4.0/doc/migration-guide.md)
* [Libindy 1.4 to 1.5 migration](https://github.com/hyperledger/indy-sdk/blob/v1.5.0/doc/migration-guide-1.4.0-1.5.0.md)

## Table of contents

* [Notes](#notes)
* [Libindy 1.5 to 1.6.0 migration](#libindy-15-to-160-migration-guide)
    * [Wallet API](#wallet-api)
    * [Anoncreds API](#anoncreds-api)
    * [Payments API](#payments-api)
    * [Pool API](#payments-api)
* [Libindy 1.6.0 to 1.6.1 migration](#libindy-160-to-161-migration-guide)
* [Libindy 1.6.1 to 1.6.2 migration](#libindy-161-to-162-migration-guide)
    * [Wallet API 1.6.2](#wallet-api-162)
    * [Ledger API 1.6.2](#ledger-api-162)
* [Libindy 1.6.2 to 1.6.3 migration](#libindy-162-to-163-migration-guide)
    * [Wallet API 1.6.3](#wallet-api-163)
    * [Ledger API 1.6.3](#ledger-api-163)
* [Libindy 1.6.3 to 1.6.4 migration](#libindy-163-to-164-migration-guide)
* [Libindy 1.6.4 to 1.6.5 migration](#libindy-164-to-165-migration-guide)
* [Libindy 1.6.5 to 1.6.6 migration](#libindy-164-to-165-migration-guide)
* [Libindy 1.6.6 to 1.6.7 migration](#libindy-164-to-165-migration-guide)


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

## Libindy 1.5 to 1.6.0 migration Guide

### Wallet API

The main idea of changes performed in Wallet API is to avoid maintaining created wallet list on Libindy side.
It allows to access wallets from a cluster and solves some problems on mobile platforms.

The following changes have been performed:
* Changed Wallet export serialization:
    * Use MsgPack instead of custom entities serialization to be more reliable and allow extend-ability in backward compatible way.
    * Changed header format to be more reliable and allow extend-ability in backward compatible way.
    * Use STOP message to make sure that there was no truncation of export file.
    * Removed EXPERIMENTAL notice for import/export API endpoints.
* Removed association between Wallet and Pool.
* Removed persistence of Wallet configuration by Libindy.
* A significant part of Wallet APIs has been updated to accept wallet configuration as a single json 
which provides whole wallet configuration. 
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

*WARNING* Wallet format of libindy v1.6 isn't compatible with a wallet format of libindy v1.5.

<table>
  <tr>  
    <th>v1.5.0 - Wallet API</th>
    <th>v1.6.0 - Wallet API</th>
  </tr>
    <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/v1.6.0/libindy/src/api/wallet.rs#L142">
            Create a new secure wallet
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
        <a href="https://github.com/hyperledger/indy-sdk/blob/v1.6.0/libindy/src/api/wallet.rs#L208">
            Open the wallet
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
        <a href="https://github.com/hyperledger/indy-sdk/blob/v1.6.0/libindy/src/api/wallet.rs#L443">
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
        <a href="https://github.com/hyperledger/indy-sdk/blob/v1.6.0/libindy/src/api/wallet.rs#L328">
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
        <a href="">
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

The main idea of changes performed in Anoncreds API is integration tags based search to Anoncreds workflow 
as it has done in Non-Secrets API.
 * Create tags for a stored object.
 * Provide efficient and flexible search for entities using WQL.
 * Avoid immediately returning all matched records.
 * Provide ability to fetch records by small batches.

The following changes have been performed:
* Updated behavior of `indy_prover_store_credential` API function to create tags for a stored credential object.
Here is the list of tags will be created:
```
{
    "schema_id": <credential schema id>,
    "schema_issuer_did": <credential schema issuer did>,
    "schema_name": <credential schema name>,
    "schema_version": <credential schema version>,
    "issuer_did": <credential issuer did>,
    "cred_def_id": <credential definition id>,
    // for every attribute in <credential values>
    "attr::<attribute name>::marker": "1", // to check existence of attribute
    "attr::<attribute name>::value": <attribute raw value>, // to check value of attribute
}
```
* ```indy_prover_get_credential``` was added to Libindy Anoncreds API to allow getting human readable credential by the given id.
* Updated ```indy_prover_get_credentials``` and ```indy_prover_get_credentials_for_proof_req``` API functions to support tags based search.
* ```indy_prover_get_credentials``` endpoint marked as DEPRECATED and will be removed in the next release because immediately returns all fetched credentials.
* ```indy_prover_get_credentials_for_proof_req``` endpoint marked as DEPRECATED and will be removed in the next release because immediately returns all fetched credentials.
* Added two chains of APIs related to credentials search that allows fetching records by batches:
     * Simple credentials search - `indy_prover_search_credentials` -> `indy_prover_fetch_credentials` -> `indy_prover_close_credentials_search`
     * Search credentials for proof request - `indy_prover_search_credentials_for_proof_req` -> `indy_prover_fetch_credentials_for_proof_req` -> `indy_prover_close_credentials_search_for_proof_req`

**Note:** 
* Functions ```indy_prover_get_credentials``` and ```indy_prover_get_credentials_for_proof_req``` support both formats of queries: old strict filter and WQL.
* Functions ```indy_prover_search_credentials``` and ```indy_prover_search_credentials_for_proof_req``` support only WQL format of queries.

References:

* [Wallet Query Language](https://github.com/hyperledger/indy-node/blob/master/docs/design/011-wallet-query-language/README.md)

<table>
  <tr>
    <th>v1.5.0 - Anoncreds API</th>
    <th>v1.6.0 - Anoncreds API</th>
  </tr>  
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/v1.6.0/libindy/src/api/anoncreds.rs#L798">
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
        <a href="https://github.com/hyperledger/indy-sdk/blob/v1.6.0/libindy/src/api/anoncreds.rs#L855">
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
        <a href="https://github.com/hyperledger/indy-sdk/blob/v1.6.0/libindy/src/api/anoncreds.rs#L925">
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
        <a href="https://github.com/hyperledger/indy-sdk/blob/v1.6.0/libindy/src/api/anoncreds.rs#L980">
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
        <a href="https://github.com/hyperledger/indy-sdk/blob/v1.6.0/libindy/src/api/anoncreds.rs#L1036">
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
        <a href="https://github.com/hyperledger/indy-sdk/blob/v1.6.0/libindy/src/api/anoncreds.rs#L1074">
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
      <a href="https://github.com/hyperledger/indy-sdk/blob/v1.6.0/libindy/src/api/anoncreds.rs#L1191">
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
      <b>NOTE:</b> Added parameter <b>extra_query_json</b>.
      Using this parameter you can pass an additional 
      query to any attribute/predicate in a proof request.
      </pre>
    </td>
  </tr>
  <tr>
    <th colspan="2">
      <a href="https://github.com/hyperledger/indy-sdk/blob/v1.6.0/libindy/src/api/anoncreds.rs#L1270">
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
      <a href="https://github.com/hyperledger/indy-sdk/blob/v1.6.0/libindy/src/api/anoncreds.rs#L1343">
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


### Payments API

The main idea of changes performed in Payment API is avoiding UTXO based payments approach. 
Payment API has been updated to support non-UTXO based crypto payments and traditional payments like VISA.

**Note:** Removed EXPERIMENTAL notice for API endpoints.

The following changes have been performed:
* Changed format of input and output parameters.
* Changed format of result values of `indy_parse_response_with_fees` and `indy_parse_payment_response` API functions.
* Renamed `indy_build_get_utxo_request` and `indy_parse_get_utxo_response` API functions.
* Added `indy_build_verify_payment_req` and `indy_parse_verify_payment_response` API functions.

<table>
  <tr>
    <th>v1.5.0 - Payment API</th>
    <th>v1.6.0 - Payment API</th>
  </tr>  
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/v1.6.0/libindy/src/api/payments.rs#L505">
            Modifies Indy request by adding information how to pay fees for this transaction
            according to selected payment method
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_add_request_fees(
        command_handle: i32,
        wallet_handle: i32,
        submitter_did: *const c_char,
        req_json: *const c_char,
        inputs_json: *const c_char,
        outputs_json: *const c_char,
        cb: fn(command_handle_: i32,
               err: ErrorCode,
               req_with_fees_json: *const c_char,
               payment_method: *const c_char))
      </pre>
    </td>
    <td>
<pre>
Left the same but the format of outputs has been 
changed to:
[{
  recipient: <str>, // payment address of recipient
  amount: <int>, // amount
  extra: <str>, // optional data
}]
      </pre>
    </td>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/v1.6.0/libindy/src/api/payments.rs#L585">
            Parses response for Indy request with fees
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_parse_response_with_fees(
        command_handle: i32,
        payment_method: *const c_char,
        resp_json: *const c_char,
        cb: fn(command_handle_: i32,
               err: ErrorCode,
               utxo_json: *const c_char))
      </pre>
    </td>
    <td>
    <pre>
indy_parse_response_with_fees(
        command_handle: i32,
        payment_method: *const c_char,
        resp_json: *const c_char,
        cb: fn(command_handle_: i32,
               err: ErrorCode,
               receipts_json: *const c_char))
<b>NOTE:</b> Format of result value has been changed to:
[{
   receipt: <str>, // receipt that can be used for 
             payment referencing and verification
   recipient: <str>, //payment address of recipient
   amount: <int>, // amount
   extra: <str>, // optional data from payment transaction
}]
      </pre>
    </td>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/v1.6.0/libindy/src/api/payments.rs#L632">
             Builds Indy request for getting sources list for payment address
             according to this payment method
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_build_get_utxo_request(
        command_handle: i32,
        wallet_handle: i32,
        submitter_did: *const c_char,
        payment_address: *const c_char,
        cb: fn(command_handle_: i32,
               err: ErrorCode,
               get_utxo_txn_json: *const c_char,
               payment_method: *const c_char))
      </pre>
    </td>
    <td>
    <pre>
indy_build_get_sources_request(
        command_handle: i32,
        wallet_handle: i32,
        submitter_did: *const c_char,
        payment_address: *const c_char,
        cb: fn(command_handle_: i32,
               err: ErrorCode,
               get_sources_txn_json: *const c_char,
               payment_method: *const c_char))
    </pre>
<b>NOTE:</b> Function and result value have been renamed.
    </td>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/v1.6.0/libindy/src/api/payments.rs#L683">
             Parses response for Indy request for getting sources list
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_parse_get_utxo_response(
        command_handle: i32,
        payment_method: *const c_char,
        resp_json: *const c_char,
        cb: fn(command_handle_: i32,
               err: ErrorCode,
               utxo_json: *const c_char))
      </pre>
    </td>
    <td>
    <pre>
indy_parse_get_sources_response(
        command_handle: i32,
        payment_method: *const c_char,
        resp_json: *const c_char,
        cb: fn(command_handle_: i32,
               err: ErrorCode,
               sources_json: *const c_char))
<b>NOTE:</b> Function and result value have been renamed.
<b>NOTE:</b> sources have the following format:
[{
   source: <str>, // source input
   paymentAddress: <str>, //payment address for this source
   amount: <int>, // amount
   extra: <str>, // optional data from payment transaction
}]
      </pre>
    </td>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/v1.6.0/libindy/src/api/payments.rs#L733">
             Builds Indy request for doing payment according to this payment method
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_build_payment_req(
        command_handle: i32,
        wallet_handle: i32,
        submitter_did: *const c_char,
        inputs_json: *const c_char,
        outputs_json: *const c_char,
        cb: fn(command_handle_: i32,
               err: ErrorCode,
               payment_req_json: *const c_char,
               payment_method: *const c_char))
      </pre>
    </td>
    <td>
<pre>
Left the same but the format of outputs has been 
changed to:
[{
  recipient: <str>, // payment address of recipient
  amount: <int>, // amount
  extra: <str>, // optional data
}]
      </pre>
    </td>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/v1.6.0/libindy/src/api/payments.rs#L805">
             Builds Indy request for doing payment according to this payment method
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_parse_payment_response(
            command_handle: i32,
            payment_method: *const c_char,
            resp_json: *const c_char,
            cb: fn(command_handle_: i32,
                   err: ErrorCode,
                   utxo_json: *const c_char))
      </pre>
    </td>
    <td>
      <pre>
indy_parse_payment_response(
            command_handle: i32,
            payment_method: *const c_char,
            resp_json: *const c_char,
            cb: fn(command_handle_: i32,
                   err: ErrorCode,
                   receipts_json: *const c_char))
<b>NOTE:</b> Format of result value has been changed to:
[{
   receipt: <str>, // receipt that can be used for 
             payment referencing and verification
   recipient: <str>, //payment address of recipient
   amount: <int>, // amount
   extra: <str>, // optional data from payment transaction
}]
      </pre>
    </td>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/v1.6.0/libindy/src/api/payments.rs#L855">
             Builds Indy request for doing tokens minting according to this payment method
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_build_mint_req(
        command_handle: i32,
        wallet_handle: i32,
        submitter_did: *const c_char,
        outputs_json: *const c_char,
        cb: fn(command_handle_: i32,
               err: ErrorCode,
               mint_req_json: *const c_char,
               payment_method: *const c_char))
      </pre>
    </td>
    <td>
<pre>
Left the same but the format of outputs has been 
changed to:
[{
  recipient: <str>, // payment address of recipient
  amount: <int>, // amount
  extra: <str>, // optional data
}]
      </pre>
    </td>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/v1.6.0/libindy/src/api/payments.rs#L1065">
             Builds Indy request for information to verify the payment receipt
        </a>
    </th>
  </tr>
  <tr>
    <td><b>NEW</b></td>
    <td>
      <pre>
indy_build_verify_payment_req(
        command_handle: i32,
        wallet_handle: i32,
        submitter_did: *const c_char,
        receipt: *const c_char,
        cb: fn(command_handle_: i32,
               err: ErrorCode,
               verify_txn_json: *const c_char,
               payment_method: *const c_char))
      </pre>
    </td>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/v1.6.0/libindy/src/api/payments.rs#L1114">
             Parses Indy response with information to verify receipt
        </a>
    </th>
  </tr>
  <tr>
    <td><b>NEW</b></td>
    <td>
      <pre>
indy_parse_verify_payment_response(
        command_handle: i32,
        payment_method: *const c_char,
        resp_json: *const c_char,
        cb: fn(command_handle_: i32,
               err: ErrorCode,
               txn_json: *const c_char))
      </pre>
    </td>
  </tr>
</table>


### Pool API

<table>
  <tr>
    <th>v1.5.0 - Pool API</th>
    <th>v1.6.0 - Pool API</th>
  </tr>  
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/v1.6.0/libindy/src/api/pool.rs#L59">
            Opens pool ledger and performs connecting to pool nodes
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_open_pool_ledger(command_handle: i32,
                      config_name: *const c_char,
                      config: *const c_char,
                      cb: fn(xcommand_handle: i32,
                           err: ErrorCode,
                           pool_handle: i32))
      </pre>
    </td>
    <td>
<pre>
Left the same but the format of config has been changed to:
{
   "timeout": int (optional), timeout for network request (in sec).
   "extended_timeout": int (optional), extended timeout for network request (in sec).
   "preordered_nodes": array<string> - (optional), names of nodes which will 
       have a priority during request sending:
       ["name_of_1st_prior_node",  "name_of_2nd_prior_node", .... ]
       Note: Not specified nodes will be placed in a random way.
}
      </pre>
    </td>
  </tr>
</table>

## Libindy 1.6.0 to 1.6.1 migration Guide

The Libindy 1.6.1 release contains fixes that don't affect API functions. 
Most of them relate to pool connection performance.

## Libindy 1.6.1 to 1.6.2 migration Guide

### Wallet API 1.6.2

Updated wallet `credentials` to accept the additional parameter `key_derivation_method`.

This parameter provides the ability to use different crypto algorithms for wallet master key derivation.

Wallet credentials json has the following format:
```
 {
   "key": string, Passphrase used to derive wallet master key
   "storage_credentials": optional<object> Credentials for wallet storage. Storage type defines set of supported keys.
                          Can be optional if storage supports default configuration.
                          For 'default' storage type should be empty.
   "key_derivation_method": optional<string> algorithm to use for master key derivation:
                          ARGON2I_MOD (used by default)
                          ARGON2I_INT - less secured but faster 
 }
```

<table>
  <tr>  
    <th>v1.6.1 - Wallet API</th>
    <th>v1.6.2 - Wallet API</th>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/v1.6.2/libindy/src/api/wallet.rs#L142">
            Create a new secure wallet
        </a>
    </th>
  <tr>
    <td>
      <pre>
indy_create_wallet(command_handle: i32,
                   config: *const c_char,
                   credentials: *const c_char,
                   cb: fn(xcommand_handle: i32,
                          err: ErrorCode))
      </pre>
    </td>
    <td>
      <b>Note:</b> Left the same but format of <i>credentials</i> parameter was changed. 
      The current format is described above.
    </td>
  </tr>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/v1.6.2/libindy/src/api/wallet.rs#L208">
            Open the wallet
        </a>
    </th>
  <tr>
    <td>
      <pre>
indy_open_wallet(command_handle: i32,
                 config: *const c_char,
                 credentials: *const c_char,
                 cb: fn(xcommand_handle: i32,
                                      err: ErrorCode,
                                      handle: i32))
      </pre>
    </td>
    <td>
      <b>Note:</b> Left the same but format of <i>credentials</i> parameter was changed. 
      The current format is described above.
    </td>
  </tr>
    <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/v1.6.2/libindy/src/api/wallet.rs#L443">
            Deletes created wallet
        </a>
    </th>
  <tr>
    <td>
      <pre>
indy_delete_wallet(command_handle: i32,
                   config: *const c_char,
                   credentials: *const c_char,
                   cb: fn(xcommand_handle: i32, 
                          err: ErrorCode))
      </pre>
    </td>
    <td>
      <b>Note:</b> Left the same but format of <i>credentials</i> parameter was changed. 
      The current format is described above.
    </td>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/v1.6.2/libindy/src/api/wallet.rs#L296">
            Exports opened wallet
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_export_wallet(
                command_handle: i32,
                wallet_handle: i32,
                export_config: *const c_char,
                cb: fn(xcommand_handle: i32,
                       err: ErrorCode))
        </pre>
    </td>
    <td>
      <b>Note:</b> Format of <i>export_config</i> parameter was changed to.
      <pre>
{
 "path": <string>, Path of the file that contains exported wallet content
 "key": <string>, Key or passphrase used for wallet export key derivation.
 "key_derivation_method": optional<string> Algorithm to use for wallet export key derivation:
                          ARGON2I_MOD - derive secured export key (used by default)
                          ARGON2I_INT - derive secured export key (less secured but faster)
}
      </pre>
    </td>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/v1.6.2/libindy/src/api/wallet.rs#L328">
            Import wallet
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_import_wallet(
            command_handle: i32,
            config: *const c_char,
            credentials: *const c_char,
            import_config_json: *const c_char,
            cb: fn(xcommand_handle: i32, 
                   err: ErrorCode)
        </pre>
    </td>
    <td>
      <b>Note:</b> Left the same but format of <i>credentials</i> parameter was changed. 
      The current format is described above.
    </td>
  </tr>
</table>

### Ledger API 1.6.2

<table>
  <tr>
    <th>v1.6.1 - Ledger API</th>
    <th>v1.6.2 - Ledger API</th>
  </tr>  
  <tr>
    <th colspan="2">
        <a https://github.com/hyperledger/indy-sdk/blob/v1.6.2/libindy/src/api/ledger.rs#L122">
            Send action to particular nodes of validator pool
        </a>
    </th>
  </tr>
  <tr>
    <td><b>NEW</b></td>
    <td>
      <pre>
indy_submit_action(
               command_handle: i32,
               pool_handle: i32,
               request_json: *const c_char,
               nodes: *const c_char,
               timeout: i32,
               cb: fn(xcommand_handle: i32,
                      err: ErrorCode,
                      request_result_json: *const c_char))
      </pre>
    </td>
  </tr>
  <tr>
    <th colspan="2">
        <a https://github.com/hyperledger/indy-sdk/blob/v1.6.2/libindy/src/api/ledger.rs#L1148">
            Builds a POOL_UPGRADE request
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_build_pool_upgrade_request(
            command_handle: i32,
            submitter_did: *const c_char,
            name: *const c_char,
            version: *const c_char,
            action: *const c_char,
            sha256: *const c_char,
            timeout: i32,
            schedule: *const c_char,
            justification: *const c_char,
            reinstall: bool,
            force: bool,
            cb: fn(xcommand_handle: i32,
                   err: ErrorCode,
                   request_json: *const c_char))
      </pre>
    </td>
    <td>
      <pre>
indy_build_pool_upgrade_request(
                command_handle: i32,
                submitter_did: *const c_char,
                name: *const c_char,
                version: *const c_char,
                action: *const c_char,
                sha256: *const c_char,
                timeout: i32,
                schedule: *const c_char,
                justification: *const c_char,
                reinstall: bool,
                force: bool,
                package: *const c_char,
                cb: fn(xcommand_handle: i32,
                       err: ErrorCode,
                       request_json: *const c_char))
      </pre>
      <b>Note:</b> Added <i>package</i> parameter that allows to specify package to be upgraded
    </td>
  </tr>
</table>

## Libindy 1.6.2 to 1.6.3 migration Guide

### Wallet API 1.6.3

* Added separate API function `indy_generate_wallet_key` to generate a random wallet master key.
* Updated `key_derivation_method` parameter of wallet `credentials` to accept the addition type - `RAW`. 
  By using this type, the result of `indy_generate_wallet_key` can be passed as a wallet master key (key derivation will be skipped).
* Fixed **typo** in the naming of key derivation methods - **ARGON** instead of **ARAGON**. 

Wallet credentials json has the following format:
```
 {
   "key": string, Key or passphrase used for wallet key derivation.
                  Look to key_derivation_method param for information about supported key derivation methods.
   "storage_credentials": optional<object> Credentials for wallet storage. Storage type defines set of supported keys.
                          Can be optional if storage supports default configuration.
                          For 'default' storage type should be empty.
   "key_derivation_method": optional<string> Algorithm to use for wallet key derivation:
                          ARGON2I_MOD - derive secured wallet master key (used by default)
                          ARGON2I_INT - derive secured wallet master key (less secured but faster)
                          RAW - raw wallet key master provided (skip derivation).
                                RAW keys can be generated with indy_generate_wallet_key call 
 }
```

<table>
  <tr>  
    <th>v1.6.2 - Wallet API</th>
    <th>v1.6.3 - Wallet API</th>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/v1.6.3/libindy/src/api/wallet.rs#L538">
            Generate wallet master key.
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <b>NEW</b>
    </td>
    <td>
      <pre>
indy_generate_wallet_key(command_handle: i32,
                         config: *const c_char,
                         cb: fn(xcommand_handle: i32,
                                err: ErrorCode,
                                key: *const c_char))
        </pre>
    </td>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/v1.6.3/libindy/src/api/wallet.rs#L142">
            Create a new secure wallet
        </a>
    </th>
  <tr>
    <td>
      <pre>
indy_create_wallet(command_handle: i32,
                   config: *const c_char,
                   credentials: *const c_char,
                   cb: fn(xcommand_handle: i32,
                          err: ErrorCode))
      </pre>
    </td>
    <td>
      <b>Note:</b> Left the same but format of <i>credentials</i> parameter was changed. 
      The current format is described above.
    </td>
  </tr>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/v1.6.3/libindy/src/api/wallet.rs#L208">
            Open the wallet
        </a>
    </th>
  <tr>
    <td>
      <pre>
indy_open_wallet(command_handle: i32,
                 config: *const c_char,
                 credentials: *const c_char,
                 cb: fn(xcommand_handle: i32,
                                      err: ErrorCode,
                                      handle: i32))
      </pre>
    </td>
    <td>
      <b>Note:</b> Left the same but format of <i>credentials</i> parameter was changed. 
      The current format is described above.
    </td>
  </tr>
    <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/v1.6.3/libindy/src/api/wallet.rs#L443">
            Deletes created wallet
        </a>
    </th>
  <tr>
    <td>
      <pre>
indy_delete_wallet(command_handle: i32,
                   config: *const c_char,
                   credentials: *const c_char,
                   cb: fn(xcommand_handle: i32, 
                          err: ErrorCode))
      </pre>
    </td>
    <td>
      <b>Note:</b> Left the same but format of <i>credentials</i> parameter was changed. 
      The current format is described above.
    </td>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/v1.6.3/libindy/src/api/wallet.rs#L296">
            Exports opened wallet
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_export_wallet(
                command_handle: i32,
                wallet_handle: i32,
                export_config: *const c_char,
                cb: fn(xcommand_handle: i32,
                       err: ErrorCode))
        </pre>
    </td>
    <td>
      <b>Note:</b> Format of <i>export_config</i> parameter was changed to.
      <pre>
{
 "path": <string>, Path of the file that contains exported wallet content
 "key": <string>, Key or passphrase used for wallet export key derivation.
 "key_derivation_method": optional<string> Algorithm to use for wallet export key derivation:
                          ARGON2I_MOD - derive secured export key (used by default)
                          ARGON2I_INT - derive secured export key (less secured but faster)
                          RAW - raw wallet key master provided (skip derivation).
}
      </pre>
    </td>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/v1.6.3/libindy/src/api/wallet.rs#L328">
            Import wallet
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_import_wallet(
            command_handle: i32,
            config: *const c_char,
            credentials: *const c_char,
            import_config_json: *const c_char,
            cb: fn(xcommand_handle: i32, 
                   err: ErrorCode)
        </pre>
    </td>
    <td>
      <b>Note:</b> Left the same but format of <i>credentials</i> parameter was changed. 
      The current format is described above.
    </td>
  </tr>
</table>

### Ledger API 1.6.3

<table>
  <tr>
    <th>v1.6.2 - Ledger API</th>
    <th>v1.6.3 - Ledger API</th>
  </tr>  
  <tr>
    <th colspan="2">
        <a https://github.com/hyperledger/indy-sdk/blob/v1.6.3/libindy/src/api/ledger.rs#L898">
            Builds a NODE request
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_build_node_request(
            command_handle: i32,
            submitter_did: *const c_char,
            target_did: *const c_char,
            data: *const c_char,
            cb: fn(xcommand_handle: i32,
                   err: ErrorCode,
                   request_json: *const c_char))
      </pre>
    </td>
    <td>
Left the same but the additional optional field <b>blskey_pop</b> has been added in data json.
    </td>
  </tr>
</table>

## Libindy 1.6.3 to 1.6.4 migration Guide

The Libindy 1.6.4 release contains fixes that don't affect API functions. 

## Libindy 1.6.4 to 1.6.5 migration Guide

The Libindy 1.6.5 release contains changes that related with Ledger API and Payment API but doesn't break them.
Parameter `submitter_did` set as the optional field for:
* Ledger API `indy_build_get_*` functions (except `indy_build_get_validator_info_request`).
* all functions in Payment API.

## Libindy 1.6.5 to 1.6.6 migration Guide

The Libindy 1.6.6 release contains fixes that don't affect API functions. 

## Libindy 1.6.6 to 1.6.7 migration Guide

* Supported hexadecimal seed. 
* Removed TGB role from `indy_build_nym_request`.