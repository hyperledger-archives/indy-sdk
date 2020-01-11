<!-- markdownlint-disable MD033 -->

# Libindy 1.12 to 1.13 migration Guide

This document is written for developers using Libindy to provide necessary information and
to simplify their transition to Libindy 1.12 from Libindy 1.13. If you are using older Libindy
version you can check migration guides history:

* [Libindy 1.3 to 1.4 migration](https://github.com/hyperledger/indy-sdk/blob/v1.4.0/doc/migration-guide.md)
* [Libindy 1.4 to 1.5 migration](https://github.com/hyperledger/indy-sdk/blob/v1.5.0/doc/migration-guide-1.4.0-1.5.0.md)
* [Libindy 1.5 to 1.6 migration](https://github.com/hyperledger/indy-sdk/blob/v1.6.0/doc/migration-guide-1.5.0-1.6.0.md)
* [Libindy 1.6 to 1.7 migration](https://github.com/hyperledger/indy-sdk/blob/v1.7.0/doc/migration-guide-1.6.0-1.7.0.md)
* [Libindy 1.7 to 1.8 migration](https://github.com/hyperledger/indy-sdk/blob/v1.8.0/doc/migration-guide-1.7.0-1.8.0.md)
* [Libindy 1.8 to 1.9 migration](https://github.com/hyperledger/indy-sdk/blob/v1.9.0/docs/migration-guides/migration-guide-1.8.0-1.9.0.md)
* [Libindy 1.9 to 1.10 migration](https://github.com/hyperledger/indy-sdk/blob/v1.10.0/docs/migration-guides/migration-guide-1.9.0-1.10.0.md)
* [Libindy 1.10 to 1.11 migration](https://github.com/hyperledger/indy-sdk/blob/v1.11.0/docs/migration-guides/migration-guide-1.10.0-1.11.0.md)
* [Libindy 1.11 to 1.12 migration](https://github.com/hyperledger/indy-sdk/blob/v1.12.0/docs/migration-guides/migration-guide-1.11.0-1.12.0.md)

## Table of contents

* [Notes](#notes)
* [Libindy 1.12 to 1.13 migration](#libindy-112-to-113-migration)
    * [Anoncreds API](#anoncreds-api)
    
## Libindy 1.12 to 1.13 migration

#### Anoncreds API

We have introduced some new functionality in revealed attributes -- you can specify multiple attributes in a single revealed attribute unit using parameter `names`. That way you will receive revealed attributes from a single credential. API calls that have changed:

* `indy_prover_get_credentials_for_proof_req`
* `indy_prover_search_credentials_for_proof_req`
* `indy_prover_create_proof`

Last call have a new attribute in a response -- `revealed_attr_groups` -- it contains data about revealed attribute units with `names` field.
Also, `indy_verifier_verify_proof` accepts this attribute in a `requested_proof` field.

##### Backwards compatibility

If Verifier (old) sends a proof request without `names` attribute to Prover (new) and it will receive proof without new fields.

If Verifier (new) sends a proof request with `names` attribute to Prover (old) it will not validate proof request because it will not have parameter `name` in it.