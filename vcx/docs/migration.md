# Introduction

This version of libVCX represents a major step forward. The last version of libvcx based on libindy 1.3 gave institutions the ability to create connections with Connect.me on iOS devices. The latest version now gives the ability to manage those connections from either the view of the institution or the consumer. Sending and accepting connection invitations, sending credential offers and requests, and sending proof requests and providing proof are all supported. In fact, this version of libVCX is now the main library and engine behind both the Verity-UI web interface and both the iOS and Android versions of Connect.me. In addition to supporting both sides of verified credential exchange libVCX now supports token related activities. When used with libSovtoken libVCX can send and receive tokens, pay for ledger transactions, issue premium credentials and pay for premium credentials. Support for libindy 1.6 and the latest Evernym agencies is also included.

## Changelog

-   Schema IDs and credential definition IDs have changed (see libindy migrating guides here: https://github.com/hyperledger/indy-sdk/tree/master/doc).  
-   The vcx_claimdef_* family of functions is now the vcx_credentialdef_* family.
-   vcx_credential_* - family of functions for receiving credential offers, sending credential requests and storing issued credentials in the wallet.
-   vcx_disclosed_proof_* - family of functions for receiving proof requests, retrieving credentials relevant to a particular proof request and generating and sending proofs in response to proof requests.
-   Many changes to vcxconfig.json parameter names
    
## Configuration Changes

| Previously              | Currently            |
|-------------------------|----------------------|
| agent_endpoint          | agency_endpoint      |
| agency_pairwise_did     | agency_did           |
| agency_pairwise_verkey  | agency_verkey        |
| agent_pairwise_did      | remote_to_sdk_did    |
| agent_pairwise_verkey   | remote_to_sdk_verkey |
| enterprise_did_agent    | sdk_to_remote_did    |
| agent_enterprise_verkey | sdk_to_remote_verkey |
| enterprise_did          | institution_did      |
| enterprise_verkey       | institution_verkey   |
| enterprise_name         | institution_name     |
| logo_url                | institution_logo_url |

## Upgrading (don’t do it!)

The sweeping changes to libVCX/libIndy and the PoC nature of the previous version mean that upgrading is not supported. The old version must be uninstalled and the configuration and wallet removed before installing the new version. Reprovisioning an agent, creating a new configuration and creating a new wallet must be done by using the provision_agent_keys.py script after installing the new version of libvcx.

## Tokens/Payments

One of the major features of the this new version is the addition of token or payment related functionality. There is a new wallet API that allows the creation of payment addresses, the querying of addresses and balances, and the sending of tokens to other addresses. Payment of ledger fees and premium credentials is handled automatically when creating schemas and credential definitions and sending credential requests. While there is a payment_handle type defined in the API it is a placeholder and does not currently have any functionality. Payments are handled automatically by libVCX and addresses with sufficient balances are automatically used when payment is needed.
