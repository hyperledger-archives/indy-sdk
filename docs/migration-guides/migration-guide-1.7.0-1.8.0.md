<!-- markdownlint-disable MD033 -->

# Libindy 1.7 to 1.8 migration Guide

This document is written for developers using Libindy to provide necessary information and
to simplify their transition to Libindy 1.8 from Libindy 1.7. If you are using older Libindy
version you can check migration guides history:

* [Libindy 1.3 to 1.4 migration](https://github.com/hyperledger/indy-sdk/blob/v1.4.0/doc/migration-guide.md)
* [Libindy 1.4 to 1.5 migration](https://github.com/hyperledger/indy-sdk/blob/v1.5.0/doc/migration-guide-1.4.0-1.5.0.md)
* [Libindy 1.5 to 1.6 migration](https://github.com/hyperledger/indy-sdk/blob/v1.6.0/doc/migration-guide-1.5.0-1.6.0.md)
* [Libindy 1.6 to 1.7 migration](https://github.com/hyperledger/indy-sdk/blob/v1.7.0/doc/migration-guide-1.6.0-1.7.0.md)

## Table of contents

* [Notes](#notes)
* [Libindy 1.7 to 1.8.0 migration](#libindy-17-to-180-migration-guide)
    * [Libindy API](#libindy-api)
    * [Crypto API](#crypto-api)
    * [Ledger API](#ledger-api)

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

## Libindy 1.7 to 1.8.0 migration Guide

### Libindy API

The main purpose of this changes is providing a way of getting additional error information like message and backtrace.

#### Changes
* Migrated Libindy to *failure* crate for better handling and error chaining.
* Added synchronous `indy_get_current_error` API function that returns details for last occurred error. 
* Updated Libindy wrappers for automatic getting error details:
    * Python - added `message` and `indy_backtrace` fields to `IndyError` object.
    * Java - added `sdkBacktrace` field to `IndyException`. Libindy `error message` set as the main for `IndyException`.
    * NodeJS - added `indyMessage` and `indyBacktrace` fields to `IndyError` object.
    * Rust - changed type of returning value from enum `ErrorCode` on structure `IndyError` with `error_code`, `message`, `indy_backtrace` fields.
    * Objective-C - added `message` and `indy_backtrace` fields to `userInfo` dictionary in `NSError` object. 
* Updated Indy-Cli to show Libindy error message in some cases.

<table>
    <tr>  
      <th>v1.7.0 - Libindy API</th>
      <th>v1.8.0 - Libindy API</th>
    </tr>
    <tr>
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.8.0/libindy/src/api/mod.rs#L266">
              Get details for last occurred error.
          </a>
      </th>
    <tr>
    <tr>
      <td>
          <b>NEW</b>
      </td>
      <td>
<pre>indy_get_current_error(
            error_json_p: *mut *const c_char)</pre>
      </td>
    </tr>
    <tr>
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.8.0/libindy/src/api/mod.rs#L239">
              Set libindy runtime configuration
          </a>
      </th>
    <tr>
    <tr>
      <td>
<pre>indy_set_runtime_config(
        config: *const c_char) -> ErrorCode</pre>
      </td>
      <td>
        <b>Note:</b> Format of <i>config</i> parameter was changed. Current format is:
<pre>
{
    "crypto_thread_pool_size": Optional[int] - 
        size of thread pool 
    "collect_backtrace": Optional<[bool] - 
        whether errors backtrace should be collected
}
</pre>
      </td>
    </tr>
</table>

### Crypto API

The main purpose of changes is the support of *Wire Messages* described in [AMES HIPE](https://github.com/hyperledger/indy-hipe/pull/43)

*Note*: New functions are *Experimental*. 

<table>
    <tr>  
      <th>v1.7.0 - Crypto API</th>
      <th>v1.8.0 - Crypto API</th>
    </tr>
    <tr>
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.7.0/libindy/src/api/crypto.rs#L565">
              Packs a message by encrypting the message and serializes it in a JWE-like format
          </a>
      </th>
    <tr>
    <tr>
      <td>
          <b>NEW</b>
      </td>
      <td>
          <pre>
indy_pack_message(command_handle: i32,
                 wallet_handle: i32,
                 message: *const u8,
                 message_len: u32,
                 receiver_keys: *const c_char,
                 sender: *const c_char,
                 cb: Option<extern fn(xcommand_handle: i32,
                                      err: ErrorCode,
                                      jwe_data: *const u8, 
                                      jwe_len: u32)>)
          </pre>
      </td>
    </tr>
    <tr>
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.7.0/libindy/src/api/crypto.rs#L673">
              Unpacks a JWE-like formatted message outputted by indy_pack_message
          </a>
      </th>
    <tr>
    <tr>
      <td>
          <b>NEW</b>
      </td>
      <td>
          <pre>
indy_unpack_message(command_handle: i32,
                    wallet_handle: i32,
                    jwe_data: *const u8,
                    jwe_len: u32,
                    cb: Option<extern fn(xcommand_handle: i32,
                                         err: ErrorCode,
                                         res_json_data : *const u8,
                                         res_json_len : u32)>)
          </pre>
      </td>
    </tr>
    <tr>
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.7.0/libindy/src/api/crypto.rs#L371">
              Encrypts a message by anonymous-encryption scheme
          </a>
      </th>
    <tr>
    <tr>
      <td>
          <pre>
indy_crypto_anon_crypt(
            command_handle: i32,
            recipient_vk: *const c_char,
            msg_data: *const u8,
            msg_len: u32,
            cb: Option<extern fn(command_handle_: i32,
                                 err: ErrorCode,
                                 encrypted_msg: *const u8,
                                 encrypted_len: u32)>)
          </pre>
      </td>
      <td>
          <b>DEPRECATED</b><br>
          Use `indy_pack_message` instead
      </td>
    </tr>
    <tr>
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.7.0/libindy/src/api/crypto.rs#L432">
              Decrypt a message by authenticated-encryption scheme
          </a>
      </th>
    <tr>
    <tr>
      <td>
          <pre>
indy_crypto_auth_decrypt(
            command_handle: i32,
             wallet_handle: i32,
             recipient_vk: *const c_char,
             encrypted_msg: *const u8,
             encrypted_len: u32,
             cb: Option<extern fn(command_handle_: IndyHandle,
                                  err: ErrorCode,
                                  sender_vk: *const c_char,
                                  msg_data: *const u8,
                                  msg_len: u32)>)
          </pre>
      </td>
      <td>
          <b>DEPRECATED</b><br>
          Use `indy_unpack_message` instead
      </td>
    </tr>
</table>

### Ledger API
* Added `NETWORK_MONITOR` to list of acceptable values for `role` parameter in `indy_build_nym_request` API function.
* Implemented automatic filtering of outdated responses based on comparison of local time with latest transaction ordering time.

## Libindy 1.8.1 to 1.8.2 migration Guide

<table>
  <tr>
    <th>v1.8.1 - Ledger API</th>
    <th>v1.8.2 - Ledger API</th>
  </tr>  
  <tr>
    <th colspan="2">
        <a https://github.com/hyperledger/indy-sdk/blob/v1.8.2/libindy/src/api/ledger.rs#L1838">
            Builds a AUTH_RULE request to change authentication rules for a ledger transaction 
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <b>NEW</b>
    </td>
    <td>
      <pre>
indy_build_auth_rule_request(command_handle: CommandHandle,
                             submitter_did: *const c_char,
                             txn_type: *const c_char,
                             action: *const c_char,
                             field: *const c_char,
                             old_value: *const c_char,
                             new_value: *const c_char,
                             constraint: *const c_char,
                             cb: fn(command_handle_: CommandHandle,
                                    err: ErrorCode,
                                    request_json: *const c_char))
      </pre>
    </td>
  </tr>
  <tr>
    <th colspan="2">
        <a https://github.com/hyperledger/indy-sdk/blob/v1.8.2/libindy/src/api/ledger.rs#L1927">
            Builds a GET_AUTH_RULE request to get authentication rules for ledger
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <b>NEW</b>
    </td>
    <td>
      <pre>
indy_build_get_auth_rule_request(command_handle: CommandHandle,
                                 submitter_did: *const c_char,
                                 txn_type: *const c_char,
                                 action: *const c_char,
                                 field: *const c_char,
                                 old_value: *const c_char,
                                 new_value: *const c_char,
                                 cb: fn(command_handle_: CommandHandle,
                                        err: ErrorCode,
                                        request_json: *const c_char))
      </pre>
    </td>
  </tr>
</table>

## Libindy 1.8.2 to 1.8.3 migration Guide

Updated behavior of `indy_build_auth_rule_request` and `indy_build_get_auth_rule_request` API functions:
*  `new_value` can be empty string for `ADD` action.
*  `new_value` can be null for `EDIT` action.
*  `old_value` is skipped during transaction serialization for `ADD` action. 