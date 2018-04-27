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
