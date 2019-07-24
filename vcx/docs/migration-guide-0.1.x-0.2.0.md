# LibVCX migration guide from 0.1.x to 0.2.0

## A Developer Guide for LibVCX migration

This document is written for developers using LibVCX to provide necessary information and
to simplify their transition to LibVCX 0.1.x from LibVCX 0.2.

* [Notes](#notes)
* [API]()
    * [Credential Definition API](#credential-definition-api)
    * [Wallet API](#wallet-api)
    * [Issuer Credential API](#issuer-credetial-api)
    * [Proof API](#proof-api)
    * [logger API](#logger-api)
* [Libvcx 0.2.0 to 0.2.1 migration](#libvcx-020-to-021-migration-guide)

### Notes

In the following tables, there are mappings for each LibVCX API part of how 0.1.x functionality maps to 0.2.0. 

Functions from version 0.1.x are listed in the left column, and the equivalent 0.2.0 function is placed in the right column. 

* If some function had been added, the word 'NEW' would be placed in the left column.
* If some function had been deleted, the word 'DELETED' would be placed in the right column.
* If some function had been changed, the current format would be placed in the right column.
* If some function had not been changed, the symbol '=' would be placed in the right column.
* To get more details about current format of a function click on the description above it.
* Bellow are signatures of functions in LibVCX C API.
 The params of <b>cb</b> (except command_handle and err) will be result values of the similar function in any LibVCX wrapper.
 
### API

#### Credential Definition API

<table>
    <tr>  
      <th>v0.1.x - Credential Definition API</th>
      <th>v0.2.0 - Credential Definition API</th>
    </tr>
    <tr> 
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.7.0/vcx/libvcx/src/credential_def.rs#L201">
              Get Payment Txn for Credential definition
          </a>
      </th>
    </tr>
    <tr>
      <td>
        <pre>
get_payment_txn(handle: u32) 
            -> Result&lt;PaymentTxn, CredDefError&gt;
        </pre>  
      </td>
      <td>
        <pre>
get_cred_def_payment_txn(handle: u32) 
            -> Result&lt;PaymentTxn, CredDefError&gt;
        </pre>
      </td>
    </tr>
    <tr> 
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.7.0/vcx/libvcx/src/credential_def.rs#L213">
              Get Revocation Registry ID
          </a>
      </th>
    </tr>
    <tr>
      <td>
        <b>
          NEW
        </b>  
      </td>
      <td>
        <pre>
get_rev_reg_id(handle: u32) 
            -> Result&lt;Option&lt;String&gt;, CredDefError&gt;
        </pre>
      </td>
    </tr>
    <tr> 
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.7.0/vcx/libvcx/src/credential_def.rs#L219">
              Get Tails File
          </a>
      </th>
    </tr>
    <tr>
      <td>
        <b>
          NEW
        </b>  
      </td>
      <td>
        <pre>
get_tails_file(handle: u32) 
            -> Result&lt;Option&lt;String&gt;, CredDefError&gt;
        </pre>
      </td>
    </tr>
    <tr> 
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.7.0/vcx/libvcx/src/credential_def.rs#L225">
              Get Revocation Registry Definition
          </a>
      </th>
    </tr>
    <tr>
      <td>
        <b>
          NEW
        </b>  
      </td>
      <td>
        <pre>
get_rev_reg_def(handle: u32) 
            -> Result&lt;Option&lt;String&gt;, CredDefError&gt;
        </pre>
      </td>
    </tr>
    <tr> 
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.7.0/vcx/libvcx/src/credential_def.rs#L231">
              Get Revocation Registry Definition Payment Txn
          </a>
      </th>
    </tr>
    <tr>
      <td>
        <b>
          NEW
        </b>  
      </td>
      <td>
        <pre>
get_rev_reg_def_payment_txn(handle: u32) 
            -> Result&lt;Option&lt;String&gt;, CredDefError&gt;
        </pre>
      </td>
    </tr>
    <tr> 
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.7.0/vcx/libvcx/src/credential_def.rs#L238">
              Get Revocation Registry Delta Payment Txn
          </a>
      </th>
    </tr>
    <tr>
      <td>
        <b>
          NEW
        </b>  
      </td>
      <td>
        <pre>
get_rev_reg_delta_payment_txn(handle: u32) 
            -> Result&lt;Option&lt;String&gt;, CredDefError&gt;
        </pre>
      </td>
    </tr>
    <tr> 
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.7.0/vcx/libvcx/src/credential_def.rs#L251">
              Find Handle By Revocation Registry Id
          </a>
      </th>
    </tr>
    <tr>
      <td>
        <b>
          NEW
        </b>  
      </td>
      <td>
        <pre>
find_handle(cred_def_id: &str) 
            -> Result&lt;u32, CredDefError&gt;
        </pre>
      </td>
    </tr>
  </table>
  
#### Wallet API

There are no actual API changes, but there is one new parameter available for `vcx_provision_agent` and `vcx_agent_provision_async` `config` variable. It is `wallet_type` - it can be used to enable different wallet types.

#### Issuer Credential API

<table>
    <tr>  
      <th>v0.1.x - Issuer Credential API</th>
      <th>v0.2.0 - Issuer Credential API</th>
    </tr>
    <tr> 
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.7.0/vcx/libvcx/src/api/issuer_credential.rs#L513">
              Revoke Credential
          </a>
      </th>
    </tr>
    <tr>
      <td>
        <pre>
vcx_issuer_create_credential(
    command_handle: u32,
    source_id: *const c_char,
    cred_def_handle: u32,
    issuer_did: *const c_char,
    credential_data: *const c_char,
    credential_name: *const c_char,
    price: *const c_char,
    cb: Option&lt;fn(xcommand_handle: u32, 
                  err: u32, 
                  credential_handle: u32)&gt;) -> u32
        </pre>  
      </td>
      <td>
        <pre>
Changed the format of param credential_data. 
was "{"state":["UT"]}", now "{"state":"UT"}"
        </pre>
      </td>
    </tr>
    <tr> 
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.7.0/vcx/libvcx/src/api/issuer_credential.rs#L513">
              Revoke Credential
          </a>
      </th>
    </tr>
    <tr>
      <td>
        <b>
          NEW
        </b>  
      </td>
      <td>
        <pre>
vcx_issuer_revoke_credential(
        command_handle: u32,
        credential_handle: u32,
        cb: Option&lt;fn(xcommand_handle: u32, 
                      err: u32)&gt;) -> u32
        </pre>
      </td>
    </tr>
    <tr> 
      <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/v1.7.0/vcx/libvcx/src/issuer_credential.rs#L606">
           Revoke Credential (Rust API)
        </a>
      </th>
    </tr>
    <tr>
      <td>
        <b>
          NEW
        </b>  
      </td>
      <td>
        <pre>
revoke_credential(handle: u32) 
                -> Result&lt;(), u32&gt;
        </pre>
      </td>
    </tr>
</table>

#### Proof API

<table>
    <tr>
        <th>v0.1.x - Proof API</th>
        <th>v0.2.0 - Proof API</th>
    </tr>
    <tr>
        <th colspan="2">
            <a href="">
                Create Proof
            </a>
        </th>
    </tr>
    <tr>
        <td>
            <pre>
vcx_proof_create(
        command_handle: u32,
        source_id: *const c_char,
        requested_attrs: *const c_char,
        requested_predicates: *const c_char,
        name: *const c_char,
        cb: Option&lt;extern fn(xcommand_handle: u32, 
                             err: u32, 
                             proof_handle: u32)&gt;) -> u32
            </pre> 
        </td>
        <td>
            <pre>
vcx_proof_create(
        command_handle: u32,
        source_id: *const c_char,
        requested_attrs: *const c_char,
        requested_predicates: *const c_char,
        revocation_interval: *const c_char,
        name: *const c_char,
        cb: Option&lt;extern fn(xcommand_handle: u32, 
                             err: u32, 
                             proof_handle: u32)&gt;) -> u32
            </pre> 
        </td>
    </tr>
    <tr>
      <td>
        <pre>
vcx_disclosed_proof_generate_proof(
             proof_handle: u32,
             selected_credentials: *const c_char,
             self_attested_attrs: *const c_char,
             cb: Option<extern fn(xcommand_handle: u32, 
                                  err: u32)>) -> u32
        </pre>  
      </td>
      <td>
        <pre>
Changed the format of param selected_credentials. 
was "{
    "attrs":{
        "attr_key": "cred_info":{}
    }
}", 
now "{
    "attrs":{
        "attr_key":{
            "credential":{"cred_info":{},"tails_file": Optional(string)}
        }
    }
}"
        </pre>
      </td>
    </tr>
</table>
    
    
### Logger API

The main purpose of this API is to forward logs of libvcx and wrappers to its consumers. 
It is needed if you consume libvcx as a `.so` or `.dll` - so you can forward logs from libvcx to your logging framework.
You don't need this endpoints if you use libvcx through the wrapper -- in Java and Python wrappers they are already forwarded to `slf4j` for Java, `log` crate for Rust and default logging facade for python.     
libvcx sets the same log function for libindy as well.

<table>
    <tr>  
      <th>v0.1.x - Logger API</th>
      <th>v0.2.0 - Logger API</th>
    </tr>
    <tr>
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.7.0/vcx/libvcx/src/api/logger.rs#L41">
              Set custom logger implementation
          </a>
      </th>
    <tr>
    <tr>
      <td>
          <b>NEW</b>
      </td>
      <td>
          <pre>
              vcx_set_logger(context: *const c_void,
                             enabled: Option<fn(context: *const c_void,
                                                level: u32,
                                                target: *const c_char) -> bool>,
                             log: Option<fn(context: *const c_void,
                                             level: u32,
                                             target: *const c_char,
                                             message: *const c_char,
                                             module_path: *const c_char,
                                             file: *const c_char,
                                             line: u32)>,
                              flush: Option<fn(context: *const c_void)>) -> ErrorCode
          </pre>
      </td>
    </tr>
    <tr>
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.7.0/vcx/libvcx/src/api/logger.rs#L21">
              Set default logger implementation.
          </a>
      </th>
    <tr>
    <tr>
      <td>
          <b>NEW</b>
      </td>
      <td>
          <pre>
              vcx_set_default_logger(pattern: *const c_char) -> ErrorCode
          </pre>
      </td>
    </tr>
    <tr>
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.7.0/vcx/libvcx/src/api/logger.rs#L76">
              Get the currently used logger.
          </a>
      </th>
    <tr>
    <tr>
      <td>
          <b>NEW</b>
      </td>
      <td>
          <pre>
              vcx_get_logger(context_p: *mut *const c_void,
                             enabled_cb_p: *mut Option<fn(context: *const c_void,
                                                          level: u32,
                                                          target: *const c_char) -> bool>,
                             log_cb_p: *mut Option<fn(context: *const c_void,
                                                      level: u32,
                                                      target: *const c_char,
                                                      message: *const c_char,
                                                      module_path: *const c_char,
                                                      file: *const c_char,
                                                      line: u32)>,
                             flush_cb_p: *mut Option<fn(context: *const c_void)>)
          </pre>
      </td>
    </tr>
</table>

## Libvcx 0.2.0 to 0.2.1 migration Guide

The Libvcx 0.2.1 release contains fixes that don't affect API functions. 

## Libvcx 0.2.1 to 0.2.2 migration Guide

The Libvcx 0.2.2 release contains fixes that don't affect API functions. 

## Libvcx 0.2.2 to 0.2.3 migration Guide

#### Changes
* Migrated Libvcx to *failure* crate for better handling and error chaining.
* Added synchronous `vcx_get_current_error` API function that returns details for last occurred error. 
* Updated Libvcx wrappers for automatic getting error details:
    * Python - added `sdk_error_full_message`, `sdk_error_cause` and `sdk_error_backtrace` fields to `VcxError` object.
    * Java - added `sdkMessage`, `sdkFullMessage`, `sdkCause`  and `sdkBacktrace` fields to `VcxException`.
    * Objective-C - added `error`, `message`, `cause`, `backtrace` fields to `userInfo` dictionary in `NSError` object.
* Updated Libvcx to support community A2A protocol. 
Added `protocol_type` field to VCX provisioning config with indicates A2A message format will be used.
    * `1.0` means the current protocol.
    * `2.0` means community (IN PROGRESS) protocol which in the current state includes draft implementation of the following HIPEs:
        * [Message Types](https://github.com/hyperledger/indy-hipe/tree/master/text/0021-message-types)
        * [Message Threading](https://github.com/hyperledger/indy-hipe/tree/master/text/0027-message-id-and-threading)
        * [Wire Message](https://github.com/hyperledger/indy-hipe/tree/master/text/0028-wire-message-format).

* Bugfixes

<table>
    <tr>  
      <th>v0.2.2 - Libvcx API</th>
      <th>v0.2.3 - Libvcx API</th>
    </tr>
    <tr>
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.8.2/vcx/libvcx/src/api/vcx.rs#L272">
              Get details for last occurred error.
          </a>
      </th>
    <tr>
    <tr>
      <td>
          <b>NEW</b>
      </td>
      <td>
<pre>vcx_get_current_error(error_json_p: *mut *const c_char)</pre>
      </td>
    </tr>
</table>

## Libvcx 0.2.2 to 0.2.3 migration Guide

The Libvcx 0.2.3 release contains fixes that don't affect API functions. 