# LibVCX migration guide from 0.1.x to 0.2.0

## A Developer Guide for LibVCX migration

This document is written for developers using LibVCX to provide necessary information and
to simplify their transition to LibVCX 0.3 from LibVCX 0.2.x.

* [Notes](#notes)
* [API]()
    * [VCX API](#vcx-api)

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