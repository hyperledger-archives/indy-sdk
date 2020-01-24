<!-- markdownlint-disable MD033 -->

# Libindy 1.10 to 1.11 migration Guide

This document is written for developers using Libindy to provide necessary information and
to simplify their transition to Libindy 1.11 from Libindy 1.10. If you are using older Libindy
version you can check migration guides history:

* [Libindy 1.3 to 1.4 migration](https://github.com/hyperledger/indy-sdk/blob/v1.4.0/doc/migration-guide.md)
* [Libindy 1.4 to 1.5 migration](https://github.com/hyperledger/indy-sdk/blob/v1.5.0/doc/migration-guide-1.4.0-1.5.0.md)
* [Libindy 1.5 to 1.6 migration](https://github.com/hyperledger/indy-sdk/blob/v1.6.0/doc/migration-guide-1.5.0-1.6.0.md)
* [Libindy 1.6 to 1.7 migration](https://github.com/hyperledger/indy-sdk/blob/v1.7.0/doc/migration-guide-1.6.0-1.7.0.md)
* [Libindy 1.7 to 1.8 migration](https://github.com/hyperledger/indy-sdk/blob/v1.8.0/doc/migration-guide-1.7.0-1.8.0.md)
* [Libindy 1.8 to 1.9 migration](https://github.com/hyperledger/indy-sdk/blob/v1.9.0/doc/migration-guide-1.8.0-1.9.0.md)
* [Libindy 1.9 to 1.10 migration](https://github.com/hyperledger/indy-sdk/blob/v1.10.0/doc/migration-guide-1.9.0-1.10.0.md)

## Table of contents

* [Notes](#notes)
* [Libindy 1.10 to 1.11 migration](#libindy-110-to-111-migration-guide)
    * [Payment API](#payment-api)
    * [Anoncreds API](#anoncreds-api)
    * [Ledger API](#ledger-api)
* [Libindy 1.11.0 to 1.11.1 migration](#libindy-1110-to-1111-migration-guide)
    * [Ledger API 1.11.1](#ledger-api-1111)
    * [Anoncreds API 1.11.1](#ledger-api-1111)

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

## Libindy 1.10 to 1.11 migration Guide

### Payment API

#### Changes

<table>
    <tr>  
      <th>v1.10.0 - Payment API</th>
      <th>v1.11.0 - Payment API</th>
    </tr>
    <tr>
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.11.0/libindy/src/api/payments.rs#L1355">
              Signs a message with a payment address.
          </a>
      </th>
    <tr>
    <tr>
      <td>
          <b>NEW</b>
      </td>
      <td>
      <pre>
indy_sign_with_address(command_handle: CommandHandle,
                       wallet_handle: WalletHandle,
                       address: *const c_char,
                       message_raw: *const u8,
                       message_len: u32,
                       cb: fn(command_handle_: CommandHandle,
                              err: ErrorCode,
                              signature_raw: *const u8,
                              signature_len: u32))
      </pre>
      </td>
    </tr>
    <tr>
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.11.0/libindy/src/api/payments.rs#L1412">
              Verify a signature with a payment address.
          </a>
      </th>
    <tr>
    <tr>
      <td>
          <b>NEW</b>
      </td>
      <td>
      <pre>
indy_verify_with_address(command_handle: CommandHandle,
                         address: *const c_char,
                         message_raw: *const u8,
                         message_len: u32,
                         signature_raw: *const u8,
                         signature_len: u32,
                         cb: fn(command_handle_: CommandHandle,
                                err: ErrorCode,
                                result: bool))
      </pre>
      </td>
    </tr>
    <tr>
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.11.0/libindy/src/api/payments_v2.rs#L11">
              Builds Indy request for getting sources list for payment address according to this payment method.
          </a>
      </th>
    <tr>
    <tr>
      <td>
          <b>NEW</b>
      </td>
      <td>
      <pre>
indy_build_get_payment_sources_with_from_request(command_handle: CommandHandle,
                                                 wallet_handle: WalletHandle,
                                                 submitter_did: *const c_char,
                                                 payment_address: *const c_char,
                                                 from: i64,
                                                 cb: fn(command_handle_: CommandHandle,
                                                        err: ErrorCode,
                                                        get_sources_txn_json: *const c_char,
                                                        payment_method: *const c_char))
      </pre>
      </td>
    </tr> 
    <tr>
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.11.0/libindy/src/api/payments_v2.rs#L67">
              Parses response for Indy request for getting sources list.
          </a>
      </th>
    <tr>
    <tr>
      <td>
          <b>NEW</b>
      </td>
      <td>
      <pre>
indy_parse_get_payment_sources_with_from_response(command_handle: CommandHandle,
                                                  payment_method: *const c_char,
                                                  resp_json: *const c_char,
                                                  cb: fn(command_handle_: CommandHandle,
                                                         err: ErrorCode,
                                                         sources_json: *const c_char,
                                                         next: i64))
      </pre>
      </td>
    </tr> 
    <tr>
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.11.0/libindy/src/api/payments.rs#L676">
              Builds Indy request for getting sources list for payment address according to this payment method.
          </a>
      </th>
    <tr>
    <tr>
      <td>
      <pre>
indy_build_get_payment_sources_request(command_handle: CommandHandle,
                                       wallet_handle: WalletHandle,
                                       submitter_did: *const c_char,
                                       payment_address: *const c_char,
                                       cb: fn(command_handle_: CommandHandle,
                                              err: ErrorCode,
                                              get_sources_txn_json: *const c_char,
                                              payment_method: *const c_char))
      </pre>
      </td>
      <td>
          <b>DEPRECATED</b>
      </td>  
    </tr>
    <tr>## Notes

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

      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.11.0/libindy/src/api/payments.rs#L729">
              Parses response for Indy request for getting sources list.
          </a>
      </th>
    <tr>
    <tr>
      <td>
      <pre>
indy_parse_get_payment_sources_response(command_handle: CommandHandle,
                                       payment_method: *const c_char,
                                       resp_json: *const c_char,
                                       cb: fn(command_handle_: CommandHandle,
                                              err: ErrorCode,
                                              sources_json: *const c_char))
      </pre>
      </td>
      <td>
          <b>DEPRECATED</b>
      </td>  
    </tr>
    <tr>
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.11.0/libindy/src/api/payments.rs#L1285">
              Gets request requirements (with minimal price) correspondent to specific auth rule in case the requester can perform this action.
          </a>
      </th>
    <tr>
    <tr>
      <td>
          <b>NEW</b>
      </td>
      <td>
      <pre>
indy_get_request_info(command_handle: CommandHandle,
                      get_auth_rule_response_json: *const c_char,
                      requester_info_json: *const c_char,
                      fees_json: *const c_char,
                      cb: fn(command_handle_: CommandHandle,
                             err: ErrorCode,
                             request_info_json: *const c_char))
      </pre>
      </td>
    </tr>
</table>

**Note** that `indy_register_payment_method` Payment API function was updated to accept:
* an additional callbacks correspondent to new functions to sign/verify a message with a payment address.
* callbacks correspondent to the new functions for getting payment sources with pagination support instead of deprecated.

### Anoncreds API

* Updated behavior of `indy_prover_create_proof` to create revocation proof based on `non_revoked` timestamps within a proof request.
Now only `primary` proof can be built if `non_revoked` intervals were not requested by a verifier.
An example test can be found here: `indy-sdk/libindy/tests/anoncreds_demos.rs/anoncreds_works_for_requested_proof_with_revocation_but_provided_primary_only`

* Added new Libindy API function `indy_generate_nonce` to generate a nonce of the size recommended for usage within a proof request. 
An example test can be found here: `indy-sdk/libindy/tests/anoncreds_demos.rs/anoncreds_works_for_single_issuer_single_prover`

### Ledger API

* Updated `indy_append_txn_author_agreement_acceptance_to_request` Libindy function to discard the time portion of `acceptance time` on appending TAA metadata into request. 
It was done cause too much time precision can lead to privacy risk.

    *NOTE* that if the following points are met:
    - Indy Pool consists of nodes with version less 1.9.2
    - Transaction Author Agreement is set on the Pool
    
    Requests to the Pool will fail during the day TAA was set.

* Updated `constraint` parameter of `indy_build_auth_rule_request` Libindy Ledger API function to accept new optional `off_ledger_signature` field that specifies if a signature of unknown ledger `DID` is allowed for an action performing (false by default). 

* Added new function `indy_append_request_endorser` to append Endorser to an existing request. 
It allows writing transactions to the ledger with preserving an original author but by different Endorser.
An example flow can be found [here](../configuration.md)
An example test can be found here: `indy-sdk/libindy/tests/ledger.rs/indy_send_request_by_endorser_works`

## Libindy 1.11.0 to 1.11.1 migration Guide

### Ledger API 1.11.1

* Extended `config` parameter of `indy_open_pool_ledger` function to accept `number_read_nodes` value. 
This value set the number of nodes to send read requests.

### Anoncreds API 1.11.1

The main idea of changes performed in Anoncreds API is to provide a way to rotate the key of a Credential Definition stored into the wallet.

**WARNING**: Rotating the credential definitional keys will result in making all credentials issued under the previous keys unverifiable.

<table>
    <tr>  
      <th>v1.11.0 - Anoncreds API</th>
      <th>v1.11.1 - Anoncreds API</th>
    </tr>
    <tr>
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.11.1/libindy/src/api/anoncreds.rs#L206">
              Generate temporary credential definitional keys for an existing one (owned by the caller of the library).
          </a>
      </th>
    <tr>
    <tr>
      <td>
          <b>NEW</b>
      </td>
      <td>
          <pre>
indy_issuer_rotate_credential_def_start(command_handle: i32,
                                        wallet_handle: WalletHandle,
                                        cred_def_id: *const c_char,
                                        config_json: *const c_char,
                                           cb: Option<extern fn(xcommand_handle: i32,
                                                                err: ErrorCode, 
                                                                cred_def_json: *const c_char)>)
          </pre>
      </td>
    </tr>
    <tr>
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.11.1/libindy/src/api/anoncreds.rs#L270">
              Apply temporary keys as main for an existing Credential Definition (owned by the caller of the library).
          </a>
      </th>
    <tr>
    <tr>
      <td>
          <b>NEW</b>
      </td>
      <td>
          <pre>
indy_issuer_rotate_credential_def_apply(command_handle: i32,
                                        wallet_handle: WalletHandle,
                                        cred_def_id: *const c_char,
                                        cb: Option<extern fn(xcommand_handle: i32,
                                                             err: ErrorCode)>)
          </pre>
      </td>
    </tr>
</table> 

#### Workflow
```
cred_def_id, cred_def_json = indy_issuer_create_and_store_credential_def(...)
indy_sign_and_submit_request(cred_def_json)
...
...
temp_cred_def_json = indy_issuer_rotate_credential_def_start(cred_def_id)
indy_sign_and_submit_request(temp_cred_def_json)
indy_issuer_rotate_credential_def_apply(cred_def_id)

```