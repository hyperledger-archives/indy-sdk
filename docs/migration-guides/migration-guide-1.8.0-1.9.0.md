<!-- markdownlint-disable MD033 -->

# Libindy 1.8 to 1.9 migration Guide

This document is written for developers using Libindy to provide necessary information and
to simplify their transition to Libindy 1.9 from Libindy 1.8. If you are using older Libindy
version you can check migration guides history:

* [Libindy 1.3 to 1.4 migration](https://github.com/hyperledger/indy-sdk/blob/v1.4.0/doc/migration-guide.md)
* [Libindy 1.4 to 1.5 migration](https://github.com/hyperledger/indy-sdk/blob/v1.5.0/doc/migration-guide-1.4.0-1.5.0.md)
* [Libindy 1.5 to 1.6 migration](https://github.com/hyperledger/indy-sdk/blob/v1.6.0/doc/migration-guide-1.5.0-1.6.0.md)
* [Libindy 1.6 to 1.7 migration](https://github.com/hyperledger/indy-sdk/blob/v1.7.0/doc/migration-guide-1.6.0-1.7.0.md)
* [Libindy 1.7 to 1.8 migration](https://github.com/hyperledger/indy-sdk/blob/v1.8.0/doc/migration-guide-1.7.0-1.8.0.md)

## Table of contents

* [Notes](#notes)
* [Libindy 1.8 to 1.8.9 migration](#libindy-18-to-190-migration-guide)
    * [Ledger API](#libindy-api)
    * [Cache API](#cache-api)
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

## Libindy 1.8 to 1.9.0 migration Guide

### Ledger API

Due to legal nuances Indy network should support flow to receive explicit confirmation from any 
transaction author that he accepts the following reality. 
The ledger is public and immutable and by writing data to the ledger the user will not be able
to exercise the right to be forgotten, so no personal data should be published to the ledger. 

So the set of new function were added to Libindy API to support work with `Transaction Author Agreement` concept [introduced on the Ledger](https://github.com/hyperledger/indy-node/blob/master/design/txn_author_agreement.md).
This guarantees that every write transaction author agree that the information they submit to the ledger meets the requirements outlined by ledger governance.   
   
#### Changes

<table>
    <tr>  
      <th>v1.8.3 - Ledger API</th>
      <th>v1.9.0 - Ledger API</th>
    </tr>
    <tr>
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.9.0/libindy/src/api/ledger.rs#L2001">
              Adds a new version of Transaction Author Agreement to the ledger
          </a>
      </th>
    <tr>
    <tr>
      <td>
          <b>NEW</b>
      </td>
      <td>
      <pre>
indy_build_txn_author_agreement_request(
                        command_handle: CommandHandle,
                        submitter_did: *const c_char,
                        text: *const c_char,
                        version: *const c_char,
                        cb: fn(command_handle_: CommandHandle,
                               err: ErrorCode,
                               request_json: *const c_char))
      </pre>
      </td>
    </tr>
    <tr>
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.9.0/libindy/src/api/ledger.rs#L2055">
              Gets a specific Transaction Author Agreement from the ledger
          </a>
      </th>
    <tr>
    <tr>
      <td>
          <b>NEW</b>
      </td>
      <td>
      <pre>
indy_build_get_txn_author_agreement_request(
                        command_handle: CommandHandle,
                        submitter_did: *const c_char,
                        data: *const c_char,
                        cb: fn(command_handle_: CommandHandle,
                               err: ErrorCode,
                               request_json: *const c_char))
      </pre>
      </td>
    </tr>
    <tr>
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.9.0/libindy/src/api/ledger.rs#L2113">
              Adds new acceptance mechanisms for transaction author agreement
          </a>
      </th>
    <tr>
    <tr>
      <td>
          <b>NEW</b>
      </td>
      <td>
      <pre>
indy_build_acceptance_mechanisms_request(
                        submitter_did: *const c_char,
                        aml: *const c_char,
                        version: *const c_char,
                        aml_context: *const c_char,
                        cb: fn(command_handle_: CommandHandle,
                               err: ErrorCode,
                               request_json: *const c_char))
      </pre>
      </td>
    </tr>    
    <tr>
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.9.0/libindy/src/api/ledger.rs#L2184">
              Get acceptance mechanisms from the ledger
          </a>
      </th>
    <tr>
    <tr>
      <td>
          <b>NEW</b>
      </td>
      <td>
      <pre>
indy_build_get_acceptance_mechanisms_request(
                        submitter_did: *const c_char,
                        timestamp: i64,
                        version: *const c_char,
                        cb: fn(command_handle_: CommandHandle,
                               err: ErrorCode,
                               request_json: *const c_char))
      </pre>
      </td>
    </tr>
    <tr>
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.9.0/libindy/src/api/ledger.rs#L2242">
              Appends transaction author agreement acceptance data to a request
          </a>
      </th>
    <tr>
    <tr>
      <td>
          <b>NEW</b>
      </td>
      <td>
      <pre>
indy_append_txn_author_agreement_acceptance_to_request(
                        request_json: *const c_char,
                        text: *const c_char,
                        version: *const c_char,
                        taa_digest: *const c_char,
                        mechanism: *const c_char,
                        time: u64,
                        cb: fn(command_handle_: CommandHandle,
                               err: ErrorCode,
                               request_with_meta_json: *const c_char))
      </pre>
      </td>
    </tr>
</table>

<table>
    <tr>  
      <th>v1.8.3 - Payment API</th>
      <th>v1.9.0 - Payment API</th>
    </tr>
    <tr>
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.9.0/libindy/src/api/payment.rs#L855">
              Append payment extra JSON with TAA acceptance data
          </a>
      </th>
    <tr>
    <tr>
      <td>
          <b>NEW</b>
      </td>
      <td>
      <pre>
indy_prepare_payment_extra_with_acceptance_data(
                        extra_json: *const c_char,
                        text: *const c_char,
                        version: *const c_char,
                        taa_digest: *const c_char,
                        mechanism: *const c_char,
                        time: u64,
                        cb: fn(command_handle_: CommandHandle,
                               err: ErrorCode,
                               extra_with_acceptance: *const c_char))
      </pre>
      </td>
    </tr>
</table>

#### Sample
```
acc_mech_request = indy_build_acceptance_mechanisms_request(...)
indy_sign_and_submit_request(..., acc_mech_request)

txn_author_agrmnt_request = indy_build_txn_author_agreement_request(...)
indy_sign_and_submit_request(..., txn_author_agrmnt_request)

nym_request = indy_build_nym_request(...)
nym_req_with_taa_acceptance = indy_append_txn_author_agreement_acceptance_to_request(nym_request, ...)
indy_sign_and_submit_request(..., nym_req_with_taa_acceptance)
```

### Cache API

Currently whenever credential definitions and/or schemas is needed, it is being fetched from the ledger.
This operation may last multiple seconds and is slowing down usage of credentials.
Caching also enables usage of anoncreds in areas where user do not have internet coverage.

The set of new *Experimental* functions were added to Libindy API to achieve the following goals:
* allow users to cache credential definitions and schemas.
* enables purging of old (not needed more) data.

<table>
    <tr>  
      <th>v1.8.3 - Cache API</th>
      <th>v1.9.0 - Cache API</th>
    </tr>
    <tr>
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.9.0/libindy/src/api/cache.rs#L12">
              Gets credential definition json data for specified credential definition id
          </a>
      </th>
    <tr>
    <tr>
      <td>
          <b>NEW</b>
      </td>
      <td>
          <pre>
indy_get_cred_def(command_handle: i32,
                  pool_handle: PoolHandle,
                  wallet_handle: WalletHandle,
                  submitter_did: *const c_char,
                  id: *const c_char,
                  options_json: *const c_char,
                  cb: Option<extern fn(xcommand_handle: i32,
                                       err: ErrorCode,
                                       cred_def_json: *const c_char)>)
          </pre>
      </td>
    </tr>
    <tr>
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.9.0/libindy/src/api/cache.rs#L72">
              Gets schema json data for specified schema id.
          </a>
      </th>
    <tr>
    <tr>
      <td>
          <b>NEW</b>
      </td>
      <td>
          <pre>
indy_get_schema(command_handle: i32,
                pool_handle: PoolHandle,
                wallet_handle: WalletHandle,
                submitter_did: *const c_char,
                id: *const c_char,
                options_json: *const c_char,
                cb: Option<extern fn(xcommand_handle: i32,
                                     err: ErrorCode,
                                     schema_json: *const c_char)>)
          </pre>
      </td>
    </tr>
    <tr>
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.9.0/libindy/src/api/cache.rs#L135">
              Purge credential definition cache
          </a>
      </th>
    <tr>
    <tr>
      <td>
          <b>NEW</b>
      </td>
      <td>
          <pre>
indy_purge_cred_def_cache(
                wallet_handle: WalletHandle,
                options_json: *const c_char,
                cb: Option<extern fn(command_handle_: i32,
                                     err: ErrorCode)>)
          </pre>
      </td>
    </tr>
    <tr>
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.9.0/libindy/src/api/cache.rs#L180">
              Purge schema cache
          </a>
      </th>
    <tr>
    <tr>
      <td>
          <b>NEW</b>
      </td>
      <td>
          <pre>
indy_purge_schema_cache(
                wallet_handle: WalletHandle,
                options_json: *const c_char,
                cb: Option<extern fn(command_handle_: IndyHandle,
                                     err: ErrorCode)>)
          </pre>
      </td>
    </tr>
</table>

### Anoncreds API

Updated behavior of `indy_verifier_verify_proof` function to check restrictions on requested predicates during validation of proof.
