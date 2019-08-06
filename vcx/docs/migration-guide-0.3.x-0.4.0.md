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
`vcx_credential_update_state_with_message` - Checks for any state change from the given message and updates the the state attribute.


#### Disclosed Proof API

`vcx_disclosed_proof_get_proof_msg` - Get the proof message for sending.
`vcx_disclosed_proof_update_state_with_message` -  Checks for any state change from the given message and updates the the state attribute.


#### Issuer Credential API

`vcx_issuer_get_credential_offer_msg` - Send a credential offer to user showing what will be included in the actual credential.
`vcx_issuer_get_credential_msg` - Send Credential that was requested by user.


#### Proof API

`vcx_proof_get_request_msg` - Get the proof request message.