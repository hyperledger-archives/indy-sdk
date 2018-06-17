# 1.5.0

* Performed significant changes related to [Wallet Storage](https://github.com/hyperledger/indy-sdk/tree/master/doc/design/003-wallet-storage):
    * Changed API of Plugged Wallet storage to extend set of commands related to working with storing data. 
    * Plugged wallet used to handles both security and storage layers. Now all encryption performs on Libindy level. 
    * The format of storing data was changed to support efficient search.
* Provided [Export/Import functionality](https://github.com/hyperledger/indy-sdk/tree/master/doc/design/009-wallet-export-import).
* Added [Non-Secrets API](https://github.com/hyperledger/indy-sdk/tree/master/doc/design/003-wallet-storage#non-secrets-api) that allows store and read application specific data in the wallet.
* Added [Generic Payments API](https://github.com/hyperledger/indy-sdk/tree/master/doc/design/004-payment-interface#payment-method-api) that provides ability to register custom payment method 
and then create payment addresses, build payment-related transactions, assign fees to transactions.
* Added ability of [loading custom plugins by Indy CLI](https://github.com/hyperledger/indy-sdk/tree/master/doc/design/006-cli-plugins).
* Added the set of commands in Indy CLI providing ability to perform [the main payments operations](https://github.com/hyperledger/indy-sdk/tree/master/doc/design/007-cli-payments):
  * Creation of payment address
  * Listing of payment addresses
  * Getting list of UTXO for payment address
  * Sending payment transaction
  * Adding fees to transactions
  * Getting transactions fees amount
* Implemented simple [Nullpay payment plugin](https://github.com/hyperledger/indy-sdk/tree/master/libnullpay) that provide experience similar to real payments system.
  * Implemented publishing of Ubuntu and Windows packages for `Nullpay` plugin.
* Added set of endpoints related to Ledger API. Implemented corresponding commands in Indy CLI. 
    * GET Validator Info request.
    * Restart POOL request.
    * Add Multi Signature to request.
* Bugfixes.

# 1.4.0

* Indy CLI tool added.
* Switching from DID-based crypto to keys-based crypto:
  * All DID-based crypto functions (`signus` module) are removed.
  * Added key-based `crypto` module.
  * Added functions to resolve keys for DIDs.
* Agent API moved into `crypto` module.
* Support the latest version of CL crypto (through `indy-crypto` library):
  * Added nonce for all protocol steps.
  * Added consistency proofs for protocol steps.
  * Representation of Proofs changed (sub-proofs now are ordered).
* Support of complete Credentials Revocation workflow in Anoncreds API:
  * Support large Tails handling through BlobStorage API.
  * Support new Revocation transactions.
  * Add calls for remote Witness calculation.
  * State-less approach in Credential issuance process.
  * Unified reference approach for Anoncreds entities.
* Extend DID API: added some methods for iteration over entities in the wallet. 
* Bugfixes.

Notes:
* There is [migration guide](doc/migration-guide.md) about API changes.
* The changes for Credential Revocation invalidates any Anoncreds made with SDK 1.3. They must be reissued.
* This release is intended for development purposes only. The 1.5.0 release of the SDK will contain changes to the wallet format. If you plan to put durable artifacts in the wallet, let us know so we can discuss migration to the future format.

# 1.3.0

* Encryption option for default wallet is added.

# 1.2.0

* indy_key_for_local_did added.

# 1.1.0

* Replaced Agent2Agent API.
* New Crypto API.
* Updated Signus API.

# 1.0.0

* Initial release.
