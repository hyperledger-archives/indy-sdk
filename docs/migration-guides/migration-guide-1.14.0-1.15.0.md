<!-- markdownlint-disable MD033 -->

# Libindy 1.14 to 1.15 migration Guide

This document is written for developers using Libindy to provide necessary information and
to simplify their transition to Libindy 1.14 from Libindy 1.15. If you are using older Libindy
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
* [Libindy 1.13 to 1.14 migration](https://github.com/hyperledger/indy-sdk/blob/v1.14.0/docs/migration-guides/migration-guide-1.13.0-1.14.0.md)

## Table of contents

* [Notes](#notes)
* [Libindy 1.14 to 1.15 migration](#libindy-114-to-115-migration)

## Libindy 1.14 to 1.15 migration

The Libindy 1.14.5 release contains bug fixes that don't affect API functions. 
The most important of them:
* Provided correction for `Fix proof verification in case of credential attribute encoded value contains leading zeros` (IS-1491).
  Indy 1.14.3 changes "0" to "" which leads to proof rejection. 