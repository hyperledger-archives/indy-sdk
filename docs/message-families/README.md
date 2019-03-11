# Message Families

This design proposes definition of default credential exchange message family protocol and 
the way how a developer can extend Libindy to use his own API helper functions to support 
a new protocol message family definition.  

## Goals and ideas

* Allow extending LibIndy to support new protocol message families and modifying the implementation.
* Simplify process of making new message family definitions available in the common Indy ecosystem.
* Simplify the process of support new agents.
* Define default protocol message families. 
* Define State Machine that developer have to follow to support new protocol message families.
* Define the way LibIndy can consume new API functions.
* Define the flow of communication Libindy static API, custom Libindy API helpers and an application.

## Components

![Components](./components.svg)

## Common A2A State Machine definition

![Common A2A State Machine definition](./state-machine-definition.svg)

### API
* indy_check_if_transition_allowed
* indy_do_step

## Credential Issuance

### Message Family

The Credential Issuance Message Family consists of the following messages:

* Credential Offer
* Credential Request
* Credential
* Credential Ack
* Credential Reject

## State Machine

The Credential Issuance State Machine consists of the following states:

* Initialized
* Issuer related states:
    * OfferCreated
    * OfferSent
    * RequestReceived
    * RejectReceived
    * CredentialCreated
    * CredentialSent
    * CredentialAckReceived
* Prover related states:
    * OfferReceived
    * RequestCreated
    * RequestSent
    * RejectCreated
    * RejectSent
    * CredentialReceived
    * CredentialAckCreated
    * CredentialAckSent
* Finished

![State Machine definition](./credential-issuance-state-machine-definition.svg)

## Workflow

![Workflow](./components.svg)