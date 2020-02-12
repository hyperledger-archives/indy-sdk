<!-- markdownlint-disable MD033 -->

# Libindy 1.11 to 1.12 migration Guide

This document is written for developers using Libindy to provide necessary information and
to simplify their transition to Libindy 1.12 from Libindy 1.11 If you are using older Libindy
version you can check migration guides history:

* [Libindy 1.3 to 1.4 migration](https://github.com/hyperledger/indy-sdk/blob/v1.4.0/doc/migration-guide.md)
* [Libindy 1.4 to 1.5 migration](https://github.com/hyperledger/indy-sdk/blob/v1.5.0/doc/migration-guide-1.4.0-1.5.0.md)
* [Libindy 1.5 to 1.6 migration](https://github.com/hyperledger/indy-sdk/blob/v1.6.0/doc/migration-guide-1.5.0-1.6.0.md)
* [Libindy 1.6 to 1.7 migration](https://github.com/hyperledger/indy-sdk/blob/v1.7.0/doc/migration-guide-1.6.0-1.7.0.md)
* [Libindy 1.7 to 1.8 migration](https://github.com/hyperledger/indy-sdk/blob/v1.8.0/doc/migration-guide-1.7.0-1.8.0.md)
* [Libindy 1.8 to 1.9 migration](https://github.com/hyperledger/indy-sdk/blob/v1.9.0/docs/migration-guides/migration-guide-1.8.0-1.9.0.md)
* [Libindy 1.9 to 1.10 migration](https://github.com/hyperledger/indy-sdk/blob/v1.10.0/docs/migration-guides/migration-guide-1.9.0-1.10.0.md)
* [Libindy 1.10 to 1.11 migration](https://github.com/hyperledger/indy-sdk/blob/v1.11.0/docs/migration-guides/migration-guide-1.10.0-1.11.0.md)

## Table of contents

* [Notes](#notes)
* [Libindy 1.11 to 1.12 migration](#libindy-111-to-112-migration)
    * [DID API](#did-api)
    * [Anoncreds API](#anoncreds-api)
    * [Ledger API](#ledger-api)
    
## Libindy 1.11 to 1.12 migration

### Minimal Support of Fully-Qualified identifiers

General format of fully-qualified identifier is `<prefix>:<method>:<value>`.
* Prefix: specifies entity type:
    * `did` - DID
    * `schema` - Schema Id
    * `creddef` - Credential Definition Id
    * `revreg` - Revocation registry Id
* Method: specifies the network this entity belongs to.
* Value: the main part of identifier.

#### DID API

In this release we have introduced fully-qualified DIDs. All functions right now accept both fully-qualified DIDs and unqualified DIDs.

* Call `indy_create_and_store_my_did` function with specifying of `method_name` field inside `did_info` parameter to create fully qualified DID. 
If this field is skipped the usual unqualified form will be created.

* The new function [indy_qualify_did](https://github.com/hyperledger/indy-sdk/blob/v1.12.0/libindy/src/api/did.rs#L729) was added. This function updates DID stored in the wallet to make it fully qualified, or to do other DID maintenance.
This functions also updates all DID related entities stored in the wallet to point on new identifier.

#### Anoncreds API

As we have released *EXPERIMENTAL* Fully-Qualified identifiers, we can work with both identifier formats in a compatible way. 

The new function [indy_to_unqualified](https://github.com/hyperledger/indy-sdk/blob/v1.12.0/libindy/src/api/anoncreds.rs#L2378) was added. 
This function gets unqualified form of a fully-qualified identifier. 
This function can accept the following entities: 
* DID
* SchemaId 
* CredentialDefinitionId 
* RevocationRegistryId 
* Schema
* CredentialDefinition
* RevocationRegistryDefinition
* CredentialOffer
* CredentialRequest
* ProofRequest

Let's consider Credential Issuance and Proof Presentation for different cases.

* FQ - fully-qualified
* U - unqualified

##### Credential Issuance
* Issuer (FQ) - Holder (U) 
    * Issuer creates DID in the fully qualified way. Schema and CredentialDefinition created based on this DID will be fully-qualified also.
    * Issuer creates Credential Offer for Holder using fully-qualified Credential Definition Id. 
    * Issuer should call `indy_to_unqualified` with `cred_offer_json` to get unqualified form of Credential Offer. 
      Issuer must send this unqualified form to Prover and must use it later on credential creation.
    * The next steps from Issuer and Holder sides are exactly the same as for old Libindy versions.
* Issuer (FQ) - Holder (FQ)
    * All steps are exactly the same as for old Libindy versions.
      All identifiers will be if fully-qualified form.
* Issuer (U) - Holder (U)
    * All steps are exactly the same as for old Libindy versions.
      All identifiers will be if unqualified form.
* Issuer (U) - Holder (FQ) 
    * All steps are exactly the same as for old Libindy versions. 
     Holder can handle unqualified identifiers as well.
     Credential will contain unqualified identifiers.

##### Proof Presentation

Proof Requests now support versioning (`ver` field). 
This field specifies whether restrictions are full qualified or not:
- omit or set "1.0" to use unqualified identifiers. 
- set "2.0" to use fully qualified identifiers. 

* Verifier (FQ) - Prover (U) 
    * Verifier should set `ver` field as '1.0' or omit it on Proof Request preparation.
        * if restrictions are fully-qualified Verifier must call `indy_to_unqualified` function with `proof_request_json` to get unqualified form. 
        * if restrictions are unqualified no additional steps are needed.
    * There are no changes from Prover side on Proof preparation.
    * Verifier -- proof verification -- must use *_id's (`schema_id`, `cred_def_id`, `rev_reg_id`) listed in `proof[identifiers]` as the keys for corresponding `schemas_json`, `credential_defs_json`, `rev_reg_defs_json`, `rev_regs_json` objects.
* Verifier (FQ) - Prover (FQ) 
    * Verifier can use as fully-qualified as unqualified on ProofRequest preparation (set corresponded `ver`). 
    * Additional steps are not needed.
* Verifier (U) - Prover (FQ) 
    * Verifier should set `ver` field as '1.0' or omit it on Proof Request preparation.
    * Prover will create unqualified proof in this case.  All identifiers will be if unqualified form.
    * Additional steps are not needed.
* Verifier (U) - Prover (U) 
    * All steps are exactly the same as for old Libindy versions. 
      All identifiers will be if unqualified form.

#### Ledger API

Although we have released Fully-Qualified DIDs, all ledger-related functions will return unqualified identifiers. 
However you can create ledger requests with both Fully-Qualified and the old ones.  