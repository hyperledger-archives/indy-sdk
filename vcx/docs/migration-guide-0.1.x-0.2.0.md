# LibVCX migration guide from 0.1.x to 0.2.0

## A Developer Guide for LibVCX migration

This document is written for developers using LibVCX to provide necessary information and
to simplify their transition to LibVCX 0.1.x from LibVCX 0.2.

* [Notes](#notes)
* [API]()
    * [Credential Definition API](#credential-definition-api-mapping)
    * [Wallet API](#wallet-api-mapping)
    * [Issuer Credential API](#issuer-credetial-api-mapping)
    * [Proof API](#proof-api-mapping)
    
### Notes

In the following tables, there are mappings for each LibVCX API part of how 0.1.2097319 functionality maps to 0.2.0. 

Functions from version 0.1.2097319 are listed in the left column, and the equivalent 0.2.0 function is placed in the right column. 

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
          get_payment_txn(handle: u32) -> Result&lt;PaymentTxn, CredDefError&gt;
        </pre>  
      </td>
      <td>
        <pre>
          get_cred_def_payment_txn(handle: u32) -> Result&lt;PaymentTxn, CredDefError&gt;
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
          get_rev_reg_id(handle: u32) -> Result&lt;Option&lt;String&gt;, CredDefError&gt;
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
          get_tails_file(handle: u32) -> Result&lt;Option&lt;String&gt;, CredDefError&gt;
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
          get_rev_reg_def(handle: u32) -> Result&lt;Option&lt;String&gt;, CredDefError&gt;
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
          get_rev_reg_def_payment_txn(handle: u32) -> Result&lt;Option&lt;String&gt;, CredDefError&gt;
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
          get_rev_reg_delta_payment_txn(handle: u32) -> Result&lt;Option&lt;String&gt;, CredDefError&gt;
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
          find_handle(cred_def_id: &str) -> Result&lt;u32, CredDefError&gt;
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
          vcx_issuer_create_credential(command_handle: u32,
                                                source_id: *const c_char,
                                                cred_def_handle: u32,
                                                issuer_did: *const c_char,
                                                credential_data: *const c_char,
                                                credential_name: *const c_char,
                                                price: *const c_char,
                                                cb: Option&lt;extern fn(xcommand_handle: u32, err: u32, credential_handle: u32)&gt;) -> u32
        </pre>  
      </td>
      <td>
        <pre>
            Changed the format of param credential_data -- was "{"state":["UT"]}", now "{"state":"UT"}"
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
           vcx_issuer_revoke_credential(command_handle: u32,
                                        credential_handle: u32,
                                        cb: Option&lt;extern fn(xcommand_handle: u32, err: u32)&gt;) -> u32
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
          revoke_credential(handle: u32) -> Result&lt;(), u32&gt;
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
                vcx_proof_create(command_handle: u32,
                                 source_id: *const c_char,
                                 requested_attrs: *const c_char,
                                 requested_predicates: *const c_char,
                                 name: *const c_char,
                                 cb: Option&lt;extern fn(xcommand_handle: u32, err: u32, proof_handle: u32)&gt;) -> u32
            </pre> 
        </td>
        <td>
            <pre>
                vcx_proof_create(command_handle: u32,
                                 source_id: *const c_char,
                                 requested_attrs: *const c_char,
                                 requested_predicates: *const c_char,
                                 revocation_interval: *const c_char,
                                 name: *const c_char,
                                 cb: Option&lt;extern fn(xcommand_handle: u32, err: u32, proof_handle: u32)&gt;) -> u32
            </pre> 
        </td>
    </tr>