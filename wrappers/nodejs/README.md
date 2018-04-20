# Indy SDK for Node.js

[![stability - experimental](https://img.shields.io/badge/stability-experimental-orange.svg)](https://nodejs.org/api/documentation.html#documentation_stability_index)
![Node version](https://img.shields.io/node/v/indy-sdk.svg)

Native bindings for [Hyperledger Indy](https://www.hyperledger.org/projects/hyperledger-indy).

- [Installing](#installing)
- [Usage](#usage)
- [API](#api)
  * [IndyError](#indyerror)
  * [anoncreds](#anoncreds)
  * [blob_storage](#blob_storage)
  * [crypto](#crypto)
  * [did](#did)
  * [ledger](#ledger)
  * [pairwise](#pairwise)
  * [pool](#pool)
  * [wallet](#wallet)
- [Contributing](#contributing)

## Installing

This module has a native compile step. It compiles C++ code and dynamically links to `libindy`.

You will need:

* C++ build tools and Python 2. See [this](https://github.com/nodejs/node-gyp#installation) for platform recommendations.
* `libindy` v1.3.1+ in your system library path. (i.e. `/usr/lib/libindy.so` for linux)

Then you can install via npm:

```sh
npm install --save indy-sdk
```

## Usage

```js
var indy = require('indy-sdk')

var did = '...'
var fullVerkey = '...'

indy.abbreviateVerkey(did, fullVerkey, function(err, verkey){
  ..
})

// if you do not provide a callback, a Promise is returned

var verkey = await indy.abbreviateVerkey(did, fullVerkey)
```

# API

### IndyError

All the functions may yield an IndyError. The errors are based on libindy error codes defined [here](https://github.com/hyperledger/indy-sdk/blob/master/libindy/include/indy_mod.h).

* `err.indyCode` - the code number from libindy
* `err.indyName` - the name string for the code

[//]: # (CODEGEN-START - don't edit by hand see `codegen/index.js`)
### anoncreds

#### issuerCreateSchema \( issuerDid, name, version, attrNames \) -&gt; \[ id, schema \]

Create credential schema entity that describes credential attributes list and allows credentials
interoperability.

Schema is public and intended to be shared with all anoncreds workflow actors usually by publishing SCHEMA transaction
to Indy distributed ledger.

It is IMPORTANT for current version POST Schema in Ledger and after that GET it from Ledger
with correct seq\_no to save compatibility with Ledger.
After that can call indy\_issuer\_create\_and\_store\_credential\_def to build corresponding Credential Definition.

* `issuerDid`: String - DID of schema issuer
* `name`: String - a name the schema
* `version`: String - a version of the schema
* `attrNames`: Json
* __->__ [ `id`: String, `schema`: Json ] - schema\_id: identifier of created schema
schema\_json: schema as json

Errors: `Common*`, `Anoncreds*`

#### issuerCreateAndStoreCredentialDef \( walletHandle, issuerDid, schema, tag, signatureType, config \) -&gt; \[ credDefId, credDef \]

Create credential definition entity that encapsulates credentials issuer DID, credential schema, secrets used for signing credentials
and secrets used for credentials revocation.

Credential definition entity contains private and public parts. Private part will be stored in the wallet. Public part
will be returned as json intended to be shared with all anoncreds workflow actors usually by publishing CRED\_DEF transaction
to Indy distributed ledger.

It is IMPORTANT for current version GET Schema from Ledger with correct seq\_no to save compatibility with Ledger.

* `walletHandle`: Handle (Number) - wallet handler \(created by open\_wallet\).
* `issuerDid`: String - a DID of the issuer signing cred\_def transaction to the Ledger
* `schema`: Json - credential schema as a json
* `tag`: String - allows to distinct between credential definitions for the same issuer and schema
* `signatureType`: String - credential definition type \(optional, 'CL' by default\) that defines credentials signature and revocation math. Supported types are:
- 'CL': Camenisch-Lysyanskaya credential signature type
* `config`: Json - type-specific configuration of credential definition as json:
- 'CL':
- support\_revocation: whether to request non-revocation credential \(optional, default false\)
* __->__ [ `credDefId`: String, `credDef`: Json ] - cred\_def\_id: identifier of created credential definition
cred\_def\_json: public part of created credential definition

Errors: `Common*`, `Wallet*`, `Anoncreds*`

#### issuerCreateAndStoreRevocReg \( walletHandle, issuerDid, revocDefType, tag, credDefId, config, tailsWriterHandle \) -&gt; \[ revocRegId, revocRegDef, revocRegEntry \]

Create a new revocation registry for the given credential definition as tuple of entities:
- Revocation registry definition that encapsulates credentials definition reference, revocation type specific configuration and
secrets used for credentials revocation
- Revocation registry state that stores the information about revoked entities in a non-disclosing way. The state can be
represented as ordered list of revocation registry entries were each entry represents the list of revocation or issuance operations.

Revocation registry definition entity contains private and public parts. Private part will be stored in the wallet. Public part
will be returned as json intended to be shared with all anoncreds workflow actors usually by publishing REVOC\_REG\_DEF transaction
to Indy distributed ledger.

Revocation registry state is stored on the wallet and also intended to be shared as the ordered list of REVOC\_REG\_ENTRY transactions.
This call initializes the state in the wallet and returns the initial entry.

Some revocation registry types \(for example, 'CL\_ACCUM'\) can require generation of binary blob called tails used to hide information about revoked credentials in public
revocation registry and intended to be distributed out of leger \(REVOC\_REG\_DEF transaction will still contain uri and hash of tails\).
This call requires access to pre-configured blob storage writer instance handle that will allow to write generated tails.

* `walletHandle`: Handle (Number) - wallet handler \(created by open\_wallet\).
* `issuerDid`: String - a DID of the issuer signing transaction to the Ledger
* `revocDefType`: String - revocation registry type \(optional, default value depends on credential definition type\). Supported types are:
- 'CL\_ACCUM': Type-3 pairing based accumulator. Default for 'CL' credential definition type
* `tag`: String - allows to distinct between revocation registries for the same issuer and credential definition
* `credDefId`: String - id of stored in ledger credential definition
* `config`: Json - type-specific configuration of revocation registry as json:
- 'CL\_ACCUM': {
"issuance\_type": \(optional\) type of issuance. Currently supported:
1\) ISSUANCE\_BY\_DEFAULT: all indices are assumed to be issued and initial accumulator is calculated over all indices;
Revocation Registry is updated only during revocation.
2\) ISSUANCE\_ON\_DEMAND: nothing is issued initially accumulator is 1 \(used by default\);
"max\_cred\_num": maximum number of credentials the new registry can process \(optional, default 100000\)
}
* `tailsWriterHandle`: Handle (Number) - handle of blob storage to store tails
* __->__ [ `revocRegId`: String, `revocRegDef`: Json, `revocRegEntry`: Json ] - revoc\_reg\_id: identifier of created revocation registry definition
revoc\_reg\_def\_json: public part of revocation registry definition
revoc\_reg\_entry\_json: revocation registry entry that defines initial state of revocation registry

Errors: `Common*`, `Wallet*`, `Anoncreds*`

#### issuerCreateCredentialOffer \( walletHandle, credDefId \) -&gt; credOffer

Create credential offer that will be used by Prover for
credential request creation. Offer includes nonce and key correctness proof
for authentication between protocol steps and integrity checking.

* `walletHandle`: Handle (Number) - wallet handler \(created by open\_wallet\)
* `credDefId`: String - id of credential definition stored in the wallet
* __->__ `credOffer`: Json - credential offer json:
```js
    {
        "schema_id": string,
        "cred_def_id": string,
        // Fields below can depend on Cred Def type
        "nonce": string,
        "key_correctness_proof" : <key_correctness_proof>
    }
````

Errors: `Common*`, `Wallet*`, `Anoncreds*`

#### issuerCreateCredential \( walletHandle, credOffer, credReq, credValues, revRegId, blobStorageReaderHandle \) -&gt; \[ cred, credRevocId, revocRegDelta \]

Check Cred Request for the given Cred Offer and issue Credential for the given Cred Request.

Cred Request must match Cred Offer. The credential definition and revocation registry definition
referenced in Cred Offer and Cred Request must be already created and stored into the wallet.

Information for this credential revocation will be store in the wallet as part of revocation registry under
generated cred\_revoc\_id local for this wallet.

This call returns revoc registry delta as json file intended to be shared as REVOC\_REG\_ENTRY transaction.
Note that it is possible to accumulate deltas to reduce ledger load.

* `walletHandle`: Handle (Number) - wallet handler \(created by open\_wallet\).
* `credOffer`: Json - a cred offer created by indy\_issuer\_create\_credential\_offer
* `credReq`: Json - a credential request created by indy\_prover\_create\_credential\_req
* `credValues`: Json - a credential containing attribute values for each of requested attribute names.
* `revRegId`: String - id of revocation registry stored in the wallet
* `blobStorageReaderHandle`: Number - configuration of blob storage reader handle that will allow to read revocation tails
* __->__ [ `cred`: Json, `credRevocId`: String, `revocRegDelta`: Json ] - cred\_json: Credential json containing signed credential values
```js
    {
        "schema_id": string,
        "cred_def_id": string,
        "rev_reg_def_id", Optional<string>,
        "values": <see cred_values_json above>,
        // Fields below can depend on Cred Def type
        "signature": <signature>,
        "signature_correctness_proof": <signature_correctness_proof>
    }
cred_revoc_id: local id for revocation info (Can be used for revocation of this cred)
revoc_reg_delta_json: Revocation registry delta json with a newly issued credential
````

Errors: `Annoncreds*`, `Common*`, `Wallet*`

#### issuerRevokeCredential \( walletHandle, blobStorageReaderHandle, revRegId, credRevocId \) -&gt; revocRegDelta

Revoke a credential identified by a cred\_revoc\_id \(returned by indy\_issuer\_create\_credential\).

The corresponding credential definition and revocation registry must be already
created an stored into the wallet.

This call returns revoc registry delta as json file intended to be shared as REVOC\_REG\_ENTRY transaction.
Note that it is possible to accumulate deltas to reduce ledger load.

* `walletHandle`: Handle (Number) - wallet handler \(created by open\_wallet\).
* `blobStorageReaderHandle`: Number
* `revRegId`: String - id of revocation registry stored in wallet
* `credRevocId`: String - local id for revocation info
* __->__ `revocRegDelta`: Json - revoc\_reg\_delta\_json: Revocation registry delta json with a revoked credential

Errors: `Annoncreds*`, `Common*`, `Wallet*`

#### issuerMergeRevocationRegistryDeltas \( revRegDelta, otherRevRegDelta \) -&gt; mergedRevRegDelta

Merge two revocation registry deltas \(returned by indy\_issuer\_create\_credential or indy\_issuer\_revoke\_credential\) to accumulate common delta.
Send common delta to ledger to reduce the load.

* `revRegDelta`: Json - revocation registry delta.
* `otherRevRegDelta`: Json - revocation registry delta for which PrevAccum value  is equal to current accum value of rev\_reg\_delta\_json.
* __->__ `mergedRevRegDelta`: Json - merged\_rev\_reg\_delta: Merged revocation registry delta

Errors: `Annoncreds*`, `Common*`, `Wallet*`

#### proverCreateMasterSecret \( walletHandle, masterSecretId \) -&gt; outMasterSecretId

Creates a master secret with a given id and stores it in the wallet.
The id must be unique.

* `walletHandle`: Handle (Number) - wallet handler \(created by open\_wallet\).
* `masterSecretId`: String - \(optional, if not present random one will be generated\) new master id
* __->__ `outMasterSecretId`: String - out\_master\_secret\_id: Id of generated master secret

Errors: `Annoncreds*`, `Common*`, `Wallet*`

#### proverCreateCredentialReq \( walletHandle, proverDid, credOffer, credDef, masterSecretId \) -&gt; \[ credReq, credReqMetadata \]

Creates a credential request for the given credential offer.

The method creates a blinded master secret for a master secret identified by a provided name.
The master secret identified by the name must be already stored in the secure wallet \(see prover\_create\_master\_secret\)
The blinded master secret is a part of the credential request.

* `walletHandle`: Handle (Number) - wallet handler \(created by open\_wallet\)
* `proverDid`: String - a DID of the prover
* `credOffer`: Json - credential offer as a json containing information about the issuer and a credential
* `credDef`: Json - credential definition json
* `masterSecretId`: String - the id of the master secret stored in the wallet
* __->__ [ `credReq`: Json, `credReqMetadata`: Json ] - cred\_req\_json: Credential request json for creation of credential by Issuer
```js
    {
     "prover_did" : string,
     "cred_def_id" : string,
        // Fields below can depend on Cred Def type
     "blinded_ms" : <blinded_master_secret>,
     "blinded_ms_correctness_proof" : <blinded_ms_correctness_proof>,
     "nonce": string
   }
cred_req_metadata_json: Credential request metadata json for processing of received form Issuer credential.
````

Errors: `Annoncreds*`, `Common*`, `Wallet*`

#### proverStoreCredential \( walletHandle, credId, credReqMetadata, cred, credDef, revRegDef \) -&gt; outCredId

Check credential provided by Issuer for the given credential request,
updates the credential by a master secret and stores in a secure wallet.

* `walletHandle`: Handle (Number) - wallet handler \(created by open\_wallet\).
* `credId`: String - \(optional, default is a random one\) identifier by which credential will be stored in the wallet
* `credReqMetadata`: Json - a credential request metadata created by indy\_prover\_create\_credential\_req
* `cred`: Json - credential json received from issuer
* `credDef`: Json - credential definition json
* `revRegDef`: Json - revocation registry definition json
* __->__ `outCredId`: String - out\_cred\_id: identifier by which credential is stored in the wallet

Errors: `Annoncreds*`, `Common*`, `Wallet*`

#### proverGetCredentials \( walletHandle, filter \) -&gt; credentials

Gets human readable credentials according to the filter.
If filter is NULL, then all credentials are returned.
Credentials can be filtered by Issuer, credential\_def and\/or Schema.

* `walletHandle`: Handle (Number) - wallet handler \(created by open\_wallet\).
* `filter`: Json - filter for credentials
```js
       {
           "schema_id": string, (Optional)
           "schema_issuer_did": string, (Optional)
           "schema_name": string, (Optional)
           "schema_version": string, (Optional)
           "issuer_did": string, (Optional)
           "cred_def_id": string, (Optional)
       }
````
* __->__ `credentials`: Json - credentials json
```js
    [{
        "referent": string, // cred_id in the wallet
        "values": <see cred_values_json above>,
        "schema_id": string,
        "cred_def_id": string,
        "rev_reg_id": Optional<string>,
        "cred_rev_id": Optional<string>
    }]
````

Errors: `Annoncreds*`, `Common*`, `Wallet*`

#### proverGetCredentialsForProofReq \( walletHandle, proofRequest \) -&gt; credentials

Gets human readable credentials matching the given proof request.

* `walletHandle`: Handle (Number) - wallet handler \(created by open\_wallet\).
* `proofRequest`: Json - proof request json
```js
    {
        "name": string,
        "version": string,
        "nonce": string,
        "requested_attributes": { // set of requested attributes
             "<attr_referent>": <attr_info>, // see below
             ...,
        },
        "requested_predicates": { // set of requested predicates
             "<predicate_referent>": <predicate_info>, // see below
             ...,
         },
        "non_revoked": Optional<<non_revoc_interval>>, // see below,
                       // If specified prover must proof non-revocation
                       // for date in this interval for each attribute
                       // (can be overridden on attribute level)
    }
where
````
* __->__ `credentials`: Json - credentials\_json: json with credentials for the given pool request.
```js
    {
        "requested_attrs": {
            "<attr_referent>": [{ cred_info: <credential_info>, interval: Optional<non_revoc_interval> }],
            ...,
        },
        "requested_predicates": {
            "requested_predicates": [{ cred_info: <credential_info>, timestamp: Optional<integer> }, { cred_info: <credential_2_info>, timestamp: Optional<integer> }],
            "requested_predicate_2_referent": [{ cred_info: <credential_2_info>, timestamp: Optional<integer> }]
        }
    }, where credential is
    {
        "referent": <string>,
        "attrs": [{"attr_name" : "attr_raw_value"}],
        "schema_id": string,
        "cred_def_id": string,
        "rev_reg_id": Optional<int>,
        "cred_rev_id": Optional<int>,
    }
````

Errors: `Annoncreds*`, `Common*`, `Wallet*`

#### proverCreateProof \( walletHandle, proofReq, requestedCredentials, masterSecretName, schemas, credentialDefs, revStates \) -&gt; proof

Creates a proof according to the given proof request
Either a corresponding credential with optionally revealed attributes or self-attested attribute must be provided
for each requested attribute \(see indy\_prover\_get\_credentials\_for\_pool\_req\).
A proof request may request multiple credentials from different schemas and different issuers.
All required schemas, public keys and revocation registries must be provided.
The proof request also contains nonce.
The proof contains either proof or self-attested attribute value for each requested attribute.

* `walletHandle`: Handle (Number) - wallet handler \(created by open\_wallet\).
* `proofReq`: Json
* `requestedCredentials`: Json - either a credential or self-attested attribute for each requested attribute
```js
    {
        "self_attested_attributes": {
            "self_attested_attribute_referent": string
        },
        "requested_attributes": {
            "requested_attribute_referent_1": {"cred_id": string, "timestamp": Optional<number>, revealed: <bool> }},
            "requested_attribute_referent_2": {"cred_id": string, "timestamp": Optional<number>, revealed: <bool> }}
        },
        "requested_predicates": {
            "requested_predicates_referent_1": {"cred_id": string, "timestamp": Optional<number> }},
        }
    }
````
* `masterSecretName`: String
* `schemas`: Json - all schemas json participating in the proof request
```js
    {
        <schema1_id>: <schema1_json>,
        <schema2_id>: <schema2_json>,
        <schema3_id>: <schema3_json>,
    }
````
* `credentialDefs`: Json - all credential definitions json participating in the proof request
```js
    {
        "cred_def1_id": <credential_def1_json>,
        "cred_def2_id": <credential_def2_json>,
        "cred_def3_id": <credential_def3_json>,
    }
````
* `revStates`: Json - all revocation states json participating in the proof request
```js
    {
        "rev_reg_def1_id": {
            "timestamp1": <rev_state1>,
            "timestamp2": <rev_state2>,
        },
        "rev_reg_def2_id": {
            "timestamp3": <rev_state3>
        },
        "rev_reg_def3_id": {
            "timestamp4": <rev_state4>
        },
    }
where
````
* __->__ `proof`: Json - Proof json
For each requested attribute either a proof \(with optionally revealed attribute value\) or
self-attested attribute value is provided.
Each proof is associated with a credential and corresponding schema\_id, cred\_def\_id, rev\_reg\_id and timestamp.
There is also aggregated proof part common for all credential proofs.
```js
    {
        "requested": {
            "revealed_attrs": {
                "requested_attr1_id": {sub_proof_index: number, raw: string, encoded: string},
                "requested_attr4_id": {sub_proof_index: number: string, encoded: string},
            },
            "unrevealed_attrs": {
                "requested_attr3_id": {sub_proof_index: number}
            },
            "self_attested_attrs": {
                "requested_attr2_id": self_attested_value,
            },
            "requested_predicates": {
                "requested_predicate_1_referent": {sub_proof_index: int},
                "requested_predicate_2_referent": {sub_proof_index: int},
            }
        }
        "proof": {
            "proofs": [ <credential_proof>, <credential_proof>, <credential_proof> ],
            "aggregated_proof": <aggregated_proof>
        }
        "identifiers": [{schema_id, cred_def_id, Optional<rev_reg_id>, Optional<timestamp>}]
    }
````

Errors: `Annoncreds*`, `Common*`, `Wallet*`

#### verifierVerifyProof \( proofRequest, proof, schemas, credentialDefsJsons, revRegDefs, revRegs \) -&gt; valid

Verifies a proof \(of multiple credential\).
All required schemas, public keys and revocation registries must be provided.

* `proofRequest`: Json - proof request json
```js
    {
        "name": string,
        "version": string,
        "nonce": string,
        "requested_attributes": { // set of requested attributes
             "<attr_referent>": <attr_info>, // see below
             ...,
        },
        "requested_predicates": { // set of requested predicates
             "<predicate_referent>": <predicate_info>, // see below
             ...,
         },
        "non_revoked": Optional<<non_revoc_interval>>, // see below,
                       // If specified prover must proof non-revocation
                       // for date in this interval for each attribute
                       // (can be overridden on attribute level)
    }
````
* `proof`: Json - created for request proof json
```js
    {
        "requested": {
            "revealed_attrs": {
                "requested_attr1_id": {sub_proof_index: number, raw: string, encoded: string},
                "requested_attr4_id": {sub_proof_index: number: string, encoded: string},
            },
            "unrevealed_attrs": {
                "requested_attr3_id": {sub_proof_index: number}
            },
            "self_attested_attrs": {
                "requested_attr2_id": self_attested_value,
            },
            "requested_predicates": {
                "requested_predicate_1_referent": {sub_proof_index: int},
                "requested_predicate_2_referent": {sub_proof_index: int},
            }
        }
        "proof": {
            "proofs": [ <credential_proof>, <credential_proof>, <credential_proof> ],
            "aggregated_proof": <aggregated_proof>
        }
        "identifiers": [{schema_id, cred_def_id, Optional<rev_reg_id>, Optional<timestamp>}]
    }
````
* `schemas`: Json - all schema jsons participating in the proof
```js
    {
        <schema1_id>: <schema1_json>,
        <schema2_id>: <schema2_json>,
        <schema3_id>: <schema3_json>,
    }
````
* `credentialDefsJsons`: Json
* `revRegDefs`: Json - all revocation registry definitions json participating in the proof
```js
    {
        "rev_reg_def1_id": <rev_reg_def1_json>,
        "rev_reg_def2_id": <rev_reg_def2_json>,
        "rev_reg_def3_id": <rev_reg_def3_json>,
    }
````
* `revRegs`: Json - all revocation registries json participating in the proof
```js
    {
        "rev_reg_def1_id": {
            "timestamp1": <rev_reg1>,
            "timestamp2": <rev_reg2>,
        },
        "rev_reg_def2_id": {
            "timestamp3": <rev_reg3>
        },
        "rev_reg_def3_id": {
            "timestamp4": <rev_reg4>
        },
    }
````
* __->__ `valid`: Boolean - valid: true - if signature is valid, false - otherwise

Errors: `Annoncreds*`, `Common*`, `Wallet*`

#### createRevocationState \( blobStorageReaderHandle, revRegDef, revRegDelta, timestamp, credRevId \) -&gt; revState

Create revocation state for a credential in the particular time moment.

* `blobStorageReaderHandle`: Number - configuration of blob storage reader handle that will allow to read revocation tails
* `revRegDef`: Json - revocation registry definition json
* `revRegDelta`: Json - revocation registry definition delta json
* `timestamp`: Timestamp (Number) - time represented as a total number of seconds from Unix Epoch
* `credRevId`: String - user credential revocation id in revocation registry
* __->__ `revState`: Json - revocation state json:
```js
    {
        "rev_reg": <revocation registry>,
        "witness": <witness>,
        "timestamp" : integer
    }
````

Errors: `Common*`, `Wallet*`, `Anoncreds*`

#### updateRevocationState \( blobStorageReaderHandle, revState, revRegDef, revRegDelta, timestamp, credRevId \) -&gt; updatedRevState

Create new revocation state for a credential based on existed state
at the particular time moment \(to reduce calculation time\).

* `blobStorageReaderHandle`: Number - configuration of blob storage reader handle that will allow to read revocation tails
* `revState`: Json - revocation registry state json
* `revRegDef`: Json - revocation registry definition json
* `revRegDelta`: Json - revocation registry definition delta json
* `timestamp`: Timestamp (Number) - time represented as a total number of seconds from Unix Epoch
* `credRevId`: String - user credential revocation id in revocation registry
* __->__ `updatedRevState`: Json - revocation state json:
```js
    {
        "rev_reg": <revocation registry>,
        "witness": <witness>,
        "timestamp" : integer
    }
````

Errors: `Common*`, `Wallet*`, `Anoncreds*`

### blob_storage

#### openBlobStorageReader \( type, config \) -&gt; handle



* `type`: String
* `config`: Json
* __->__ `handle`: Number


#### openBlobStorageWriter \( type, config \) -&gt; handle



* `type`: String
* `config`: Json
* __->__ `handle`: Number


### crypto

#### createKey \( walletHandle, key \) -&gt; vk

Creates keys pair and stores in the wallet.

* `walletHandle`: Handle (Number) - Wallet handle \(created by open\_wallet\).
* `key`: Json - Key information as json. Example:
```js
{
    "seed": string, // Optional (if not set random one will be used); Seed information that allows deterministic key creation.
    "crypto_type": string, // Optional (if not set then ed25519 curve is used); Currently only 'ed25519' value is supported for this field.
}
````
* __->__ `vk`: String - Ver key of generated key pair, also used as key identifier

Errors: `Common*`, `Wallet*`, `Crypto*`

#### setKeyMetadata \( walletHandle, verkey, metadata \) -&gt; void

Saves\/replaces the meta information for the giving key in the wallet.

* `walletHandle`: Handle (Number) - Wallet handle \(created by open\_wallet\).
verkey - the key \(verkey, key id\) to store metadata.
metadata - the meta information that will be store with the key.
* `verkey`: String
* `metadata`: String
* __->__ void

Errors: `Common*`, `Wallet*`, `Crypto*`

#### getKeyMetadata \( walletHandle, verkey \) -&gt; metadata

Retrieves the meta information for the giving key in the wallet.

* `walletHandle`: Handle (Number) - Wallet handle \(created by open\_wallet\).
verkey - The key \(verkey, key id\) to retrieve metadata.
* `verkey`: String
* __->__ `metadata`: String - The meta information stored with the key; Can be null if no metadata was saved for this key.

Errors: `Common*`, `Wallet*`, `Crypto*`

#### cryptoSign \( walletHandle, signerVk, messageRaw \) -&gt; signatureRaw

Signs a message with a key.

Note to use DID keys with this function you can call indy\_key\_for\_did to get key id \(verkey\)
for specific DID.

* `walletHandle`: Handle (Number) - wallet handler \(created by open\_wallet\).
* `signerVk`: String - id \(verkey\) of my key. The key must be created by calling indy\_create\_key or indy\_create\_and\_store\_my\_did
* `messageRaw`: Buffer - a pointer to first byte of message to be signed
* __->__ `signatureRaw`: Buffer - a signature string

Errors: `Common*`, `Wallet*`, `Crypto*`

#### cryptoVerify \( signerVk, messageRaw, signatureRaw \) -&gt; valid

Verify a signature with a verkey.

Note to use DID keys with this function you can call indy\_key\_for\_did to get key id \(verkey\)
for specific DID.

* `signerVk`: String - verkey of signer of the message
* `messageRaw`: Buffer - a pointer to first byte of message that has been signed
* `signatureRaw`: Buffer - a pointer to first byte of signature to be verified
* __->__ `valid`: Boolean - valid: true - if signature is valid, false - otherwise

Errors: `Common*`, `Wallet*`, `Ledger*`, `Crypto*`

#### cryptoAuthCrypt \( walletHandle, senderVk, recipientVk, messageRaw \) -&gt; encryptedMsgRaw

Encrypt a message by authenticated-encryption scheme.

Sender can encrypt a confidential message specifically for Recipient, using Sender's public key.
Using Recipient's public key, Sender can compute a shared secret key.
Using Sender's public key and his secret key, Recipient can compute the exact same shared secret key.
That shared secret key can be used to verify that the encrypted message was not tampered with,
before eventually decrypting it.

Note to use DID keys with this function you can call indy\_key\_for\_did to get key id \(verkey\)
for specific DID.

* `walletHandle`: Handle (Number) - wallet handle \(created by open\_wallet\).
* `senderVk`: String - id \(verkey\) of my key. The key must be created by calling indy\_create\_key or indy\_create\_and\_store\_my\_did
* `recipientVk`: String - id \(verkey\) of their key
* `messageRaw`: Buffer - a pointer to first byte of message that to be encrypted
* __->__ `encryptedMsgRaw`: Buffer - an encrypted message as a pointer to array of bytes.

Errors: `Common*`, `Wallet*`, `Ledger*`, `Crypto*`

#### cryptoAuthDecrypt \( walletHandle, recipientVk, encryptedMsgRaw \) -&gt; \[ senderVk, decryptedMsgRaw \]

Decrypt a message by authenticated-encryption scheme.

Sender can encrypt a confidential message specifically for Recipient, using Sender's public key.
Using Recipient's public key, Sender can compute a shared secret key.
Using Sender's public key and his secret key, Recipient can compute the exact same shared secret key.
That shared secret key can be used to verify that the encrypted message was not tampered with,
before eventually decrypting it.

Note to use DID keys with this function you can call indy\_key\_for\_did to get key id \(verkey\)
for specific DID.

* `walletHandle`: Handle (Number) - wallet handler \(created by open\_wallet\).
* `recipientVk`: String - id \(verkey\) of my key. The key must be created by calling indy\_create\_key or indy\_create\_and\_store\_my\_did
* `encryptedMsgRaw`: Buffer - a pointer to first byte of message that to be decrypted
* __->__ [ `senderVk`: String, `decryptedMsgRaw`: Buffer ] - sender verkey and decrypted message as a pointer to array of bytes

Errors: `Common*`, `Wallet*`, `Crypto*`

#### cryptoAnonCrypt \( recipientVk, messageRaw \) -&gt; encryptedMsgRaw

Encrypts a message by anonymous-encryption scheme.

Sealed boxes are designed to anonymously send messages to a Recipient given its public key.
Only the Recipient can decrypt these messages, using its private key.
While the Recipient can verify the integrity of the message, it cannot verify the identity of the Sender.

Note to use DID keys with this function you can call indy\_key\_for\_did to get key id \(verkey\)
for specific DID.

* `recipientVk`: String - verkey of message recipient
* `messageRaw`: Buffer - a pointer to first byte of message that to be encrypted
* __->__ `encryptedMsgRaw`: Buffer - an encrypted message as a pointer to array of bytes

Errors: `Common*`, `Wallet*`, `Ledger*`, `Crypto*`

#### cryptoAnonDecrypt \( walletHandle, recipientVk, encryptedMsg \) -&gt; decryptedMsgRaw

Decrypts a message by anonymous-encryption scheme.

Sealed boxes are designed to anonymously send messages to a Recipient given its public key.
Only the Recipient can decrypt these messages, using its private key.
While the Recipient can verify the integrity of the message, it cannot verify the identity of the Sender.

Note to use DID keys with this function you can call indy\_key\_for\_did to get key id \(verkey\)
for specific DID.

* `walletHandle`: Handle (Number) - wallet handler \(created by open\_wallet\).
* `recipientVk`: String - id \(verkey\) of my key. The key must be created by calling indy\_create\_key or indy\_create\_and\_store\_my\_did
* `encryptedMsg`: Buffer
* __->__ `decryptedMsgRaw`: Buffer - decrypted message as a pointer to an array of bytes

Errors: `Common*`, `Wallet*`, `Crypto*`

### did

#### createAndStoreMyDid \( walletHandle, did \) -&gt; \[ did, verkey \]

Creates keys \(signing and encryption keys\) for a new
DID \(owned by the caller of the library\).
Identity's DID must be either explicitly provided, or taken as the first 16 bit of verkey.
Saves the Identity DID with keys in a secured Wallet, so that it can be used to sign
and encrypt transactions.

* `walletHandle`: Handle (Number) - wallet handler \(created by open\_wallet\).
* `did`: Json - Identity information as json. Example:
```js
{
    "did": string, (optional;
            if not provided and cid param is false then the first 16 bit of the verkey will be used as a new DID;
            if not provided and cid is true then the full verkey will be used as a new DID;
            if provided, then keys will be replaced - key rotation use case)
    "seed": string, (optional; if not provide then a random one will be created)
    "crypto_type": string, (optional; if not set then ed25519 curve is used;
              currently only 'ed25519' value is supported for this field)
    "cid": bool, (optional; if not set then false is used;)
}
````
* __->__ [ `did`: String, `verkey`: String ] - did: DID generated and stored in the wallet
verkey: The DIDs verification key

Errors: `Common*`, `Wallet*`, `Crypto*`

#### replaceKeysStart \( walletHandle, did, identity \) -&gt; verkey

Generated temporary keys \(signing and encryption keys\) for an existing
DID \(owned by the caller of the library\).

* `walletHandle`: Handle (Number) - wallet handler \(created by open\_wallet\).
* `did`: String
* `identity`: Json - Identity information as json. Example:
```js
{
    "seed": string, (optional; if not provide then a random one will be created)
    "crypto_type": string, (optional; if not set then ed25519 curve is used;
              currently only 'ed25519' value is supported for this field)
}
````
* __->__ `verkey`: String - verkey: The DIDs verification key

Errors: `Common*`, `Wallet*`, `Crypto*`

#### replaceKeysApply \( walletHandle, did \) -&gt; void

Apply temporary keys as main for an existing DID \(owned by the caller of the library\).

* `walletHandle`: Handle (Number) - wallet handler \(created by open\_wallet\).
* `did`: String - DID stored in the wallet
* __->__ void

Errors: `Common*`, `Wallet*`, `Crypto*`

#### storeTheirDid \( walletHandle, identity \) -&gt; void

Saves their DID for a pairwise connection in a secured Wallet,
so that it can be used to verify transaction.

* `walletHandle`: Handle (Number) - wallet handler \(created by open\_wallet\).
* `identity`: Json - Identity information as json. Example:
```js
    {
       "did": string, (required)
       "verkey": string (optional, can be avoided if did is cryptonym: did == verkey),
    }
````
* __->__ void

Errors: `Common*`, `Wallet*`, `Crypto*`

#### keyForDid \( poolHandle, walletHandle, did \) -&gt; key

Returns ver key \(key id\) for the given DID.

"indy\_key\_for\_did" call follow the idea that we resolve information about their DID from
the ledger with cache in the local wallet. The "indy\_open\_wallet" call has freshness parameter
that is used for checking the freshness of cached pool value.

Note if you don't want to resolve their DID info from the ledger you can use
"indy\_key\_for\_local\_did" call instead that will look only to the local wallet and skip
freshness checking.

Note that "indy\_create\_and\_store\_my\_did" makes similar wallet record as "indy\_create\_key".
As result we can use returned ver key in all generic crypto and messaging functions.

* `poolHandle`: Handle (Number) - Pool handle \(created by open\_pool\).
* `walletHandle`: Handle (Number) - Wallet handle \(created by open\_wallet\).
did - The DID to resolve key.
* `did`: String
* __->__ `key`: String - The DIDs ver key \(key id\).

Errors: `Common*`, `Wallet*`, `Crypto*`

#### keyForLocalDid \( walletHandle, did \) -&gt; key

Returns ver key \(key id\) for the given DID.

"indy\_key\_for\_local\_did" call looks data stored in the local wallet only and skips freshness
checking.

Note if you want to get fresh data from the ledger you can use "indy\_key\_for\_did" call
instead.

Note that "indy\_create\_and\_store\_my\_did" makes similar wallet record as "indy\_create\_key".
As result we can use returned ver key in all generic crypto and messaging functions.

* `walletHandle`: Handle (Number) - Wallet handle \(created by open\_wallet\).
did - The DID to resolve key.
* `did`: String
* __->__ `key`: String - The DIDs ver key \(key id\).

Errors: `Common*`, `Wallet*`, `Crypto*`

#### setEndpointForDid \( walletHandle, did, address, transportKey \) -&gt; void

Set\/replaces endpoint information for the given DID.

* `walletHandle`: Handle (Number) - Wallet handle \(created by open\_wallet\).
did - The DID to resolve endpoint.
address -  The DIDs endpoint address.
transport\_key - The DIDs transport key \(ver key, key id\).
* `did`: String
* `address`: String
* `transportKey`: String
* __->__ void

Errors: `Common*`, `Wallet*`, `Crypto*`

#### getEndpointForDid \( walletHandle, poolHandle, did \) -&gt; \[ address, transportVk \]

Returns endpoint information for the given DID.

* `walletHandle`: Handle (Number) - Wallet handle \(created by open\_wallet\).
did - The DID to resolve endpoint.
* `poolHandle`: Handle (Number)
* `did`: String
* __->__ [ `address`: String, `transportVk`: String ] - The DIDs endpoint.
- transport\_vk - The DIDs transport key \(ver key, key id\).

Errors: `Common*`, `Wallet*`, `Crypto*`

#### setDidMetadata \( walletHandle, did, metadata \) -&gt; void

Saves\/replaces the meta information for the giving DID in the wallet.

* `walletHandle`: Handle (Number) - Wallet handle \(created by open\_wallet\).
did - the DID to store metadata.
metadata - the meta information that will be store with the DID.
* `did`: String
* `metadata`: String
* __->__ void

Errors: `Common*`, `Wallet*`, `Crypto*`

#### getDidMetadata \( walletHandle, did \) -&gt; metadata

Retrieves the meta information for the giving DID in the wallet.

* `walletHandle`: Handle (Number) - Wallet handle \(created by open\_wallet\).
did - The DID to retrieve metadata.
* `did`: String
* __->__ `metadata`: String - The meta information stored with the DID; Can be null if no metadata was saved for this DID.

Errors: `Common*`, `Wallet*`, `Crypto*`

#### getMyDidWithMeta \( walletHandle, myDid \) -&gt; didWithMeta

Retrieves the information about the giving DID in the wallet.

* `walletHandle`: Handle (Number) - Wallet handle \(created by open\_wallet\).
did - The DID to retrieve information.
* `myDid`: String
* __->__ `didWithMeta`: Json - did\_with\_meta: {
"did": string - DID stored in the wallet,
"verkey": string - The DIDs transport key \(ver key, key id\),
"metadata": string - The meta information stored with the DID
}

Errors: `Common*`, `Wallet*`, `Crypto*`

#### listMyDidsWithMeta \( walletHandle \) -&gt; dids

Retrieves the information about all DIDs stored in the wallet.

* `walletHandle`: Handle (Number) - Wallet handle \(created by open\_wallet\).
* __->__ `dids`: Json - dids: \[{
"did": string - DID stored in the wallet,
"verkey": string - The DIDs transport key \(ver key, key id\).,
"metadata": string - The meta information stored with the DID
}\]

Errors: `Common*`, `Wallet*`, `Crypto*`

#### abbreviateVerkey \( did, fullVerkey \) -&gt; verkey

Retrieves abbreviated verkey if it is possible otherwise return full verkey.

* `did`: String - DID.
* `fullVerkey`: String - The DIDs verification key,
* __->__ `verkey`: String - verkey: The DIDs verification key in either abbreviated or full form

Errors: `Common*`, `Wallet*`, `Crypto*`

### ledger

#### signAndSubmitRequest \( poolHandle, walletHandle, submitterDid, request \) -&gt; requestResult

Signs and submits request message to validator pool.

Adds submitter information to passed request json, signs it with submitter
sign key \(see wallet\_sign\), and sends signed request message
to validator pool \(see write\_request\).

* `poolHandle`: Handle (Number) - pool handle \(created by open\_pool\_ledger\).
* `walletHandle`: Handle (Number) - wallet handle \(created by open\_wallet\).
* `submitterDid`: String - Id of Identity stored in secured Wallet.
* `request`: Json - Request data json.
* __->__ `requestResult`: Json

Errors: `Common*`, `Wallet*`, `Ledger*`, `Crypto*`

#### submitRequest \( poolHandle, request \) -&gt; requestResult

Publishes request message to validator pool \(no signing, unlike sign\_and\_submit\_request\).

The request is sent to the validator pool as is. It's assumed that it's already prepared.

* `poolHandle`: Handle (Number) - pool handle \(created by open\_pool\_ledger\).
* `request`: Json - Request data json.
* __->__ `requestResult`: Json

Errors: `Common*`, `Ledger*`

#### signRequest \( walletHandle, submitterDid, request \) -&gt; signedRequest

Signs request message.

Adds submitter information to passed request json, signs it with submitter
sign key \(see wallet\_sign\).

* `walletHandle`: Handle (Number) - wallet handle \(created by open\_wallet\).
* `submitterDid`: String - Id of Identity stored in secured Wallet.
* `request`: Json - Request data json.
* __->__ `signedRequest`: Json - Signed request json.

Errors: `Common*`, `Wallet*`, `Ledger*`, `Crypto*`

#### buildGetDdoRequest \( submitterDid, targetDid \) -&gt; requestResult

Builds a request to get a DDO.

* `submitterDid`: String - Id of Identity stored in secured Wallet.
* `targetDid`: String - Id of Identity stored in secured Wallet.
* __->__ `requestResult`: Json

Errors: `Common*`

#### buildNymRequest \( submitterDid, targetDid, verkey, alias, role \) -&gt; request

Builds a NYM request. Request to create a new NYM record for a specific user.

* `submitterDid`: String - DID of the submitter stored in secured Wallet.
* `targetDid`: String - Target DID as base58-encoded string for 16 or 32 bit DID value.
* `verkey`: String - Target identity verification key as base58-encoded string.
* `alias`: String - NYM's alias.
* `role`: String - Role of a user NYM record:
null \(common USER\)
TRUSTEE
STEWARD
TRUST\_ANCHOR
empty string to reset role
* __->__ `request`: Json

Errors: `Common*`

#### buildAttribRequest \( submitterDid, targetDid, hash, raw, enc \) -&gt; request

Builds an ATTRIB request. Request to add attribute to a NYM record.

* `submitterDid`: String - DID of the submitter stored in secured Wallet.
* `targetDid`: String - Target DID as base58-encoded string for 16 or 32 bit DID value.
* `hash`: String - \(Optional\) Hash of attribute data.
* `raw`: String - \(Optional\) Json, where key is attribute name and value is attribute value.
* `enc`: String - \(Optional\) Encrypted value attribute data.
* __->__ `request`: Json

Errors: `Common*`

#### buildGetAttribRequest \( submitterDid, targetDid, hash, raw, enc \) -&gt; request

Builds a GET\_ATTRIB request. Request to get information about an Attribute for the specified DID.

* `submitterDid`: String - DID of the read request sender.
* `targetDid`: String - Target DID as base58-encoded string for 16 or 32 bit DID value.
* `hash`: String - \(Optional\) Requested attribute hash.
* `raw`: String - \(Optional\) Requested attribute name.
* `enc`: String - \(Optional\) Requested attribute encrypted value.
* __->__ `request`: Json

Errors: `Common*`

#### buildGetNymRequest \( submitterDid, targetDid \) -&gt; request

Builds a GET\_NYM request. Request to get information about a DID \(NYM\).

* `submitterDid`: String - DID of the read request sender.
* `targetDid`: String - Target DID as base58-encoded string for 16 or 32 bit DID value.
* __->__ `request`: Json

Errors: `Common*`

#### buildSchemaRequest \( submitterDid, data \) -&gt; request

Builds a SCHEMA request. Request to add Credential's schema.

* `submitterDid`: String - DID of the submitter stored in secured Wallet.
* `data`: String - Credential schema.
```js
{
````
* __->__ `request`: Json

Errors: `Common*`

#### buildGetSchemaRequest \( submitterDid, id \) -&gt; request

Builds a GET\_SCHEMA request. Request to get Credential's Schema.

* `submitterDid`: String - DID of the read request sender.
* `id`: String - Schema ID in ledger
* __->__ `request`: Json

Errors: `Common*`

#### parseGetSchemaResponse \( getSchemaResponse \) -&gt; \[ schemaId, schema \]

Parse a GET\_SCHEMA response to get Schema in the format compatible with Anoncreds API.

* `getSchemaResponse`: String - response of GET\_SCHEMA request.
* __->__ [ `schemaId`: String, `schema`: Json ] - Schema Id and Schema json.
```js
{
    id: identifier of schema
    attrNames: array of attribute name strings
    name: Schema's name string
    version: Schema's version string
    ver: Version of the Schema json
}
````

Errors: `Common*`

#### buildCredDefRequest \( submitterDid, data \) -&gt; request

Builds an CRED\_DEF request. Request to add a Credential Definition \(in particular, public key\),
that Issuer creates for a particular Credential Schema.

* `submitterDid`: String - DID of the submitter stored in secured Wallet.
* `data`: String - credential definition json
```js
{
````
* __->__ `request`: Json

Errors: `Common*`

#### buildGetCredDefRequest \( submitterDid, id \) -&gt; request

Builds a GET\_CRED\_DEF request. Request to get a Credential Definition \(in particular, public key\),
that Issuer creates for a particular Credential Schema.

* `submitterDid`: String - DID of the read request sender.
* `id`: String - Credential Definition ID in ledger.
* __->__ `request`: Json

Errors: `Common*`

#### parseGetCredDefResponse \( getCredDefResponse \) -&gt; \[ credDefId, credDef \]

Parse a GET\_CRED\_DEF response to get Credential Definition in the format compatible with Anoncreds API.

* `getCredDefResponse`: String - response of GET\_CRED\_DEF request.
* __->__ [ `credDefId`: String, `credDef`: Json ] - Credential Definition Id and Credential Definition json.
```js
{
    id: string - identifier of credential definition
    schemaId: string - identifier of stored in ledger schema
    type: string - type of the credential definition. CL is the only supported type now.
    tag: string - allows to distinct between credential definitions for the same issuer and schema
    value: Dictionary with Credential Definition's data: {
        primary: primary credential public key,
        Optional<revocation>: revocation credential public key
    },
    ver: Version of the Credential Definition json
}
````

Errors: `Common*`

#### buildNodeRequest \( submitterDid, targetDid, data \) -&gt; request

Builds a NODE request. Request to add a new node to the pool, or updates existing in the pool.

* `submitterDid`: String - DID of the submitter stored in secured Wallet.
* `targetDid`: String - Target Node's DID.  It differs from submitter\_did field.
* `data`: String - Data associated with the Node: {
* __->__ `request`: Json

Errors: `Common*`

#### buildGetTxnRequest \( submitterDid, data \) -&gt; request

Builds a GET\_TXN request. Request to get any transaction by its seq\_no.

* `submitterDid`: String - DID of the request submitter.
* `data`: Number
* __->__ `request`: Json

Errors: `Common*`

#### buildPoolConfigRequest \( submitterDid, writes, force \) -&gt; request

Builds a POOL\_CONFIG request. Request to change Pool's configuration.

* `submitterDid`: String - DID of the submitter stored in secured Wallet.
* `writes`: Boolean - Whether any write requests can be processed by the pool
\(if false, then pool goes to read-only state\). True by default.
* `force`: Boolean - Whether we should apply transaction \(for example, move pool to read-only state\)
without waiting for consensus of this transaction.
* __->__ `request`: Json

Errors: `Common*`

#### buildPoolRestartRequest \( submitterDid, action, datetime \) -&gt; request

Builds a POOL\_RESTART request.

* `submitterDid`: String - Id of Identity stored in secured Wallet.
* `action`: String
* `datetime`: String
* __->__ `request`: Json

Errors: `Common*`

#### buildPoolUpgradeRequest \( submitterDid, name, version, action, sha256, timeout, schedule, justification, reinstall, force \) -&gt; request

Builds a POOL\_UPGRADE request. Request to upgrade the Pool \(sent by Trustee\).
It upgrades the specified Nodes \(either all nodes in the Pool, or some specific ones\).

* `submitterDid`: String - DID of the submitter stored in secured Wallet.
* `name`: String - Human-readable name for the upgrade.
* `version`: String - The version of indy-node package we perform upgrade to.
Must be greater than existing one \(or equal if reinstall flag is True\).
* `action`: String - Either start or cancel.
* `sha256`: String - sha256 hash of the package.
* `timeout`: Number - \(Optional\) Limits upgrade time on each Node.
* `schedule`: String - \(Optional\) Schedule of when to perform upgrade on each node. Map Node DIDs to upgrade time.
* `justification`: String - \(Optional\) justification string for this particular Upgrade.
* `reinstall`: Boolean - Whether it's allowed to re-install the same version. False by default.
* `force`: Boolean - Whether we should apply transaction \(schedule Upgrade\) without waiting
for consensus of this transaction.
* __->__ `request`: Json

Errors: `Common*`

#### buildRevocRegDefRequest \( submitterDid, data \) -&gt; request

Builds a REVOC\_REG\_DEF request. Request to add the definition of revocation registry
to an exists credential definition.

* `submitterDid`: String - DID of the submitter stored in secured Wallet.
* `data`: String - Revocation Registry data:
```js
    {
        "id": string - ID of the Revocation Registry,
        "revocDefType": string - Revocation Registry type (only CL_ACCUM is supported for now),
        "tag": string - Unique descriptive ID of the Registry,
        "credDefId": string - ID of the corresponding CredentialDefinition,
        "value": Registry-specific data {
            "issuanceType": string - Type of Issuance(ISSUANCE_BY_DEFAULT or ISSUANCE_ON_DEMAND),
            "maxCredNum": number - Maximum number of credentials the Registry can serve.
            "tailsHash": string - Hash of tails.
            "tailsLocation": string - Location of tails file.
            "publicKeys": <public_keys> - Registry's public key.
        },
        "ver": string - version of revocation registry definition json.
    }
````
* __->__ `request`: Json

Errors: `Common*`

#### buildGetRevocRegDefRequest \( submitterDid, id \) -&gt; request

Builds a GET\_REVOC\_REG\_DEF request. Request to get a revocation registry definition,
that Issuer creates for a particular Credential Definition.

* `submitterDid`: String - DID of the read request sender.
* `id`: String - ID of Revocation Registry Definition in ledger.
* __->__ `request`: Json

Errors: `Common*`

#### parseGetRevocRegDefResponse \( getRevocRefDefResponse \) -&gt; \[ revocRegDefId, revocRegDef \]

Parse a GET\_REVOC\_REG\_DEF response to get Revocation Registry Definition in the format
compatible with Anoncreds API.

* `getRevocRefDefResponse`: String
* __->__ [ `revocRegDefId`: String, `revocRegDef`: Json ] - Revocation Registry Definition Id and Revocation Registry Definition json.
```js
{
    "id": string - ID of the Revocation Registry,
    "revocDefType": string - Revocation Registry type (only CL_ACCUM is supported for now),
    "tag": string - Unique descriptive ID of the Registry,
    "credDefId": string - ID of the corresponding CredentialDefinition,
    "value": Registry-specific data {
        "issuanceType": string - Type of Issuance(ISSUANCE_BY_DEFAULT or ISSUANCE_ON_DEMAND),
        "maxCredNum": number - Maximum number of credentials the Registry can serve.
        "tailsHash": string - Hash of tails.
        "tailsLocation": string - Location of tails file.
        "publicKeys": <public_keys> - Registry's public key.
    },
    "ver": string - version of revocation registry definition json.
}
````

Errors: `Common*`

#### buildRevocRegEntryRequest \( submitterDid, revocRegDefId, revDefType, value \) -&gt; request

Builds a REVOC\_REG\_ENTRY request.  Request to add the RevocReg entry containing
the new accumulator value and issued\/revoked indices.
This is just a delta of indices, not the whole list.
So, it can be sent each time a new credential is issued\/revoked.

* `submitterDid`: String - DID of the submitter stored in secured Wallet.
* `revocRegDefId`: String - ID of the corresponding RevocRegDef.
* `revDefType`: String - Revocation Registry type \(only CL\_ACCUM is supported for now\).
* `value`: String - Registry-specific data: {

```js
 {
````
* __->__ `request`: Json

Errors: `Common*`

#### buildGetRevocRegRequest \( submitterDid, revocRegDefId, timestamp \) -&gt; request

Builds a GET\_REVOC\_REG request. Request to get the accumulated state of the Revocation Registry
by ID. The state is defined by the given timestamp.

* `submitterDid`: String - DID of the read request sender.
* `revocRegDefId`: String - ID of the corresponding Revocation Registry Definition in ledger.
* `timestamp`: Timestamp (Number) - Requested time represented as a total number of seconds from Unix Epoch
* __->__ `request`: Json

Errors: `Common*`

#### parseGetRevocRegResponse \( getRevocRegResponse \) -&gt; \[ revocRegDefId, revocReg, timestamp \]

Parse a GET\_REVOC\_REG response to get Revocation Registry in the format compatible with Anoncreds API.

* `getRevocRegResponse`: String - response of GET\_REVOC\_REG request.
* __->__ [ `revocRegDefId`: String, `revocReg`: Json, `timestamp`: Timestamp (Number) ] - Revocation Registry Definition Id, Revocation Registry json and Timestamp.
```js
{
    "value": Registry-specific data {
        "accum": string - current accumulator value.
    },
    "ver": string - version revocation registry json
}
````

Errors: `Common*`

#### buildGetRevocRegDeltaRequest \( submitterDid, revocRegDefId, from, to \) -&gt; request

Builds a GET\_REVOC\_REG\_DELTA request. Request to get the delta of the accumulated state of the Revocation Registry.
The Delta is defined by from and to timestamp fields.
If from is not specified, then the whole state till to will be returned.

* `submitterDid`: String - DID of the read request sender.
* `revocRegDefId`: String - ID of the corresponding Revocation Registry Definition in ledger.
* `from`: Timestamp (Number) - Requested time represented as a total number of seconds from Unix Epoch
* `to`: Timestamp (Number) - Requested time represented as a total number of seconds from Unix Epoch
* __->__ `request`: Json

Errors: `Common*`

#### parseGetRevocRegDeltaResponse \( getRevocRegDeltaResponse \) -&gt; \[ revocRegDefId, revocRegDelta, timestamp \]

Parse a GET\_REVOC\_REG\_DELTA response to get Revocation Registry Delta in the format compatible with Anoncreds API.

* `getRevocRegDeltaResponse`: String
* __->__ [ `revocRegDefId`: String, `revocRegDelta`: Json, `timestamp`: Timestamp (Number) ] - Revocation Registry Definition Id, Revocation Registry Delta json and Timestamp.
```js
{
    "value": Registry-specific data {
        prevAccum: string - previous accumulator value.
        accum: string - current accumulator value.
        issued: array<number> - an array of issued indices.
        revoked: array<number> an array of revoked indices.
    },
    "ver": string - version revocation registry delta json
}
````

Errors: `Common*`

### pairwise

#### isPairwiseExists \( walletHandle, theirDid \) -&gt; exists

Check if pairwise is exists.

* `walletHandle`: Handle (Number) - wallet handler \(created by open\_wallet\).
* `theirDid`: String - encrypted DID
* __->__ `exists`: Boolean - exists: true - if pairwise is exists, false - otherwise

Errors: `Common*`, `Wallet*`

#### createPairwise \( walletHandle, theirDid, myDid, metadata \) -&gt; void

Creates pairwise.

* `walletHandle`: Handle (Number) - wallet handler \(created by open\_wallet\).
* `theirDid`: String - encrypted DID
* `myDid`: String - encrypted DID
metadata Optional: extra information for pairwise
* `metadata`: String
* __->__ void

Errors: `Common*`, `Wallet*`

#### listPairwise \( walletHandle \) -&gt; listPairwise

Get list of saved pairwise.

* `walletHandle`: Handle (Number) - wallet handler \(created by open\_wallet\).
* __->__ `listPairwise`: Json - list\_pairwise: list of saved pairwise

Errors: `Common*`, `Wallet*`

#### getPairwise \( walletHandle, theirDid \) -&gt; pairwiseInfo

Gets pairwise information for specific their\_did.

* `walletHandle`: Handle (Number) - wallet handler \(created by open\_wallet\).
* `theirDid`: String - encoded Did
* __->__ `pairwiseInfo`: Json - pairwise\_info\_json: did info associated with their did

Errors: `Common*`, `Wallet*`

#### setPairwiseMetadata \( walletHandle, theirDid, metadata \) -&gt; void

Save some data in the Wallet for pairwise associated with Did.

* `walletHandle`: Handle (Number) - wallet handler \(created by open\_wallet\).
* `theirDid`: String - encoded Did
* `metadata`: String - some extra information for pairwise
* __->__ void

Errors: `Common*`, `Wallet*`

### pool

#### createPoolLedgerConfig \( configName, config \) -&gt; void

Creates a new local pool ledger configuration that can be used later to connect pool nodes.

* `configName`: String - Name of the pool ledger configuration.
* `config`: Json? - Pool configuration json. if NULL, then default config will be used. Example:
```js
{
    "genesis_txn": string (optional), A path to genesis transaction file. If NULL, then a default one will be used.
                   If file doesn't exists default one will be created.
}
````
* __->__ void

Errors: `Common*`, `Ledger*`

#### openPoolLedger \( configName, config \) -&gt; poolHandle

Opens pool ledger and performs connecting to pool nodes.

Pool ledger configuration with corresponded name must be previously created
with indy\_create\_pool\_ledger\_config method.
It is impossible to open pool with the same name more than once.

config\_name: Name of the pool ledger configuration.
config \(optional\): Runtime pool configuration json.
if NULL, then default config will be used. Example:
```js
{
    "refresh_on_open": bool (optional), Forces pool ledger to be refreshed immediately after opening.
                     Defaults to true.
    "auto_refresh_time": int (optional), After this time in minutes pool ledger will be automatically refreshed.
                       Use 0 to disable automatic refresh. Defaults to 24*60.
    "network_timeout": int (optional), Network timeout for communication with nodes in milliseconds.
                      Defaults to 20000.
}
````

* `configName`: String
* `config`: String
* __->__ `poolHandle`: Handle (Number) - Handle to opened pool to use in methods that require pool connection.

Errors: `Common*`, `Ledger*`

#### refreshPoolLedger \( handle \) -&gt; void

Refreshes a local copy of a pool ledger and updates pool nodes connections.

* `handle`: Handle (Number) - pool handle returned by indy\_open\_pool\_ledger
* __->__ void

Errors: `Common*`, `Ledger*`

#### listPools \(  \) -&gt; pools

Lists names of created pool ledgers

* __->__ `pools`: Json


#### closePoolLedger \( handle \) -&gt; void

Closes opened pool ledger, opened nodes connections and frees allocated resources.

* `handle`: Handle (Number) - pool handle returned by indy\_open\_pool\_ledger.
* __->__ void

Errors: `Common*`, `Ledger*`

#### deletePoolLedgerConfig \( configName \) -&gt; void

Deletes created pool ledger configuration.

* `configName`: String - Name of the pool ledger configuration to delete.
* __->__ void

Errors: `Common*`, `Ledger*`

### wallet

#### createWallet \( poolName, name, xtype, config, credentials \) -&gt; void

Creates a new secure wallet with the given unique name.

* `poolName`: String - Name of the pool that corresponds to this wallet.
* `name`: String - Name of the wallet.
* `xtype`: String? - Type of the wallet. Defaults to 'default'.
Custom types can be registered with indy\_register\_wallet\_type call.
* `config`: String? - Wallet configuration json. List of supported keys are defined by wallet type.
if NULL, then default config will be used.
* `credentials`: String? - Wallet credentials json. List of supported keys are defined by wallet type.
if NULL, then default config will be used.
* __->__ void

Errors: `Common*`, `Wallet*`

#### openWallet \( name, runtimeConfig, credentials \) -&gt; handle

Opens the wallet with specific name.

Wallet with corresponded name must be previously created with indy\_create\_wallet method.
It is impossible to open wallet with the same name more than once.

* `name`: String - Name of the wallet.
* `runtimeConfig`: String? - Runtime wallet configuration json. if NULL, then default runtime\_config will be used. Example:
```js
{
    "freshness_time": string (optional), Amount of minutes to consider wallet value as fresh. Defaults to 24*60.
    ... List of additional supported keys are defined by wallet type.
}
````
* `credentials`: String? - Wallet credentials json. List of supported keys are defined by wallet type.
if NULL, then default credentials will be used.
* __->__ `handle`: Handle (Number) - Handle to opened wallet to use in methods that require wallet access.

Errors: `Common*`, `Wallet*`

#### listWallets \(  \) -&gt; wallets

Lists created wallets as JSON array with each wallet metadata: name, type, name of associated pool

* __->__ `wallets`: Json


#### closeWallet \( handle \) -&gt; void

Closes opened wallet and frees allocated resources.

* `handle`: Handle (Number) - wallet handle returned by indy\_open\_wallet.
* __->__ void

Errors: `Common*`, `Wallet*`

#### deleteWallet \( name, credentials \) -&gt; void

Deletes created wallet.

* `name`: String - Name of the wallet to delete.
* `credentials`: String? - Wallet credentials json. List of supported keys are defined by wallet type.
if NULL, then default credentials will be used.
* __->__ void

Errors: `Common*`, `Wallet*`


[//]: # (CODEGEN-END - don't edit by hand see `codegen/index.js`)

## Contributing

[![JavaScript Style Guide](https://img.shields.io/badge/code_style-standard-brightgreen.svg)](https://standardjs.com)

Setup an Indy SDK environment, and start a local pool.

 * [ubuntu](https://github.com/hyperledger/indy-sdk/blob/master/doc/ubuntu-build.md)
 * [osx](https://github.com/hyperledger/indy-sdk/blob/master/doc/mac-build.md)
 * [windows](https://github.com/hyperledger/indy-sdk/blob/master/doc/windows-build.md)

```sh
# You will need libindy in your system library path. (i.e. /usr/lib/libindy.so for linux)
# or in this directory (i.e. wrappers/nodejs/libindy.so)

# Copy over the libindy header files. This is needed for the build step.
cp -r ../../libindy/include/ .

# Install dependencies and do the initial build.
npm install

# Run the tests
RUST_LOG=trace TEST_POOL_IP=10.0.0.2 npm test
# If you built with libindy locally (i.e. wrappers/nodejs/libindy.so) you need to set LD_LIBRARY_PATH
LD_LIBRARY_PATH=./ RUST_LOG=trace TEST_POOL_IP=10.0.0.2 npm test

# To recompile the native bindings
npm run rebuild
```

Much of the cpp code and README documentation is generated by scripts in the `codegen` folder.
