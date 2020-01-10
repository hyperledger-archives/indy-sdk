# LibVCX migration guide from 0.5.x to 0.6.0

## A Developer Guide for LibVCX migration

This document is written for developers using LibVCX to provide necessary information and
to simplify their transition to LibVCX 0.5 from LibVCX 0.6.x.

* [API]()
    * [Vcx API](#vcx-api)

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
