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

The sweeping changes to libVCX/libIndy and the PoC nature of the previous version mean that upgrading is not supported. The old version must be uninstalled and the configuration and wallet removed before installing the new version. Consider backing up and removing ~/.indy_client. Reprovisioning an agent, creating a new configuration and creating a new wallet must be done by using the provision_agent_keys.py script after installing the new version of libvcx.

## Tokens/Payments

One of the major features of the this new version is the addition of token or payment related functionality. There is a new wallet API that allows the creation of payment addresses, the querying of addresses and balances, and the sending of tokens to other addresses. Payment of ledger fees and premium credentials is handled automatically when creating schemas and credential definitions and sending credential requests. While there is a payment_handle type defined in the API it is a placeholder and does not currently have any functionality. Payments are handled automatically by libVCX and addresses with sufficient balances are automatically used when payment is needed.

## Changes to Common API Calls
Most calls to methods in libvcx are unchanged or changes are small, and where they exist they usually are related to supporting the token payment system. The following table shows where changes have occurred (using the python wrapper as an example). Asterisks are used to denote changed parameters.

| Previously              | Currently               |
|-------------------------|-------------------------|
| vcx_init(<br />config_path:str<br />) | vcx_init(<br />config_path:str<br />) |
| Connection.create(<br />source_id:str<br />) | Connection.create(<br />source_id:str<br />) |
| connection.connect(<br />phone_nbr<br />)| connection.connect(<br />phone_nbr<br />)|
| connection.update_state()| connection.update_state()|
| connection.get_state()| connection.get_state()|
| Schema.create(<br />source_id:str, <br />name:str, <br />attrs:list<br />)<br />&nbsp;<br />&nbsp; | Schema.create(<br />source_id:str, <br />name:str, <br />\*version:str, <br />attrs:list, <br />\*payment_handle:int<br />) |
| schema.get_sequence_number() | schema.\*get_schema_id() |
| CredentialDef.create(<br />source_id:str, <br />name:str, <br />schema_seq_nbr:str, <br />)<br />&nbsp; | CredentialDef.create(<br />source_id:str, <br />name:str, <br />\*schema_id:str, <br />\*payment_handle:int<br />)|
| | cred_def.\*get_cred_def_id()|
| IssuerCredential.create(<br />source_id:str, <br />attrs:dict, <br />schema_seq_no:str, <br />name:str<br />)<br />&nbsp; | IssuerCredential.create(<br />source_id:str, <br />\*attrs:dict, <br />\*cred_def_id:str, <br />name:str, <br />\*price:str<br />) |
| credential.send_offer(<br />connection<br />) | credential.send_offer(<br />connection<br />) |
| credential.send_credential(<br />connection<br />) | credential.send_credential(<br />connection<br />) |
| credential.update_state()| credential.update_state()|
| credential.get_state()| credential.get_state()|
| Proof.create(<br />source_id:str, <br />name:str, <br />requested_attrs:list<br />)| Proof.create(<br />source_id:str, <br />name:str, <br />\*requested_attrs:list<br />)|
| proof.request_proof(<br />connection<br />)| proof.request_proof(<br />connection<br />)|
| proof.update_state()| proof.update_state()|
| proof.get_state()| proof.get_state()|
| proof.get_proof(<br />connection<br />)| proof.get_proof(<br />connection<br />)|

## Schema and Proof Attribute Changes
The attribute data structures used to define schemas and proofs have changed as follows:

| Previously              | Currently               |
|-------------------------|-------------------------|
| Schema:<br />&nbsp;<br />{<br />"attr_names":["attr1", "attr2", "attr3", ...],<br />"name":"schema_name",<br />"version": "version_string"<br />}| ["attr1", "attr2", "attr3", ...]|
| Proof:<br />&nbsp;<br />[{<br />&nbsp;&nbsp;"name": "attr1",<br />&nbsp;&nbsp;"issuerDid": "DID1"<br />},<br />{<br />&nbsp;&nbsp;"name": "attr2"<br />&nbsp;&nbsp;"issuerDid": "DID2",<br />}, ...<br />]<br />| [{<br />&nbsp;&nbsp;"name": "attr1",<br />&nbsp;&nbsp;"restrictions": [{<br />&nbsp;&nbsp;&nbsp;&nbsp;"criteria1_name": "criteria1_value",<br />&nbsp;&nbsp;&nbsp;&nbsp;"criteria2_name": "criteria2_value", ...<br />&nbsp;&nbsp;}, ...]<br />},<br />{<br />&nbsp;&nbsp;"name": "attr2",<br />&nbsp;&nbsp;"restrictions": []<br />}, ...<br />]|

## Documentation
For more detail on the methods and parameters, refer to the in-line documentation for the nodejs or python3 wrapper that you are using.
