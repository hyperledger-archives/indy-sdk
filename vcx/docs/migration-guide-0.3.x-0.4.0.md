# LibVCX migration guide from 0.3.x to 0.4.0

## A Developer Guide for LibVCX migration

This document is written for developers using LibVCX to provide necessary information and
to simplify their transition to LibVCX 0.4 from LibVCX 0.3.x.

* [API]()
    * [Vcx API](#vcx-api)
    * [Utils API](#utils-api)
    * [Wallet API](#wallet-api)
    * [Credential API](#credential-api)
    * [Disclosed Proof API](#disclosed-proof-api)
    * [Issuer Credential API](#issuer-credential-api)
    * [Proof API](#proof-api)
* [Libvcx 0.4.0 to 0.4.1 migration](#libvcx-040-to-041-migration-guide)


### API

Added a set of new APIs around credentials and proofs that work with messages that should be exchanged without handling the transport of those messages.
This removes the dependency on an agency/cloud-agent and allows the user of the SDK to transport those messages themselves. 


#### Vcx API

`vcx_init_minimal` - Initialize vcx with the minimal configuration (wallet, pool must already be set with  vcx_wallet_set_handle() and vcx_pool_set_handle()) and without any agency configuration


#### Utils API

`vcx_pool_set_handle` - Set the pool handle before calling vcx_init_minimal


#### Wallet API

`vcx_wallet_set_handle` - Set the wallet handle before calling vcx_init_minimal

#### Credential API

`vcx_credential_get_request_msg` - Get the credential request message that can be sent to the specified connection.
`vcx_credential_update_state_with_message` - Checks for any state change from the given message and updates the  state attribute.


#### Disclosed Proof API

`vcx_disclosed_proof_get_proof_msg` - Get the proof message for sending.
`vcx_disclosed_proof_update_state_with_message` -  Checks for any state change from the given message and updates the  state attribute.


#### Issuer Credential API

`vcx_issuer_get_credential_offer_msg` - Send a credential offer to user showing what will be included in the actual credential.
`vcx_issuer_get_credential_msg` - Send Credential that was requested by user.


#### Proof API

`vcx_proof_get_request_msg` - Get the proof request message.


## Libvcx 0.4.0 to 0.4.1 migration Guide

#### Endorse a transaction 

* In the current state, Libvcx provides functionality for the publishing of 2 types of entities that can be endorsed:
* Schema
* Credential Definition

The set of new similar functions was added to provide a way how these entities (`schema`, `credentialdef`) can be endorsered.
    * `vcx_*_prepare_for_endorser` - build transaction and crete internal object in differed state.
    * `vcx_*_update_state` - functions to update state of internal object.
    * `vcx_*_get_state` - functions to get state of internal object.
    * `vcx_endorse_transaction` - function to endorse a transaction to the ledger.

```
let (schema_hsndle, schema_json) = vcx_schema_prepare_for_endorser(...)
vcx_schema_get_state(schema_hsndle) == Built
vcx_endorse_transaction(schema_json)
vcx_schema_update_state(schema_hsndle)
vcx_schema_get_state(schema_hsndle) == Published
```

#### Sign with Address

Supported sign/verify with payment address functionality:
    * `vcx_wallet_sign_with_address` - to sign a message with a payment address.
    * `vcx_wallet_verify_with_address` - to verify a signature with a payment address.
    
#### Vcx init

Extended Libvcx initialization config to accept pool configuration.
```
{
    ...
    "pool_config": "{
        "timeout": int (optional) - specifies the maximum number of seconds to wait for pool response (ACK, REPLY).
        "extended_timeout": int (optional), an additional number of seconds to wait for REPLY in case ACK has been received.
        "preordered_nodes": array<string> -  (optional), names of nodes which will have priority during request sending.
            This can be useful if a user prefers querying specific nodes.
            Note: Nodes not specified will be placed randomly.
        "number_read_nodes": int (optional) - the number of nodes to send read requests (2 by default)
    }"

}
```

## Libvcx 0.4.1 to 0.4.2 migration Guide

Extended VCX provisioning config to accept optional `did_method` filed. 
This field should be used to create fully qualified DIDs.
The format of identifiers that are used on CredentialIssuance and ProofPresentation will be determined based on the type of remote DID.
