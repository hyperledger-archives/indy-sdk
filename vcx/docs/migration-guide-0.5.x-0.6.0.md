# LibVCX migration guide from 0.5.x to 0.6.0

## A Developer Guide for LibVCX migration

This document is written for developers using LibVCX to provide necessary information and
to simplify their transition to LibVCX 0.5 from LibVCX 0.6.x.

* [API]()
    * [Vcx API](#vcx-api)
* [Libvcx 0.6.0 to 0.6.1 migration](#libvcx-060-to-061-migration-guide)
* [Libvcx 0.6.1 to 0.6.2 migration](#libvcx-061-to-062-migration-guide)

#### Vcx API

We extended the support for Aries protocols:
 * Trust Ping (https://github.com/hyperledger/aries-rfcs/tree/master/features/0048-trust-ping)
    * Added `vcx_connection_send_ping` function to send `Ping` message on remote connection.
    * Updated connection state machine to respond on inbound `Ping` message after the connection is established.
    
   Flow:
    * Outbound Ping:
        1. call `vcx_connection_send_ping` API function to send `ping` message on remote connection. 
        1. call `download_message` to download received messages from an agency.
        1. check that message with `ping_response` type is received.
    * Inbound Ping:
        1. call `download_message` to download messages from an agency.
        1. take the message with `ping` type.
        1. call `update_state_with_message` API function to handle received `ping` message.
 
 * Discover Features (https://github.com/hyperledger/aries-rfcs/tree/master/features/0031-discover-features)
    * Added `vcx_connection_send_discovery_features` function to send discovery features message to the specified connection to discover which features it supports, and to what extent.
    * Updated connection state machine to respond on `Query` and `Disclose` messages after the connection is established.
    
   Flow:
    * Outbound Discover Features:
        1. call `vcx_connection_send_discovery_features` to send `discovery` query on remote connection.
        1. call `download_message` to download received messages from an agency.
        1. take the message with `disclose` type.
        1. call `update_state_with_message` API function to handle `disclose` message.
        
    * Inbound Discover Features:
        1. call `download_message` to download messages from an agency.
        1. take the message with `query` type.
        1. call `update_state_with_message` API function to handle received `query` message.
        
 * Service Decorator (https://github.com/hyperledger/aries-rfcs/tree/master/features/0056-service-decorator)

#### Settings

Added a new Vcx setting: `actors`. 

This setting is used within Discover Features protocol to specify the set of protocols that application supports.

The following actors are implemented by default: `[inviter, invitee, issuer, holder, prover, verifier, sender, receiver]`.

You need to edit this list in case application supports the less number of actors.

## Libvcx 0.6.0 to 0.6.1 migration Guide

The Libvcx 0.6.1 release contains fixes that don't affect API functions and behaviour. 

## Libvcx 0.6.1 to 0.6.2 migration Guide

 We extended the support for Aries protocols:
 * Basic Message (https://github.com/hyperledger/aries-rfcs/tree/master/features/0095-basic-message)
    * Updated `vcx_connection_send_message` function to send any kind of messages:
        * if the message is matched to a known aries message - send as is.
        * if the message isn't known - wrap and send as `basic` message.
 * Accept incoming messages with `basicmessage` type. Use `download_message` to download messages from an agency.


Updated library to support "names" parameter in Proof Request Revealed Attributes (IS-1381).
Here is the current format of `revealed_attrs` parameter accepting by `vcx_proof_create` function:
```
requested_attrs: Describes requested attribute
 {
     "name": Optional<string>, // attribute name, (case insensitive and ignore spaces)
     "names": Optional<[string, string]>, // attribute names, (case insensitive and ignore spaces)
                                          // NOTE: should either be "name" or "names", not both and not none of them.
                                          // Use "names" to specify several attributes that have to match a single credential.
     "restrictions":  (filter_json) {
        "schema_id": string, (Optional)
        "schema_issuer_did": string, (Optional)
        "schema_name": string, (Optional)
        "schema_version": string, (Optional)
        "issuer_did": string, (Optional)
        "cred_def_id": string, (Optional)
    },
     "non_revoked": {
         "from": Optional<(u64)> Requested time represented as a total number of seconds from Unix Epoch, Optional
         "to": Optional<(u64)>
             //Requested time represented as a total number of seconds from Unix Epoch, Optional
     }
 }
```
Note: Use `names` to request from Prover several attributes that must correspond to a single credential.