# Libindy migration Guide

## A Developer Guide for Libindy migration

There are a lot APIs that have been changed in Libindy 1.4.0.
This document is written for developers using Libindy 1.3.0 to provide necessary information and
to simplify their transition to API of Libindy 1.4.0.

* [Notes](#notes)
* [Api]()
    * [Anoncreds API](#anoncreds-api-mapping)
    * [Ledger API](#ledger-api-mapping)
    * [Signus API](#signus-api-mapping)
    * [Crypto API](#crypto-api-mapping)
    * [Blob Storage API](#blob-storage-api-mapping)
    * [Agent API](#agent-api-mapping)
    * [Pairwise API](#pairwise-api-mapping)
    * [Pool API](#pool-api-mapping)
    * [Wallet API](#wallet-api-mapping)
* [Explore the Code](#explore-the-code)

### Notes

In the following tables, there are mappings for each Libindy API part of how 1.3.0 functionality maps to 1.4.0. 

Functions from v1.3.0 are listed in the left column, and the equivalent 1.4.0 function is placed in the right column. 

* If some function had been added, the word 'NEW' would be placed in the left column.
* If some function had been deleted, the word 'DELETED' would be placed in the right column.
* If some function had been changed, the current format would be placed in the right column.
* If some function had not been changed, the symbol '=' would be placed in the right column.
* To get more details about current format of a function click on the description above it.


### Anoncreds API mapping
Anoncreds API is the most affected part of Libindy. The complete design of Anoncreds can be found [here](https://github.com/hyperledger/indy-sdk/tree/master/doc/design/anoncreds).

There are three main types of changes:
* Improved support of Revocation 
* Changed params of some functions to avoid persisting in wallet intermediate steps entities
* Changed format of some input and output objects such as filter, proof request, credential info and etc to use different identifiers for public entities:
    * Schema - id in the format ```did | marker | name | version``` instead of triple ```name, version, did``` 
    * Credential Definition - id in the format ```did | marker | signatureType | schemaID``` instead of pair ```did, schema_key```
    * Revocation Registry - id in the format ```did | marker | credDefID | revocDefType | revocDefTag``` instead of ```seqNo```

<table>  
  <th>v1.3.0 - Anoncreds API</th>
  <th>v1.4.0 - Anoncreds API</th>
  <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/anoncreds.rs#L15">
            Issuer create Credential Schema
        </a>
    </th>
  </tr>
  <tr>
    <td>
        <b>NEW</b>
    </td>
    <td>
      <pre>
indy_issuer_create_schema(
            command_handle: i32,
            issuer_did: *const c_char,
            name: *const c_char,
            version: *const c_char,
            attrs: *const c_char,
            cb: fn(xcommand_handle: i32, 
                   err: ErrorCode,
                   schema_id: *const c_char, 
                   schema_json: *const c_char))
        </pre>
    </td>
  </tr>
  <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/anoncreds.rs#L73">
            Issuer create Credential Definition for the given Schema
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_issuer_create_and_store_claim_def(
        command_handle: i32,
        wallet_handle: i32,
        issuer_did: *const c_char,
        schema_json: *const c_char,
        signature_type: *const c_char,
        create_non_revoc: bool,
        cb: fn(xcommand_handle: i32, 
               err: ErrorCode,
               claim_def_json: *const c_char))
        </pre>
    </td>
    <td>
      <pre>
indy_issuer_create_and_store_credential_def(
        command_handle: i32,
        wallet_handle: i32,
        issuer_did: *const c_char,
        schema_json: *const c_char,
        tag: *const c_char,
        signature_type: *const c_char,
        config_json: *const c_char,
        cb: fn(xcommand_handle: i32, 
               err: ErrorCode,
               cred_def_id: *const c_char,
               cred_def_json: *const c_char))
        </pre>
      <b>It is IMPORTANT</b> for current Pool version get Schema from Ledger
      with correct seq_no to save backward compatibility before the creation of Credential Definition.
    </td>
  </tr>
  <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/anoncreds.rs#L142">
            Issuer create a new revocation registry for the given Credential Definition 
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_issuer_create_and_store_revoc_reg(
        command_handle: i32,
        wallet_handle: i32,
        issuer_did: *const c_char,
        schema_seq_no: i32,
        max_claim_num: i32,
        cb: fn(xcommand_handle: i32, 
               err: ErrorCode,
               revoc_reg_json: *const c_char))
        </pre>
    </td>
    <td>
      <pre>
indy_issuer_create_and_store_revoc_reg(
        command_handle: i32,
        wallet_handle: i32,
        issuer_did: *const c_char,
        revoc_def_type: *const c_char,
        tag: *const c_char,
        cred_def_id: *const c_char,
        config_json: *const c_char,
        tails_writer_handle: i32,
        cb: fn(xcommand_handle: i32, 
               err: ErrorCode,
               revoc_reg_id: *const c_char,
               revoc_reg_def_json: *const c_char,
               revoc_reg_entry_json: *const c_char))
        </pre>
    </td>
  </tr>
  <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/anoncreds.rs#L230">
            Issuer create credential offer
        </a>
    </th>
  </tr>
  <tr>
    <td>
        <b>NEW</b>
    </td>
    <td>
      <pre>indy_issuer_create_credential_offer(
        command_handle: i32,
        wallet_handle: i32,
        cred_def_id: *const c_char,
        cb: fn(xcommand_handle: i32, 
               err: ErrorCode,
               cred_offer_json: *const c_char))</pre>
      <b>Note</b>: The format of Credential Offer was changed
    </td>
  </tr>
  <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/anoncreds.rs#L280">
            Issuer issue Credential for the given Credential Request
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_issuer_create_claim(
    command_handle: i32,
    wallet_handle: i32,
    claim_req_json: *const c_char,
    claim_json: *const c_char,
    user_revoc_index: i32,
    cb: fn(xcommand_handle: i32, 
           err: ErrorCode,
           revoc_reg_update_json: *const c_char,
           xclaim_json: *const c_char))
        </pre>
    </td>
    <td>
      <pre>
indy_issuer_create_credential(
        command_handle: i32,
        wallet_handle: i32,
        cred_offer_json: *const c_char,
        cred_req_json: *const c_char,
        cred_values_json: *const c_char,
        rev_reg_id: *const c_char,
        blob_storage_reader_handle: i32,
        cb: fn(xcommand_handle: i32, 
               err: ErrorCode,
               cred_json: *const c_char,
               cred_revoc_id: *const c_char,
               revoc_reg_delta_json: *const c_char))
        </pre>
      <b>Note</b>: The format of Credential was changed
    </td>
  </tr>
  <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/anoncreds.rs#L368">
            Issuer revoke a credential
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_issuer_revoke_claim(
    command_handle: i32,
    wallet_handle: i32,
    issuer_did: *const c_char,
    schema_seq_no: i32,
    user_revoc_index: i32,
    cb: fn(xcommand_handle: i32, 
           err: ErrorCode,
           revoc_reg_update_json: *const c_char))
        </pre>
    </td>
    <td>
      <pre>
indy_issuer_revoke_credential(
        command_handle: i32,
        wallet_handle: i32,
        blob_storage_reader_cfg_handle: i32,
        rev_reg_id: *const c_char,
        cred_revoc_id: *const c_char,
        cb: fn(xcommand_handle: i32, 
               err: ErrorCode,
               revoc_reg_delta_json: *const c_char))
        </pre>
    </td>
  </tr>
  <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/anoncreds.rs#L476">
            Issuer merge two revocation registry deltas
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <b>NEW</b>
    </td>
    <td>
      <pre>
indy_issuer_merge_revocation_registry_deltas(
        command_handle: i32,
        rev_reg_delta_json: *const c_char,
        other_rev_reg_delta_json: *const c_char,
        cb: fn(xcommand_handle: i32, 
               err: ErrorCode,
               merged_rev_reg_delta: *const c_char))
        </pre>
    </td>
  </tr>
  <tr> 
    <th colspan="2">
Prover stores a Claim Offer from the given issuer in a secure storage.
   </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_prover_store_claim_offer(
            command_handle: i32,
            wallet_handle: i32,
            claim_offer_json: *const c_char,
            cb: fn(xcommand_handle: i32, 
                   err: ErrorCode))
        </pre>
    </td>
    <td>
      <b>DELETED</b>
    </td>
  </tr>
  <tr> 
    <th colspan="2">
Prover gets all stored Claim Offers
   </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_prover_get_claim_offers(
        command_handle: i32,
        wallet_handle: i32,
        filter_json: *const c_char,
        cb: fn(xcommand_handle: i32, 
               err: ErrorCode,
               claim_offers_json: *const c_char))
        </pre>
    </td>
    <td>
      <b>DELETED</b>
    </td>
  </tr>
  <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/anoncreds.rs#L518">
            Prover creates a Master Secret
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_prover_create_master_secret(
                command_handle: i32,
                wallet_handle: i32,
                master_secret_name: *const c_char,
                cb: fn(xcommand_handle: i32, 
                       err: ErrorCode))
        </pre>
    </td>
    <td>
      <pre>
indy_prover_create_master_secret(
        command_handle: i32,
        wallet_handle: i32,
        master_secret_id: *const c_char,
        cb: fn(xcommand_handle: i32, 
               err: ErrorCode,
               out_master_secret_id: *const c_char))
        </pre>
    </td>
  </tr>
  <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/anoncreds.rs#L559">
            Prover creates a Credential Request for the given Credential Offer
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_prover_create_and_store_claim_req(
            command_handle: i32,
            wallet_handle: i32,
            prover_did: *const c_char,
            claim_offer_json: *const c_char,
            claim_def_json: *const c_char,
            master_secret_name: *const c_char,
            cb: fn(xcommand_handle: i32, 
                   err: ErrorCode,
                   claim_req_json: *const c_char))
        </pre>
    </td>
    <td>
      <pre>
indy_prover_create_credential_req(
        command_handle: i32,
        wallet_handle: i32,
        prover_did: *const c_char,
        cred_offer_json: *const c_char,
        cred_def_json: *const c_char,
        master_secret_id: *const c_char,
        cb: fn(xcommand_handle: i32, 
               err: ErrorCode,
               cred_req_json: *const c_char,
               cred_req_metadata_json: *const c_char))
        </pre>
        <b>Note</b>: The format of Credential Request was changed
    </td>
  </tr>
  <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/anoncreds.rs#L627">
            Prover stores Credential in a secure wallet
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_prover_store_claim(
                command_handle: i32,
                wallet_handle: i32,
                claims_json: *const c_char,
                cb: fn(xcommand_handle: i32, 
                       err: ErrorCode))
        </pre>
    </td>
    <td>
      <pre>
indy_prover_store_credential(
            command_handle: i32,
            wallet_handle: i32,
            cred_id: *const c_char,
            cred_req_json: *const c_char,
            cred_req_metadata_json: *const c_char,
            cred_json: *const c_char,
            cred_def_json: *const c_char,
            rev_reg_def_json: *const c_char,
            cb: fn(xcommand_handle: i32, 
                   err: ErrorCode,
                   out_cred_id: *const c_char))
        </pre>
    </td>
  </tr>
  <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/anoncreds.rs#L689">
            Prover gets human readable claims according to the filter
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_prover_get_claims(
            command_handle: i32,
            wallet_handle: i32,
            filter_json: *const c_char,
            cb: fn(xcommand_handle: i32, 
                   err: ErrorCode,
                   claims_json: *const c_char))
        </pre>
    </td>
    <td>
      <pre>
indy_prover_get_credentials(
        command_handle: i32,
        wallet_handle: i32,
        filter_json: *const c_char,
        cb: fn(xcommand_handle: i32, 
               err: ErrorCode,
               matched_credentials_json: *const c_char))
        </pre>
        <b>Note</b>: The formats of Filter and Matched Credential were changed
    </td>
  </tr>
  <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/anoncreds.rs#L748">
            Prover gets human readable credentials matching the given proof request
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_prover_get_claims_for_proof_req(
            command_handle: i32,
            wallet_handle: i32,
            proof_request_json: *const c_char,
            cb: fn(xcommand_handle: i32, 
                   err: ErrorCode,
                   claims_json: *const c_char))
        </pre>
    </td>
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
        <b>Note</b>: The formats of Proof Request and Matched Credential were changed
    </td>
  </tr>
  <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/anoncreds.rs#L856">
            Prover creates a proof according to the given proof request
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
fn indy_prover_create_proof(
        command_handle: i32,
        wallet_handle: i32,
        proof_req_json: *const c_char,
        requested_claims_json: *const c_char,
        schemas_json: *const c_char,
        master_secret_name: *const c_char,
        claim_defs_json: *const c_char,
        revoc_regs_json: *const c_char,
        cb: fn(xcommand_handle: i32, 
               err: ErrorCode,
               proof_json: *const c_char))
        </pre>
    </td>
    <td>
      <pre>
indy_prover_create_proof(
        command_handle: i32,
        wallet_handle: i32,
        proof_req_json: *const c_char,
        requested_credentials_json: *const c_char,
        master_secret_id: *const c_char,
        schemas_json: *const c_char,
        credential_defs_json: *const c_char,
        rev_states_json: *const c_char,
        cb: fn(xcommand_handle: i32, 
               err: ErrorCode,
               proof_json: *const c_char))
        </pre>
        <b>Note</b>: The formats of Proof Request, Requested Credentials and Proof were changed
    </td>
  </tr>
  <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/anoncreds.rs#L1029">
            Verifier verifies a proof
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_verifier_verify_proof(
            command_handle: i32,
            proof_request_json: *const c_char,
            proof_json: *const c_char,
            schemas_json: *const c_char,
            claim_defs_jsons: *const c_char,
            revoc_regs_json: *const c_char,
            cb: fn(xcommand_handle: i32, 
                   err: ErrorCode,
                   valid: bool))
        </pre>
    </td>
    <td>
      <pre>
indy_verifier_verify_proof(
            command_handle: i32,
            proof_request_json: *const c_char,
            proof_json: *const c_char,
            schemas_json: *const c_char,
            credential_defs_json: *const c_char,
            rev_reg_defs_json: *const c_char,
            rev_regs_json: *const c_char,
            cb: fn(xcommand_handle: i32, 
                   err: ErrorCode,
                   valid: bool))
        </pre>
        <b>Note</b>: The formats of Proof Request and Proof were changed
    </td>
  </tr>
  <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/anoncreds.rs#L1152">
            Create revocation state for a credential in the particular time moment
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <b>NEW</b>
    </td>
    <td>
      <pre>
indy_create_revocation_state(
        command_handle: i32,
        blob_storage_reader_handle: i32,
        rev_reg_def_json: *const c_char,
        rev_reg_delta_json: *const c_char,
        timestamp: u64,
        cred_rev_id: *const c_char,
        cb: fn(xcommand_handle: i32, 
               err: ErrorCode,
               rev_state_json: *const c_char))
        </pre>
    </td>
  </tr>
  <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/anoncreds.rs#L1208">
            Create new revocation state for a credential based on existed
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <b>NEW</b>
    </td>
    <td>
      <pre>
indy_update_revocation_state(
        command_handle: i32,
        blob_storage_reader_handle: i32,
        rev_state_json: *const c_char,
        rev_reg_def_json: *const c_char,
        rev_reg_delta_json: *const c_char,
        timestamp: u64,
        cred_rev_id: *const c_char,
        cb: fn(xcommand_handle: i32,
               err: ErrorCode,
               updated_rev_state_json: *const c_char))
        </pre>
    </td>
  </tr>
</table>


### Blob Storage API mapping
CL revocation schema introduces Revocation Tails entity used to hide information about revoked credential.
Tails are static information that may require huge amount of data and stored outside of Libindy wallet. 
A way how to access tails blobs can be very application specific. 
To access this Libindy 1.4.0 provides new Blob Storage API.

<table>  
  <th colspan="2">v1.4.0 - Blob Storage API</th>
  <tr>
    <td>
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/blob_storage.rs#L12">
            Open Blob Storage reader
        </a>
    </td>
    <td>
      <pre>
indy_open_blob_storage_reader(
                command_handle: i32,
                type_: *const c_char,
                config_json: *const c_char,
                cb: fn(command_handle_: i32, 
                       err: ErrorCode, 
                       handle: i32))
        </pre>
    </td>
  </tr>
  <tr>
    <td>
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/blob_storage.rs#L34">
            Open Blob Storage writer
        </a>
    </td>
    <td>
      <pre>
indy_open_blob_storage_writer(command_handle: i32,
                              type_: *const c_char,
                              config_json: *const c_char,
                              cb: fn(command_handle_: i32,
                                     err: ErrorCode, 
                                     handle: i32))
        </pre>
    </td>
  </tr>
</table>   


### Ledger API mapping
There are four types of changes in Ledger API:
* Added new transaction builders for Revocation support
* Added new transaction builders for Node support
* Added parsers of transaction responses related to Anoncreds API
* Changed format of some transaction builders

<table>  
  <th>v1.3.0 - Ledger API</th>
  <th>v1.4.0 - Ledger API</th>
  <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/ledger.rs#L374">
            Builds a SCHEMA request. Request to add Credential's schema
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_build_schema_request(
            command_handle: i32,
            submitter_did: *const c_char,
            data: *const c_char,
            cb: fn(xcommand_handle: i32, 
                   err: ErrorCode,
                   request_json: *const c_char))
              </pre>
    </td>
    <td>
Left the same but the format of data has been changed to:
<pre>
{
    id: identifier of schema
    attrNames: array of attribute name strings
    name: Schema's name string
    version: Schema's version string,
    ver: version of the Schema json
}
</pre>
    </td>
  </tr>
  <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/ledger.rs#L418">
            Builds a GET_SCHEMA request. Request to get Credential's Schema
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_build_get_schema_request(
            command_handle: i32,
            submitter_did: *const c_char,
            dest: *const c_char,
            data: *const c_char,
            cb: fn(xcommand_handle: i32, 
                   err: ErrorCode,
                   request_json: *const c_char))
              </pre>
    </td>
    <td>
      <pre>
indy_build_get_schema_request(
            command_handle: i32,
            submitter_did: *const c_char,
            id: *const c_char,
            cb: fn(xcommand_handle: i32, 
                   err: ErrorCode,
                   request_json: *const c_char))
              </pre>
    </td>
  </tr>
  <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/ledger.rs#L455">
            Parse a GET_SCHEMA response to get Schema in the format compatible with Anoncreds API
        </a>
    </th>
  </tr>
  <tr>
    <td>
        <b>NEW</b>
    </td>
    <td>
      <pre>
indy_parse_get_schema_response(
            command_handle: i32,
            get_schema_response: *const c_char,
            cb: fn(xcommand_handle: i32, 
                   err: ErrorCode,
                   schema_id: *const c_char,
                   schema_json: *const c_char))
              </pre>
    </td>
  </tr>
  <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/ledger.rs#L497">
            Builds an CRED_DEF request. Request to add a Credential Definition
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_build_claim_def_txn(
        command_handle: i32,
        submitter_did: *const c_char,
        xref: i32,
        signature_type: *const c_char,
        data: *const c_char,
        cb: fn(xcommand_handle: i32, 
               err: ErrorCode,
               request_result_json: *const c_char))
              </pre>
    </td>
    <td>
      <pre>
indy_build_cred_def_request(
        command_handle: i32,
        submitter_did: *const c_char,
        data: *const c_char,
        cb: fn(xcommand_handle: i32, 
               err: ErrorCode,
               request_result_json: *const c_char))
              </pre>
    </td>
  </tr>
  <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/ledger.rs#L546">
            Builds a GET_CRED_DEF request. Request to get a Credential Definition
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_build_get_claim_def_txn(
        command_handle: i32,
        submitter_did: *const c_char,
        xref: i32,
        signature_type: *const c_char,
        origin: *const c_char,
        cb: fn(xcommand_handle: i32, 
               err: ErrorCode,
               request_json: *const c_char))
              </pre>
    </td>
    <td>
      <pre>
indy_build_get_cred_def_request(
        command_handle: i32,
        submitter_did: *const c_char,
        id: *const c_char,
        cb: fn(xcommand_handle: i32, 
               err: ErrorCode,
               request_json: *const c_char))
              </pre>
    </td>
  </tr>
  <tr> 
  </tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/ledger.rs#L584">
            Parse a GET_CRED_DEF response to get Credential Definition in the format compatible with Anoncreds API
        </a>
    </th>
  <tr>
    <td>
        <b>NEW</b>
    </td>
    <td>
      <pre>
indy_parse_get_cred_def_response(
        command_handle: i32,
        get_cred_def_response: *const c_char,
        cb: fn(xcommand_handle: i32, 
               err: ErrorCode,
               cred_def_id: *const c_char,
               cred_def_json: *const c_char))
              </pre>
    </td>
  </tr>
  <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/ledger.rs#L715">
            Builds a POOL_CONFIG request. Request to change Pool's configuration
        </a>
    </th>
  <tr>
    <td>
        <b>NEW</b>
    </td>
    <td>
      <pre>
indy_build_pool_config_request(
            command_handle: i32,
            submitter_did: *const c_char,
            writes: bool,
            force: bool,
            cb: fn(xcommand_handle: i32, 
                   err: ErrorCode,
                   request_json: *const c_char))
              </pre>
    </td>
  </tr>
  <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/ledger.rs#L796">
            Builds a POOL_UPGRADE request. Request to upgrade the Pool (sent by Trustee)
        </a>
    </th>
  </tr>
  <tr>
    <td>
        <b>NEW</b>
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
            cb: fn(xcommand_handle: i32, 
                   err: ErrorCode,
                   request_json: *const c_char))
              </pre>
    </td>
  </tr>
  <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/ledger.rs#L868">
            Builds a REVOC_REG_DEF request. Request to add the definition of revocation registry to an exists Credential Definition
        </a>
    </th>
  </tr>
  <tr>
    <td>
        <b>NEW</b>
    </td>
    <td>
      <pre>
indy_build_revoc_reg_def_request(
        command_handle: i32,
        submitter_did: *const c_char,
        data: *const c_char,
        cb: fn(xcommand_handle: i32, 
               err: ErrorCode,
               rev_reg_def_req: *const c_char))
              </pre>
    </td>
  </tr>
  <tr>  
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/ledger.rs#L920">
            Builds a GET_REVOC_REG_DEF request. Request to get a Revocation Registry Definition 
        </a>
    </th>
  </tr>
  <tr>
    <td>
        <b>NEW</b>
    </td>
    <td>
      <pre>
indy_build_get_revoc_reg_def_request(
            command_handle: i32,
            submitter_did: *const c_char,
            id: *const c_char,
            cb: fn(xcommand_handle: i32, 
                   err: ErrorCode,
                   request_json: *const c_char))
              </pre>
    </td>
  </tr> 
  <tr> 
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/ledger.rs#L958">
            Parse a GET_REVOC_REG_DEF response to get Revocation Registry Definition in the format compatible with Anoncreds API.  </tr>
          </a>
      </th>
  <tr>
    <td>
        <b>NEW</b>
    </td>
    <td>
      <pre>
indy_parse_get_revoc_reg_def_response(
        command_handle: i32,
        get_revoc_reg_def_response: *const c_char,
        cb: fn(xcommand_handle: i32, 
               err: ErrorCode,
               revoc_reg_def_id: *const c_char,
               revoc_reg_def_json: *const c_char))
              </pre>
    </td>
  </tr> 
  <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/ledger.rs#L1008">
            Builds a REVOC_REG_ENTRY request.  Request to add the Revocation Registry Entry record containing the new accumulator value and issued/revoked indices.
        </a>
    </th>
  <tr>
    <td>
        <b>NEW</b>
    </td>
    <td>
      <pre>
indy_build_revoc_reg_entry_request(
            command_handle: i32,
            submitter_did: *const c_char,
            revoc_reg_def_id: *const c_char,
            rev_def_type: *const c_char,
            value: *const c_char,
            cb: fn(xcommand_handle: i32, 
                   err: ErrorCode,
                   request_json: *const c_char))
              </pre>
    </td>
  </tr>
  <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/ledger.rs#L1065">
            Builds a GET_REVOC_REG request. Request to get the accumulated state of the Revocation Registry in a particular time
        </a>
    </th>
  <tr>
    <td>
        <b>NEW</b>
    </td>
    <td>
      <pre>
indy_build_get_revoc_reg_request(
            command_handle: i32,
            submitter_did: *const c_char,
            revoc_reg_def_id: *const c_char,
            timestamp: i64,
            cb: fn(xcommand_handle: i32, 
                   err: ErrorCode,
                   request_json: *const c_char))
              </pre>
    </td>
  </tr>
  <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/ledger.rs#L1106">
            Parse a GET_REVOC_REG response to get Revocation Registry in the format compatible with Anoncreds API
        </a>
    </th>
  <tr>
    <td>
        <b>NEW</b>
    </td>
    <td>
      <pre>
indy_parse_get_revoc_reg_response(
        command_handle: i32,
        get_revoc_reg_response: *const c_char,
        cb: fn(xcommand_handle: i32, 
               err: ErrorCode,
               revoc_reg_def_id: *const c_char,
               revoc_reg_json: *const c_char,
               timestamp: u64))
              </pre>
    </td>
  </tr>
  <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/ledger.rs#L1148">
            Builds a GET_REVOC_REG_DELTA request. Request to get the delta of the accumulated state of the Revocation Registry.
        </a>
    </th>
  </tr>
  <tr>
    <td>
        <b>NEW</b>
    </td>
    <td>
      <pre>
indy_build_get_revoc_reg_delta_request(
        command_handle: i32,
        submitter_did: *const c_char,
        revoc_reg_def_id: *const c_char,
        from: i64,
        to: i64,
        cb: fn(xcommand_handle: i32, 
               err: ErrorCode,
                request_json: *const c_char))
              </pre>
    </td>
  </tr>
  <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/ledger.rs#L1195">
            Parse a GET_REVOC_REG_DELTA response to get Revocation Registry Delta in the format compatible with Anoncreds API.
        </a>
    </th>
  </tr>
  <tr>
    <td>
        <b>NEW</b>
    </td>
    <td>
      <pre>
indy_parse_get_revoc_reg_delta_response(
        command_handle: i32,
        get_revoc_reg_delta_response: *const c_char,
        cb: fn(xcommand_handle: i32, 
               err: ErrorCode,
               revoc_reg_def_id: *const c_char,
               revoc_reg_delta_json: *const c_char,
               timestamp: u64))
              </pre>
    </td>
  </tr>
  <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/ledger.rs#L11">
            Signs and submits request message to validator pool
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_sign_and_submit_request(...)
       </pre>
    </td>
    <td>
      <b>=</b>
    </td>
  </tr>
  <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/ledger.rs#L61">
            Send request message to validator pool
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_submit_request(...)
       </pre>
    </td>
    <td>
      <b>=</b>
    </td>
  </tr>
  <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/ledger.rs#L100">
            Signs request message
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_sign_request(...)
       </pre>
    </td>
    <td>
      <b>=</b>
    </td>
  </tr>
  <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/ledger.rs#L185">
            Builds a NYM request. Request to create a new NYM record for a specific user.
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_build_nym_request(...)
       </pre>
    </td>
    <td>
      <b>=</b>
    </td>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/ledger.rs#L337">
            Builds a GET_NYM request. Request to get information about a DID (NYM).
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_build_get_nym_request(...)
       </pre>
    </td>
    <td>
      <b>=</b>
    </td>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/ledger.rs#L239">
            Builds an ATTRIB request. Request to add attribute to a NYM record.
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_build_attrib_request(...)
       </pre>
    </td>
    <td>
      <b>=</b>
    </td>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/ledger.rs#L288">
            Builds a GET_ATTRIB request. Request to get information about an Attribute for the specified DID.
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_build_get_attrib_request(...)
       </pre>
    </td>
    <td>
      <b>=</b>
    </td>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/ledger.rs#L630">
            Builds a NODE request. Request to add a new node to the pool, or updates existing in the pool.
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_build_node_request(...)
       </pre>
    </td>
    <td>
      <b>=</b>
    </td>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/ledger.rs#L679">
            Builds a GET_TXN request. Request to get any transaction by its seq_no.
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_build_get_txn_request(...)
       </pre>
    </td>
    <td>
      <b>=</b>
    </td>
  </tr>
</table>                                  



### Signus API mapping
The most significant change of this part is renaming Signus API to Did API. 
Furthermore, some functions of Signus API were deleted because the same goals can be achieved by using a combination of others.

<table>  
  <th>v1.3.0 - Signus API</th>
  <th>v1.4.0 - Crypto API</th>
   <tr> 
     <th colspan="2">
       Signs a message
     </th>
   </tr>
   <tr>
     <td>
       <pre>
indy_sign(...)
               </pre>
     </td>
     <td>
       <b>DELETED</b> <span>(use combination of either <i>did.indy_key_for_did</i> or <i>did.indy_key_for_local_did</i> with <i>crypto.indy_crypto_sign</i> instead)</span>
     </td>
   </tr>
   <tr> 
     <th colspan="2">
         Verify a signature
     </th>
   </tr>
   <tr>
     <td>
       <pre>
indy_verify_signature(...)
               </pre>
     </td>
     <td>
       <b>DELETED</b> <span>(use combination of either <i>did.indy_key_for_did</i> or <i>did.indy_key_for_local_did</i> with <i>crypto.indy_crypto_verify</i> instead)</span>
     </td>
   </tr>
   <tr> 
     <th colspan="2">
         Encrypts a message
     </th>
   </tr>
   <tr>
     <td>
       <pre>
indy_encrypt(...)
               </pre>
     </td>
     <td>
       <b>DELETED</b> <span>(use combination of either <i>did.indy_key_for_did</i> or <i>did.indy_key_for_local_did</i> with <i>crypto.indy_crypto_auth_crypt</i> instead)</span>
     </td>
   </tr>
   <tr> 
     <th colspan="2">
         Decrypts a message
     </th>
   </tr>
   <tr>
     <td>
       <pre>
indy_decrypt(...)
               </pre>
     </td>
     <td>
       <b>DELETED</b> <span>(use combination of either <i>did.indy_key_for_did</i> or <i>did.indy_key_for_local_did</i> with <i>crypto.indy_crypto_auth_decrypt</i> instead)</span>
     </td>
   </tr>
   <tr> 
     <th colspan="2">
         Encrypts a message by anonymous-encryption scheme
     </th>
   </tr>
   <tr>
     <td>
       <pre>
indy_encrypt_sealed(...)
               </pre>
     </td>
     <td>
       <b>DELETED</b> <span>(use combination of either <i>did.indy_key_for_did</i> or <i>did.indy_key_for_local_did</i> with <i>crypto.indy_crypto_anon_crypt</i> instead)</span>
     </td>
   </tr>
   <tr> 
     <th colspan="2">
         Decrypts a message by anonymous-encryption scheme
     </th>
   </tr>
   <tr>
     <td>
       <pre>
indy_decrypt_sealed(...)
               </pre>
     </td>
     <td>
       <b>DELETED</b> <span>(use combination of either <i>did.indy_key_for_did</i> or <i>did.indy_key_for_local_did</i> with <i>crypto.indy_crypto_anon_decrypt</i> instead)</span>
     </td>
   </tr>
   <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/did.rs#L460">
            Get info about My DID in format: DID, verkey, metadata
        </a>
    </th>
   </tr>
   <tr>
     <td>
       <b>NEW</b>
     </td>
     <td>
       <pre>
indy_get_my_did_with_meta(command_handle: i32,
                          wallet_handle: i32,
                          my_did: *const c_char,
                          cb: fn(xcommand_handle: i32,
                                 err: ErrorCode,
                                 did_with_meta: *const c_char))
               </pre>
     </td>
   </tr>
   <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/did.rs#L546">
            Lists created DIDs with metadata as JSON array with each DID in format: DID, verkey, metadata
        </a>
    </th>
   </tr>
   <tr>
     <td>
       <b>NEW</b>
     </td>
     <td>
       <pre>
indy_list_my_dids_with_meta(command_handle: i32,
                            wallet_handle: i32,
                            cb: fn(xcommand_handle: i32, 
                                   err: ErrorCode,
                                   ids: *const c_char))
               </pre>
     </td>
   </tr>
   <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/did.rs#L588">
            Retrieves abbreviated verkey if it is possible otherwise return full verkey.
        </a>
    </th>
   </tr>
   <tr>
     <td>
       <b>NEW</b>
     </td>
     <td>
       <pre>
indy_abbreviate_verkey(command_handle: i32,
                       did: *const c_char,
                       full_verkey: *const c_char,
                       cb: fn(xcommand_handle: i32, 
                              err: ErrorCode,
                              verkey: *const c_char))
               </pre>
     </td>
   </tr>
  <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/did.rs#L13">
            Creates key for a new DID
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_create_and_store_my_did(...)
              </pre>
    </td>
    <td>
      <b>=</b>
    </td>
  </tr>
  <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/did.rs#L72">
            Generated temporary key for an existing DID.
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_replace_keys_start(...)
              </pre>
    </td>
    <td>
      <b>=</b>
    </td>
  </tr>
  <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/did.rs#L124">
            Apply temporary key as main for an existing DID 
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_replace_keys_apply(...)
              </pre>
    </td>
    <td>
      <b>=</b>
    </td>
  </tr>
  <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/did.rs#L163">
            Saves their DID for a pairwise connection in a secured Wallet
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_store_their_did(...)
              </pre>
    </td>
    <td>
      <b>=</b>
    </td>
  </tr>
  <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/did.rs#L207">
            Returns ver key (key id) for the given DID.
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_key_for_did(...)
              </pre>
    </td>
    <td>
      <b>=</b>
    </td>
  </tr>
  </tr>
  <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/did.rs#L264">
            Returns ver key (key id) for the given DID.
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_key_for_local_did(...)
              </pre>
    </td>
    <td>
      <b>=</b>
    </td>
  </tr>
  <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/did.rs#L316">
            Set/replace endpoint information for the given DID.
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_set_endpoint_for_did(...)
              </pre>
    </td>
    <td>
      <b>=</b>
    </td>
  </tr>
  <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/did.rs#L364">
            Gets endpoint information for the given DID.
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_get_endpoint_for_did(...)
              </pre>
    </td>
    <td>
      <b>=</b>
    </td>
  </tr>
  <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/did.rs#L413">
            Saves/replaces the meta information for the giving DID in the wallet.
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_set_did_metadata(...)
              </pre>
    </td>
    <td>
      <b>=</b>
    </td>
  </tr>
  <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/did.rs#L457">
            Retrieves the meta information for the giving DID in the wallet.    
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_get_did_metadata(...)
      </pre>
    </td>
    <td>
      <b>=</b>
    </td>
  </tr>
</table> 
    
### Crypto API mapping

<table>  
  <th>v1.3.0 - Crypto API</th>
  <th>v1.4.0 - Crypto API</th>
  <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/crypto.rs#L243">
           Encrypt a message by authenticated-encryption scheme.
        </a>
    </th>
  </tr>
  <tr>
    <td>
<pre>
indy_crypto_box(command_handle: i32,
                wallet_handle: i32,
                my_vk: *const c_char,
                their_vk: *const c_char,
                message_raw: *const u8,
                message_len: u32,
                cb: fn(xcommand_handle: i32, 
                       err: ErrorCode,
                       encrypted_msg_raw: *const u8, 
                       encrypted_msg_len: u32,
                       nonce_raw: *const u8, 
                       nonce_len: u32))
        </pre>
    </td>
    <td>
<pre>
indy_crypto_auth_crypt(
                command_handle: i32,
                wallet_handle: i32,
                my_vk: *const c_char,
                their_vk: *const c_char,
                msg_data: *const u8,
                msg_len: u32,
                cb: fn(command_handle_: i32,
                       err: ErrorCode,
                       encrypted_msg: *const u8,
                       encrypted_len: u32))
        </pre>
    </td>
  </tr>
  <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/crypto.rs#L303">
           Decrypt a message by authenticated-encryption scheme.
        </a>
    </th>
  </tr>
  <tr>
    <td>
<pre>
indy_crypto_box_open(
                command_handle: i32,
                wallet_handle: i32,
                my_vk: *const c_char,
                their_vk: *const c_char,
                encrypted_msg_raw: *const u8,
                encrypted_msg_len: u32,
                nonce_raw: *const u8,
                nonce_len: u32,
                cb: fn(xcommand_handle: i32, 
                       err: ErrorCode,
                       decrypted_msg_raw: *const u8, 
                       decrypted_msg_len: u32))
        </pre>
    </td>
    <td>
<pre>
indy_crypto_auth_decrypt(
                command_handle: i32,
                wallet_handle: i32,
                my_vk: *const c_char,
                encrypted_msg: *const u8,
                encrypted_len: u32,
                cb: fn(command_handle_: i32,
                       err: ErrorCode,
                       their_vk: *const c_char,
                       msg_data: *const u8,
                       msg_len: u32))
        </pre>
    </td>
  </tr>
  <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/crypto.rs#L360">
           Encrypts a message by anonymous-encryption scheme.
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_crypto_box_seal(
                command_handle: i32,
                their_vk: *const c_char,
                message_raw: *const u8,
                message_len: u32,
                cb: fn(xcommand_handle: i32, 
                       err: ErrorCode,
                       encrypted_msg_raw: *const u8, 
                       encrypted_msg_len: u32))
        </pre>
    </td>
    <td>
      <pre>
indy_crypto_anon_crypt(
                command_handle: i32,
                their_vk: *const c_char,
                msg_data: *const u8,
                msg_len: u32,
                cb: fn(command_handle_: i32,
                       err: ErrorCode,
                       encrypted_msg: *const u8,
                       encrypted_len: u32))
        </pre>
    </td>
  </tr>
  <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/crypto.rs#L411">
           Decrypts a message by anonymous-encryption scheme.
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_crypto_box_seal_open(
                command_handle: i32,
                wallet_handle: i32,
                my_vk: *const c_char,
                encrypted_msg_raw: *const u8,
                encrypted_msg_len: u32,
                cb: fn(xcommand_handle: i32, 
                       err: ErrorCode,
                       decrypted_msg_raw: *const u8, 
                       decrypted_msg_len: u32))
        </pre>
    </td>
    <td>
      <pre>
indy_crypto_anon_decrypt(
                command_handle: i32,
                wallet_handle: i32,
                my_vk: *const c_char,
                encrypted_msg: *const u8,
                encrypted_len: u32,
                cb: fn(command_handle_: i32,
                       err: ErrorCode,
                       msg_data: *const u8,
                       msg_len: u32))
        </pre>
    </td>
  </tr>
  <tr> 
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/crypto.rs#L13">
            Creates keys pair and stores in the wallet.
        </a>
    </th>
  </tr>
    <tr>
      <td>
        <pre>
  indy_create_key(...)
                </pre>
      </td>
      <td>
        <b>=</b>
      </td>
    </tr>
    <tr> 
      <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/crypto.rs#L59">
            Saves/replaces the meta information for the giving key in the wallet.
        </a>
      </th>
    </tr>
    <tr>
      <td>
        <pre>
  indy_set_key_metadata(...)
                </pre>
      </td>
      <td>
        <b>=</b>
      </td>
    </tr>
    <tr> 
      <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/crypto.rs#L103">
              Retrieves the meta information for the giving key in the wallet.
        </a>
      </th>
    </tr>
    <tr>
      <td>
        <pre>
  indy_get_key_metadata(...)
                </pre>
      </td>
      <td>
        <b>=</b>
      </td>
    </tr>
    <tr> 
      <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/crypto.rs#L146">
              Signs a message with a key.
        </a>
      </th>
    </tr>
    <tr>
      <td>
        <pre>
  indy_crypto_sign(...)
                </pre>
      </td>
      <td>
        <b>=</b>
      </td>
    </tr>
    <tr> 
      <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/crypto.rs#L193">
               Verify a signature with a verkey.
        </a>
      </th>
    </tr>
    <tr>
      <td>
        <pre>
  indy_crypto_verify(...)
                </pre>
      </td>
      <td>
        <b>=</b>
      </td>
    </tr>
</table>

### Agent API mapping
The Agent API was completely deleted from Libindy but its functionality in simplified form was saved as part of Crypto API.

<table>  
  <th>v1.3.0 - Agent API</th>
  <th>v1.4.0 - Crypto API</th>
  <tr> 
    <th colspan="2">
      Encrypt a message by authenticated-encryption scheme
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_prep_msg(command_handle: i32,
              wallet_handle: i32,
              sender_vk: *const c_char,
              recipient_vk: *const c_char,
              msg_data: *const u8,
              msg_len: u32,
              cb: fn(command_handle_: i32,
                     err: ErrorCode,
                     encrypted_msg: *const u8,
                     encrypted_len: u32))
              </pre>
    </td>
    <td>
      <pre>
indy_crypto_auth_crypt(
                command_handle: i32,
                wallet_handle: i32,
                my_vk: *const c_char,
                their_vk: *const c_char,
                msg_data: *const u8,
                msg_len: u32,
                cb: fn(command_handle_: i32,
                       err: ErrorCode,
                       encrypted_msg: *const u8,
                       encrypted_len: u32))
        </pre>
    </td>
  </tr>
  <tr> 
    <th colspan="2">
      Encrypts a message by anonymous-encryption scheme.
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_prep_anonymous_msg(
          command_handle: i32,
          recipient_vk: *const c_char,
          msg_data: *const u8,
          msg_len: u32,
          cb: fn(command_handle_: i32,
                 err: ErrorCode,
                 encrypted_msg: *const u8,
                 encrypted_len: u32))
        </pre>
    </td>
    <td>
      <pre>
indy_crypto_anon_crypt(
          command_handle: i32,
          their_vk: *const c_char,
          msg_data: *const u8,
          msg_len: u32,
          cb: fn(command_handle_: i32,
                 err: ErrorCode,
                 encrypted_msg: *const u8,
                 encrypted_len: u32))
        </pre>
    </td>
  </tr>
  <tr> 
    <th colspan="2">
      Decrypts a message.
    </th>
  </tr>
  <tr>
    <td rowspan="2">
      <pre>
indy_parse_msg(command_handle: i32,
               wallet_handle: i32,
               recipient_vk: *const c_char,
               encrypted_msg: *const u8,
               encrypted_len: u32,
               cb: fn(command_handle_: i32,
                      err: ErrorCode,
                      sender_vk: *const c_char,
                      msg_data: *const u8,
                      msg_len: u32))
      </pre>
    </td>
    <td>
      <pre>
Decrypt a message by authenticated-encryption scheme.
Reverse to <i>indy_crypto_auth_crypt</i><hr>indy_crypto_auth_decrypt(command_handle: i32,
                         wallet_handle: i32,
                         my_vk: *const c_char,
                         encrypted_msg: *const u8,
                         encrypted_len: u32,
                         cb: fn(command_handle_: i32,
                                err: ErrorCode,
                                their_vk: *const c_char,
                                msg_data: *const u8,
                                msg_len: u32))
      </re>
    </td>
  </tr>
  <tr>
    <td>
      <pre>
Decrypts a message by anonymous-encryption scheme.
Reverse to <i>indy_crypto_anon_crypt</i><hr>indy_crypto_anon_decrypt(command_handle: i32,
                         wallet_handle: i32,
                         my_vk: *const c_char,
                         encrypted_msg: *const u8,
                         encrypted_len: u32,
                         cb: fn(command_handle_: i32,
                                err: ErrorCode,
                                msg_data: *const u8,
                                msg_len: u32))
      </pre>
    </td>
  </tr>
</table>                                  

### Pairwise API mapping
The Agent API does not have any changes.

### Pool API mapping
The Pool API does not have any changes.

### Wallet API mapping
The Wallet API does not have any changes.

### Explore the Code
Here you can find integration tests that demonstrates basic revocation scenario using the Libindy and Ledger
* [Rust](https://github.com/hyperledger/indy-sdk/blob/master/libindy/tests/interaction.rs)
* [Java](https://github.com/hyperledger/indy-sdk/blob/master/wrappers/java/src/test/java/org/hyperledger/indy/sdk/interaction/AnoncredsRevocationInteractionTest.java)
* [Pyton](https://github.com/hyperledger/indy-sdk/blob/master/wrappers/python/tests/interation/interaction.py)
* [XCode](https://github.com/hyperledger/indy-sdk/blob/master/wrappers/ios/libindy-pod/Indy-demoTests/Demo%20Tests/Interaction.mm)
