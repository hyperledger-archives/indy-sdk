# Changelog

## 1.8.0 - 2018-1-31
* Added function `indy_get_current_error` to get additional information for last error occurred in Libindy.
* Updated Libindy wrappers for automatic getting error details:
    * Python - added `message` and `indy_backtrace` fields to `IndyError` object.
    * Java - added `sdkBacktrace` field to `IndyException`. Libindy `error message` set as the main for `IndyException`.
    * NodeJS - added `indyMessage` and `indyBacktrace` fields to `IndyError` object.
    * Rust - changed type of returning value from enum `ErrorCode` on structure `IndyError` with `error_code`, `message`, `indy_backtrace` fields.
    * Objective-C - added `message` and `indy_backtrace` fields to `userInfo` dictionary in `NSError` object. 
* Updated Indy-Cli to show Libindy error message in some cases.
* Implemented automatic filtering of outdated responses based on comparison of local time with latest transaction ordering time.
* Added *EXPERIMENTAL* `indy_pack_message` and `indy_unpack_message` functions to support *Wire Messages* described in [AMES HIPE](https://github.com/hyperledger/indy-hipe/pull/43)
* Functions `indy_crypto_anon_crypt` and `indy_crypto_anon_decrypt` marked as *Deprecated*.
* Removed `bindgen` folder from Libindy NodeJS wrapper.
* Added `NETWORK_MONITOR` role to NYM transaction builder.
* Bugfixes

NOTE: This version of libindy will work slower with older versions of node due to freshness changes.

## 1.7.0 - 2018-12-21
* Added VCX - a library built over libindy for **V**erifiable **C**redentials e**X**change. API is EXPERIMENTAL.
    * At the current moment mobile builds are not available - they should be added in future releases.
* Added Logging API
    * Added function `indy_get_logger` for plugins to give their logging to libindy
    * Added function `indy_set_logger` for client apps and wrappers to receive logs from libindy
    * Integrated libindy logging into Slf4j for Java wrapper and into python logging facade
* Updated API of Rust wrapper. Now there is no three methods for each API call, there is only one that returns Future.
* Introduced multithreading for Wallet API and CRED_DEF generation 
* Bugfixes

## 1.6.8 - 2018-11-22
* Fix State Proof verification for some types of GET requests to the ledger
* Additional clean-up for secrets in logs
* Update CLI help

## 1.6.7 - 2018-10-9
* Supported setting fees in `did rotate-key` CLI command.
* Supported hexadecimal seed for did and key creation. 
* Removed TGB role.
* Added EXPERIMENTAL Rust wrapper for Libindy.
* Bugfixes.

## 1.6.6 - 2018-09-13
* Fixed Android build rustflags. Now all architectures have same flags.

## 1.6.5 - 2018-09-7
* Fixed `ARGON2I` constants usage to be compatible with the latest sodium.
* Parameter `submitter_did` set as the optional field for:
    * Ledger API `indy_build_get_*` functions (except `indy_build_get_validator_info_request`).
    * all functions in Payment API.
* Fixed Android build rustflags for all architectures for libc linking.

## 1.6.4 - 2018-08-31
* Early API types checks
* Workaround for OS permissions on Android
* Fix Android build ARMv7

## 1.6.3 - 2018-08-28
* Performed the following changes related to Libindy Wallet API:
    * Added separate API function `indy_generate_wallet_key` to generate a random wallet master key.
    * Updated `key_derivation_method` parameter of wallet `credentials` to accept the addition type - `RAW`. 
      By using this type, the result of `indy_generate_wallet_key` can be passed as a wallet master key (key derivation will be skipped).
    * Updated Indy CLI wallet related commands to accept the addition parameter `key_derivation_method`.
* Updated `data` parameter of `indy_build_node_request` API function to accept `blskey_pop` (Proof of possession for BLS key).
* Bugfixes
    * Fixed build flags for Android.s
    * Other minor bugfixes.

## 1.6.2 - 2018-08-14
* Performed the following changes related to Libindy Ledger API:
    * Added `indy_submit_action` endpoint that provides the ability to send either GET_VALIDATOR_INFO or 
      POOL_RESTART request to specific nodes and to specify custom timeout for a response from a node.
    * Updated `indy_build_pool_upgrade_request` API function to accept the additional parameter `package` that allow specify package to be upgraded.* Bugfixes
* Added `pool restart` command in Indy CLI.
* Updated Libindy CD pipeline to run iOS tests and to publish artifacts for Libindy and Libnullpay.
* Updated wallet `credentials` to accept the additional parameter `key_derivation_method`.
  This parameter provides the ability to use different crypto algorithms for master key derivation.
* Bugfixes
 
## 1.6.1 bugfixes - 2018-07-30
* Fix connection performance issue
* Fix Android publishing
  
## 1.6.0 - 2018-07-27
* Integrated tags based search in Anoncreds workflow:
    * Updated `indy_prover_store_credential` API function to create tags for a stored credential object.
    * API functions `indy_prover_get_credentials` and `indy_prover_get_credentials_for_proof_req` marked as `Deprecated`.
    * Added two chains of APIs related to credentials search that allows fetching records by batches:
        * Simple credentials search - `indy_prover_search_credentials`
        * Search credentials for proof request - `indy_prover_search_credentials_for_proof_req`
    * Supported [WQL query language](https://github.com/hyperledger/indy-sdk/tree/master/doc/design/011-wallet-query-language) for all search functions in Anoncreds API.
* Added `indy_prover_get_credential` API function allows to get human-readable credential by the specific id from Wallet.
* Performed changes related to Libindy Wallet behavior:
    * Changed Wallet export serialization format to use the same message pack as the rest of LibIndy.
    * Removed association between Wallet and Pool.
    * Removed persistence of Wallet configuration by Libindy.
    * Updated `wallet_create`, `wallet_open`, `wallet_delete` functions to accept wallet configuration as a single JSON.
* Performed changes related to Libindy [Pool behavior](https://github.com/hyperledger/indy-sdk/tree/master/doc/design/009-efficient-connections):
    * Changed Pool connection logic to avoid unnecessary opened connections.
    * Changed Catch-up process to get all transactions from a single node.
    * Implemented logic of persisting of actual Pool Leger at the end of catch-up process and starting from this point on the next time.
    * Updated format of `config` parameter in `indy_open_pool_ledger` API function to specify runtime Pool configuration.
* Payment API has been updated to support non-UTXO based crypto payments and traditional payments like VISA.
Performed the following changes related to Libindy Payments API:
    * Changed format of input and output parameters.
    * Changed format of result values of `indy_parse_response_with_fees` and `indy_parse_payment_response` API functions.
    * Renamed `indy_build_get_utxo_request` and `indy_parse_get_utxo_response` API functions.
    * Added `indy_build_verify_payment_req` and `indy_parse_verify_payment_response` API functions.
    * Removed EXPERIMENTAL notice from endpoints.
* Added `ledger verify-payment-receipt` command in Indy CLI.
* Implemented experimental support of Android.
* Bugfixes       

Notes:

* There is [migration guide](doc/migration-guide-1.5.0-1.6.0.md) about API changes.
* Wallet format of libindy v1.6 isn't compatible with a wallet format of libindy v1.5. As result it is impossible to use wallets
  created with older libindy versions with libindy v1.6.
  
## 1.5.0 - 2018-06-28

* Introduction of [Wallet Storage](https://github.com/hyperledger/indy-sdk/tree/master/doc/design/003-wallet-storage) concept:
  * In v1.4 libindy allowed to plug different wallet implementations. Plugged wallet in v1.4 handled both security
    and storage layers. In contrast Libindy v1.5 restricts plugged interface by handling only storage layer.
    All encryption is performed in libindy. It simplifies plugged wallets and provides warranty of a good security level
    for 3d party wallets implementations.
  * The format of wallet data was changed for better security and support of efficient search
* Added EXPERIMENTAL [Wallet Export/Import API](https://github.com/hyperledger/indy-sdk/tree/master/doc/design/009-wallet-export-import) and
  corresponded commands to Indy CLI
* ```indy_list_wallets``` endpoint is DEPRECATED and will be removed in the next release. The main idea is avoid
  maintaining created wallet list on libindy side. It will allow to access wallets from a cluster and solve
  some problems on mobile platforms. ```indy_create_wallet``` and ```indy_open_wallet``` endpoints will
  also get related changes in the next release.
* Added [Non-Secrets API](https://github.com/hyperledger/indy-sdk/tree/master/doc/design/003-wallet-storage#non-secrets-api) that allows store and read  
  application specific data in the wallet
* Added EXPERIMENTAL [Generic Payments API](https://github.com/hyperledger/indy-sdk/tree/master/doc/design/004-payment-interface#payment-method-api) that provides
  ability to register custom payment method
  and then create payment addresses, build payment-related transactions, assign fees to transactions
* Added ability to [load custom plugins using Indy CLI](https://github.com/hyperledger/indy-sdk/tree/master/doc/design/006-cli-plugins)
* Added the set of commands in Indy CLI providing ability to perform
  [the main payments operations](https://github.com/hyperledger/indy-sdk/tree/master/doc/design/007-cli-payments):
  * Creation of payment address
  * Listing of payment addresses
  * Getting list of UTXO for payment address
  * Sending payment transaction
  * Adding fees to transactions
  * Getting transactions fees amount
* Implemented simple [Nullpay payment plugin](https://github.com/hyperledger/indy-sdk/tree/master/libnullpay) that provide experience
  similar to real payments system
* Implemented publishing of Ubuntu and Windows packages for `Nullpay` plugin
* Added new Ledger API endpoints and corresponded commands in Indy CLI
  * GET Validator Info request builder
  * Restart POOL request builder
  * Add Multi Signature to request
* Optimized Pool connection process. Libindy v1.5 uses cache of Pool Ledger to speed up opening pool operation.
* Bugfixes

Notes:

* There is [migration guide](doc/migration-guide-1.4.0-1.5.0.md) about API changes
* Wallet format of libindy v1.5 isn't compatible with a wallet format of libindy v1.4. As result it is impossible to use wallets
  created with older libindy versions with libindy v1.5.
* Tails handling contains breaking-change hotfix and blob-storage tails files generated by v1.4 is incompatible with 1.5.

## 1.4.0 - 2018-05-10

* Indy CLI tool added
* Switching from DID-based crypto to keys-based crypto:
  * All DID-based crypto functions (`signus` module) are removed
  * Added key-based `crypto` module
  * Added functions to resolve keys for DIDs
* Agent API moved into `crypto` module
* Support the latest version of CL crypto (through `indy-crypto` library):
  * Added nonce for all protocol steps
  * Added consistency proofs for protocol steps
  * Representation of Proofs changed (sub-proofs now are ordered)
* Support of complete Credentials Revocation workflow in Anoncreds API:
  * Support large Tails handling through BlobStorage API
  * Support new Revocation transactions
  * Add calls for remote Witness calculation
  * State-less approach in Credential issuance process
  * Unified reference approach for Anoncreds entities
* Extend DID API: added some methods for iteration over entities in the wallet.
* Bugfixes

Notes:

* There is [migration guide](doc/migration-guide-1.3.0-1.4.0.md) about API changes
* The changes for Credential Revocation invalidates any Anoncreds made with SDK 1.3. They must be reissued
* This release is intended for development purposes only. The 1.5.0 release of the SDK will contain changes to the wallet format. If you plan to put durable artifacts in the wallet, let us know so we can discuss migration to the future format

## 1.3.0 - 2018-01-12

* Encryption option for default wallet is added

## 1.2.0 - 2018-01-11

* indy_key_for_local_did added

## 1.1.0 - 2017-11-10

* Replaced Agent2Agent API
* New Crypto API
* Updated Signus API

## 1.0.0 - 2017-08-31

* Initial release
