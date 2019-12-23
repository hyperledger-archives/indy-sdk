<!-- markdownlint-disable MD033 -->

# Libindy 1.13 to 1.14 migration Guide

This document is written for developers using Libindy to provide necessary information and
to simplify their transition to Libindy 1.13 from Libindy 1.14. If you are using older Libindy
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
* [Libindy 1.12 to 1.13 migration](https://github.com/hyperledger/indy-sdk/blob/v1.13.0/docs/migration-guides/migration-guide-1.12.0-1.13.0.md)

## Table of contents

* [Notes](#notes)
* [Libindy 1.13 to 1.14 migration](#libindy-113-to-114-migration)
    * [Ledger API](#ledger-api)
    
## Libindy 1.13 to 1.14 migration

#### Ledger API

We did some changes related to transaction author agreement functionality. 

This changes allow to user to review and accept the TAA in advance of it being written to the ledger. 
Thus when we submit a transaction we can report the real date of meaningful acceptance, 
instead of an arbitrary date engineered to be newer than when the TAA is added.

The TAA could be legally accepted at any point after the TAA is approved by network governance. 

There are two changes related to Libindy Ledger API:
* extended definition of `indy_build_txn_author_agreement_request` to accept new parameters:
    * `ratification_ts` - the date (timestamp) of TAA ratification by network government.
    * `retirement_ts` - the date (timestamp) of TAA retirement.
    
   Please take a look that this breaks API regarding earlier Libindy versions.
      
* added a new function `indy_build_disable_all_txn_author_agreements_request` to build DISABLE_ALL_TXN_AUTHR_AGRMTS request. 
Request to disable all Transaction Author Agreement on the ledger.

More details regarding updated transaction author agreement workflow you can find in this [file](../how-tos/transaction-author-agreement.md).