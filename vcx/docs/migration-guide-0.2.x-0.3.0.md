# LibVCX migration guide from 0.2.x to 0.3.0

## A Developer Guide for LibVCX migration

This document is written for developers using LibVCX to provide necessary information and
to simplify their transition to LibVCX 0.3 from LibVCX 0.2.x.

* [Notes](#notes)
* [API]()
    * [VCX API](#vcx-api)
* [Libvcx 0.3.0 to 0.3.1 migration](#libvcx-030-to-031-migration-guide)
* [Libvcx 0.3.1 to 0.3.2 migration](#libvcx-030-to-031-migration-guide)

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

#### VCX API

<table>
    <tr>  
      <th>v0.2.x - VCX API</th>
      <th>v0.3.0 - VCX API</th>
    </tr>
    <tr> 
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.9.0/vcx/libvcx/src/vcx.rs#L245">
              Retrieve author agreement and acceptance mechanisms set on the Ledger
          </a>
      </th>
    </tr>
    <tr>
      <td>
        <b>NEW</b>
      </td>
      <td>
        <pre>
vcx_get_ledger_author_agreement(
    command_handle: u32,
    cb: Option&lt;fn(xcommand_handle: u32, 
                  err: u32, 
                  author_agreement: *const c_char)&gt;) -> u32
        </pre>  
      </td>
    </tr>
    <tr> 
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.9.0/vcx/libvcx/src/vcx.rs#L287">
              Set some accepted agreement as active
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
vcx_set_active_txn_author_agreement_meta(text: *const c_char,
                                         version: *const c_char,
                                         hash: *const c_char,
                                         acc_mech_type: *const c_char,
                                         time_of_acceptance: u64) -> u32
        </pre>  
      </td>
    </tr>
  </table>


#### Sample
```
agrement = vcx_get_ledger_author_agreement(...)
agrement = json.parse(agrement)
vcx_set_active_txn_author_agreement_meta(text, version, null, acc_mech_type, tome)
vcx_schema_create(...)
```

## Libvcx 0.3.0 to 0.3.1 migration Guide

#### Changes

The set of new similar functions was added to Libvcx library to provide a way of manually updating a state of Libvcx objects (connection, credential, proof).
These functions check the message any state change and update the state attribute. 

<table>
    <tr>  
      <th>v0.3.0 - Libvcx API</th>
      <th>v0.3.1 - Libvcx API</th>
    </tr>
    <tr>
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.3.1/vcx/libvcx/src/api/connection.rs#L357">
              Checks the message any connection state change and updates the the state attribute.
          </a>
      </th>
    <tr>
    <tr>
      <td>
          <b>NEW</b>
      </td>
      <td>
<pre>vcx_connection_update_state_with_message(command_handle: u32,
                                              connection_handle: u32,
                                              message: *const c_char,
                                              cb: Option<extern fn(xcommand_handle: u32, 
                                                                   err: u32, 
                                                                   state: u32)>) -> u32</pre>
      </td>
    </tr>
    <tr>
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.3.1/vcx/libvcx/src/api/issuer_credential.rs#L208">
              Checks and updates the state of issuer credential based on the given message
          </a>
      </th>
    <tr>
    <tr>
      <td>
          <b>NEW</b>
      </td>
      <td>
<pre>vcx_issuer_credential_update_state_with_message(command_handle: u32,
                                                     credential_handle: u32,
                                                     message: *const c_char,
                                                     cb: Option<extern fn(xcommand_handle: u32, 
                                                                          err: u32, 
                                                                          state: u32)>) -> u32</pre>
      </td>
    </tr>
    <tr>
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.3.1/vcx/libvcx/src/api/proof.rs#L156">
              Checks for any state change from the given message and updates the proof state attribute
          </a>
      </th>
    <tr>
    <tr>
      <td>
          <b>NEW</b>
      </td>
      <td>
<pre>vcx_proof_update_state_with_message(command_handle: u32,
                                         proof_handle: u32,
                                         message: *const c_char,
                                         cb: Option<extern fn(xcommand_handle: u32, 
                                                              err: u32, 
                                                              state: u32)>) -> u32</pre>
      </td>
    </tr>
</table>

## Libvcx 0.3.1 to 0.3.2 migration Guide

The Libvcx 0.3.2 release contains fixes that don't affect API functions. 