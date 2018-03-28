# Indy SDK for Node.js

[![stability - experimental](https://img.shields.io/badge/stability-experimental-orange.svg)](https://nodejs.org/api/documentation.html#documentation_stability_index)

Native bindings for [Hyperledger Indy](https://www.hyperledger.org/projects/hyperledger-indy).

## Installing

This module has a native compile step. It compiles C++ code and dynamically links to `libindy`.

You will need:

* C++ build tools and Python 2. See [this](https://github.com/nodejs/node-gyp#installation) for platform recommendations.
* `libindy` on your system in a library path. (i.e. `/usr/lib/libindy.so` for linux)

Then you can install via npm:

```sh
npm install --save indy-sdk
```

## API

```js
var indy = require('indy-sdk')

var did = '...'
var fullVerkey = '...'

indy.abbreviate_verkey(did, fullVerkey, function(err, verkey){
  ..
})

// if you do not provide a callback, a Promise is returned

var verkey = await indy.abbreviate_verkey(did, fullVerkey)
```

### IndyError

All the functions may yield an IndyError. The errors are based on libindy error codes defined [here](https://github.com/hyperledger/indy-sdk/blob/master/libindy/include/indy_mod.h).

* `err.indy_code` - the code number from libindy
* `err.indy_name` - the name string for the code

[//]: # (CODEGEN-START - don't edit by hand see `codegen/index.js`)
#### anoncreds

##### issuer\_create\_and\_store\_claim\_def\(walletHandle, issuerDid, schema, signatureType, createNonRevoc\) -&gt; claim\_def\_json

Create keys \(both primary and revocation\) for the given schema and signature type \(currently only CL signature type is supported\).
Store the keys together with signature type and schema in a secure wallet as a claim definition.

* `walletHandle`: Number
* `issuerDid`: String
* `schema`: Json
* `signatureType`: String
* `createNonRevoc`: Boolean
* __->__ `claimDef`: Json - claim definition json containing information about signature type, schema and issuer's public key.

Errors: Common\*, Wallet\*, Anoncreds\*

##### issuer\_create\_and\_store\_revoc\_reg\(walletHandle, issuerDid, schema, maxClaimNum\) -&gt; revoc\_reg\_json

Create a new revocation registry for the given claim definition.
Stores it in a secure wallet.

* `walletHandle`: Number
* `issuerDid`: String
* `schema`: Json
* `maxClaimNum`: Number
* __->__ `revocReg`: Json - Revocation registry json

Errors: Common\*, Wallet\*, Anoncreds\*

##### issuer\_create\_claim\_offer\(walletHandle, schema, issuerDid, proverDid\) -&gt; claim\_offer\_json

Create claim offer in Wallet

* `walletHandle`: Number
* `schema`: Json
* `issuerDid`: String
* `proverDid`: String
* __->__ `claimOffer`: Json - claim offer json:
       {
           "issuer\_did": string,
           "schema\_key" : {name: string, version: string, did: string},
           "nonce": string,
           "key\_correctness\_proof" : &lt;key\_correctness\_proof&gt;
       }

Errors: Common\*, Wallet\*, Anoncreds\*

##### issuer\_create\_claim\(walletHandle, claimReq, claim, userRevocIndex\) -&gt; \[revoc\_reg\_update\_json, xclaim\_json\]

Signs a given claim values for the given user by a given key \(claim def\).
The corresponding claim definition and revocation registry must be already created
an stored into the wallet.

* `walletHandle`: Number
* `claimReq`: Json
* `claim`: Json
* `userRevocIndex`: Number
* __->__ [`revocRegUpdate`: Json, `xclaim`: Json] - Revocation registry update json with a newly issued claim
Claim json containing signed claim values, issuer\_did, schema\_key, and revoc\_reg\_seq\_no
used for issuance
    {
        "values": &lt;see claim\_values\_json above&gt;,
        "signature": &lt;signature&gt;,
        "revoc\_reg\_seq\_no": int,
        "issuer\_did", string,
        "schema\_key" : {name: string, version: string, did: string},
        "signature\_correctness\_proof": &lt;signature\_correctness\_proof&gt;
    }

Errors: Annoncreds\*, Common\*, Wallet\*

##### issuer\_revoke\_claim\(walletHandle, issuerDid, schema, userRevocIndex\) -&gt; revoc\_reg\_update\_json

Revokes a user identified by a user\_revoc\_index in a given revoc-registry.
The corresponding claim definition and revocation registry must be already
created an stored into the wallet.

* `walletHandle`: Number
* `issuerDid`: String
* `schema`: Json
* `userRevocIndex`: Number
* __->__ `revocRegUpdate`: Json - Revocation registry update json with a revoked claim

Errors: Annoncreds\*, Common\*, Wallet\*

##### prover\_store\_claim\_offer\(walletHandle, claimOffer\) -&gt; void

Stores a claim offer from the given issuer in a secure storage.

* `walletHandle`: Number
* `claimOffer`: Json
* __->__ void

Errors: Common\*, Wallet\*

##### prover\_get\_claim\_offers\(walletHandle, filter\) -&gt; claim\_offers\_json

Gets all stored claim offers \(see prover\_store\_claim\_offer\).
A filter can be specified to get claim offers for specific Issuer, claim\_def or schema only.

* `walletHandle`: Number
* `filter`: Json
* __->__ `claimOffers`: Json - A json with a list of claim offers for the filter.
       {
           \[{
           "issuer\_did": string,
           "schema\_key" : {name: string, version: string, did: string},
           "nonce": string,
           "key\_correctness\_proof" : &lt;key\_correctness\_proof&gt;
           }\]
       }

Errors: Common\*, Wallet\*

##### prover\_create\_master\_secret\(walletHandle, masterSecretName\) -&gt; void

Creates a master secret with a given name and stores it in the wallet.
The name must be unique.

* `walletHandle`: Number
* `masterSecretName`: String
* __->__ void

Errors: Annoncreds\*, Common\*, Wallet\*

##### prover\_create\_and\_store\_claim\_req\(walletHandle, proverDid, claimOffer, claimDef, masterSecretName\) -&gt; claim\_req\_json

Creates a clam request json for the given claim offer and stores it in a secure wallet.
The claim offer contains the information about Issuer \(DID, schema\_seq\_no\),
and the schema \(schema\_key\).
The method creates a blinded master secret for a master secret identified by a provided name.
The master secret identified by the name must be already stored in the secure wallet \(see prover\_create\_master\_secret\)
The blinded master secret is a part of the claim request.

* `walletHandle`: Number
* `proverDid`: String
* `claimOffer`: Json
* `claimDef`: Json
* `masterSecretName`: String
* __->__ `claimReq`: Json - Claim request json.
    {
     "blinded\_ms" : &lt;blinded\_master\_secret&gt;,
     "schema\_key" : {name: string, version: string, did: string},
     "issuer\_did" : string,
     "prover\_did" : string,
     "blinded\_ms\_correctness\_proof" : &lt;blinded\_ms\_correctness\_proof&gt;,
     "nonce": string
   }

Errors: Annoncreds\*, Common\*, Wallet\*

##### prover\_store\_claim\(walletHandle, claims, revReg\) -&gt; void

Updates the claim by a master secret and stores in a secure wallet.
The claim contains the information about
schema\_key, issuer\_did, revoc\_reg\_seq\_no \(see issuer\_create\_claim\).
Seq\_no is a sequence number of the corresponding transaction in the ledger.
The method loads a blinded secret for this key from the wallet,
updates the claim and stores it in a wallet.

* `walletHandle`: Number
* `claims`: Json
* `revReg`: Json
* __->__ void

Errors: Annoncreds\*, Common\*, Wallet\*

##### prover\_get\_claims\(walletHandle, filter\) -&gt; claims\_json

Gets human readable claims according to the filter.
If filter is NULL, then all claims are returned.
Claims can be filtered by Issuer, claim\_def and\/or Schema.

* `walletHandle`: Number
* `filter`: Json
* __->__ `claims`: Json - claims json
    \[{
        "referent": &lt;string&gt;,
        "attrs": \[{"attr\_name" : "attr\_raw\_value"}\],
        "schema\_key" : {name: string, version: string, did: string},
        "issuer\_did": string,
        "revoc\_reg\_seq\_no": int,
    }\]

Errors: Annoncreds\*, Common\*, Wallet\*

##### prover\_get\_claims\_for\_proof\_req\(walletHandle, proofRequest\) -&gt; claims\_json

Gets human readable claims matching the given proof request.

* `walletHandle`: Number
* `proofRequest`: Json
* __->__ `claims`: Json - json with claims for the given pool request.
Claim consists of referent, human-readable attributes \(key-value map\), schema\_key, issuer\_did and revoc\_reg\_seq\_no.
    {
        "requested\_attr1\_referent": \[claim1, claim2\],
        "requested\_attr2\_referent": \[\],
        "requested\_attr3\_referent": \[claim3\],
        "requested\_predicate\_1\_referent": \[claim1, claim3\],
        "requested\_predicate\_2\_referent": \[claim2\],
    }, where claim is
    {
        "referent": &lt;string&gt;,
        "attrs": \[{"attr\_name" : "attr\_raw\_value"}\],
        "schema\_key" : {name: string, version: string, did: string},
        "issuer\_did": string,
        "revoc\_reg\_seq\_no": int
    }

Errors: Annoncreds\*, Common\*, Wallet\*

##### prover\_create\_proof\(walletHandle, proofReq, requestedClaims, schemas, masterSecretName, claimDefs, revocRegs\) -&gt; proof\_json

Creates a proof according to the given proof request
Either a corresponding claim with optionally revealed attributes or self-attested attribute must be provided
for each requested attribute \(see indy\_prover\_get\_claims\_for\_pool\_req\).
A proof request may request multiple claims from different schemas and different issuers.
All required schemas, public keys and revocation registries must be provided.
The proof request also contains nonce.
The proof contains either proof or self-attested attribute value for each requested attribute.

* `walletHandle`: Number
* `proofReq`: Json
* `requestedClaims`: Json
* `schemas`: Json
* `masterSecretName`: String
* `claimDefs`: Json
* `revocRegs`: Json
* __->__ `proof`: Json - Proof json
For each requested attribute either a proof \(with optionally revealed attribute value\) or
self-attested attribute value is provided.
Each proof is associated with a claim and corresponding schema\_seq\_no, issuer\_did and revoc\_reg\_seq\_no.
There ais also aggregated proof part common for all claim proofs.
    {
        "requested": {
            "requested\_attr1\_id": \[claim\_proof1\_referent, revealed\_attr1, revealed\_attr1\_as\_int\],
            "requested\_attr2\_id": \[self\_attested\_attribute\],
            "requested\_attr3\_id": \[claim\_proof2\_referent\]
            "requested\_attr4\_id": \[claim\_proof2\_referent, revealed\_attr4, revealed\_attr4\_as\_int\],
            "requested\_predicate\_1\_referent": \[claim\_proof2\_referent\],
            "requested\_predicate\_2\_referent": \[claim\_proof3\_referent\],
        }
        "proof": {
            "proofs": {
                "claim\_proof1\_referent": &lt;claim\_proof&gt;,
                "claim\_proof2\_referent": &lt;claim\_proof&gt;,
                "claim\_proof3\_referent": &lt;claim\_proof&gt;
            },
            "aggregated\_proof": &lt;aggregated\_proof&gt;
        }
        "identifiers": {"claim\_proof1\_referent":{issuer\_did, rev\_reg\_seq\_no, schema\_key: {name, version, did}}}
    }

Errors: Annoncreds\*, Common\*, Wallet\*

##### verifier\_verify\_proof\(proofRequest, proof, schemas, claimDefsJsons, revocRegs\) -&gt; valid

Verifies a proof \(of multiple claim\).
All required schemas, public keys and revocation registries must be provided.

* `proofRequest`: Json
* `proof`: Json
* `schemas`: Json
* `claimDefsJsons`: Json
* `revocRegs`: Json
* __->__ `valid`: Boolean - valid: true - if signature is valid, false - otherwise

Errors: Annoncreds\*, Common\*, Wallet\*

#### crypto

##### create\_key\(walletHandle, key\) -&gt; vk

Creates keys pair and stores in the wallet.

* `walletHandle`: Number
* `key`: Json
* __->__ `vk`: String - Error Code
cb:
- xcommand\_handle: command handle to map callback to caller context.
- err: Error code.
- verkey: Ver key of generated key pair, also used as key identifier

Errors: Common\*, Wallet\*, Crypto\*

##### set\_key\_metadata\(walletHandle, verkey, metadata\) -&gt; void

Saves\/replaces the meta information for the giving key in the wallet.

* `walletHandle`: Number
* `verkey`: String
* `metadata`: String
* __->__ void - Error Code
cb:
- xcommand\_handle: command handle to map callback to caller context.
- err: Error code.

Errors: Common\*, Wallet\*, Crypto\*

##### get\_key\_metadata\(walletHandle, verkey\) -&gt; metadata

Retrieves the meta information for the giving key in the wallet.

* `walletHandle`: Number
* `verkey`: String
* __->__ `metadata`: String - Error Code
cb:
- xcommand\_handle: Command handle to map callback to caller context.
- err: Error code.
- metadata - The meta information stored with the key; Can be null if no metadata was saved for this key.

Errors: Common\*, Wallet\*, Crypto\*

##### crypto\_sign\(walletHandle, myVk, messageRaw\) -&gt; signature\_raw

Signs a message with a key.

Note to use DID keys with this function you can call indy\_key\_for\_did to get key id \(verkey\)
for specific DID.

* `walletHandle`: Number
* `myVk`: String
* `messageRaw`: Buffer
* __->__ `signatureRaw`: Buffer - a signature string

Errors: Common\*, Wallet\*, Crypto\*

##### crypto\_verify\(theirVk, messageRaw, signatureRaw\) -&gt; valid

Verify a signature with a verkey.

Note to use DID keys with this function you can call indy\_key\_for\_did to get key id \(verkey\)
for specific DID.

* `theirVk`: String
* `messageRaw`: Buffer
* `signatureRaw`: Buffer
* __->__ `valid`: Boolean - valid: true - if signature is valid, false - otherwise

Errors: Common\*, Wallet\*, Ledger\*, Crypto\*

##### crypto\_auth\_crypt\(walletHandle, myVk, theirVk, messageRaw\) -&gt; encrypted\_msg\_raw

Encrypt a message by authenticated-encryption scheme.

Sender can encrypt a confidential message specifically for Recipient, using Sender's public key.
Using Recipient's public key, Sender can compute a shared secret key.
Using Sender's public key and his secret key, Recipient can compute the exact same shared secret key.
That shared secret key can be used to verify that the encrypted message was not tampered with,
before eventually decrypting it.

Note to use DID keys with this function you can call indy\_key\_for\_did to get key id \(verkey\)
for specific DID.

* `walletHandle`: Number
* `myVk`: String
* `theirVk`: String
* `messageRaw`: Buffer
* __->__ `encryptedMsgRaw`: Buffer - an encrypted message

Errors: Common\*, Wallet\*, Ledger\*, Crypto\*

##### crypto\_auth\_decrypt\(walletHandle, myVk, encryptedMsgRaw\) -&gt; \[their\_vk, decrypted\_msg\_raw\]

Decrypt a message by authenticated-encryption scheme.

Sender can encrypt a confidential message specifically for Recipient, using Sender's public key.
Using Recipient's public key, Sender can compute a shared secret key.
Using Sender's public key and his secret key, Recipient can compute the exact same shared secret key.
That shared secret key can be used to verify that the encrypted message was not tampered with,
before eventually decrypting it.

Note to use DID keys with this function you can call indy\_key\_for\_did to get key id \(verkey\)
for specific DID.

* `walletHandle`: Number
* `myVk`: String
* `encryptedMsgRaw`: Buffer
* __->__ [`theirVk`: String, `decryptedMsgRaw`: Buffer] - sender verkey and decrypted message

Errors: Common\*, Wallet\*, Crypto\*

##### crypto\_anon\_crypt\(theirVk, messageRaw\) -&gt; encrypted\_msg\_raw

Encrypts a message by anonymous-encryption scheme.

Sealed boxes are designed to anonymously send messages to a Recipient given its public key.
Only the Recipient can decrypt these messages, using its private key.
While the Recipient can verify the integrity of the message, it cannot verify the identity of the Sender.

Note to use DID keys with this function you can call indy\_key\_for\_did to get key id \(verkey\)
for specific DID.

* `theirVk`: String
* `messageRaw`: Buffer
* __->__ `encryptedMsgRaw`: Buffer - an encrypted message

Errors: Common\*, Wallet\*, Ledger\*, Crypto\*

##### crypto\_anon\_decrypt\(walletHandle, myVk, encryptedMsg\) -&gt; decrypted\_msg\_raw

Decrypts a message by anonymous-encryption scheme.

Sealed boxes are designed to anonymously send messages to a Recipient given its public key.
Only the Recipient can decrypt these messages, using its private key.
While the Recipient can verify the integrity of the message, it cannot verify the identity of the Sender.

Note to use DID keys with this function you can call indy\_key\_for\_did to get key id \(verkey\)
for specific DID.

* `walletHandle`: Number
* `myVk`: String
* `encryptedMsg`: Buffer
* __->__ `decryptedMsgRaw`: Buffer - decrypted message

Errors: Common\*, Wallet\*, Crypto\*

#### did

##### create\_and\_store\_my\_did\(walletHandle, did\) -&gt; \[did, verkey\]

Creates keys \(signing and encryption keys\) for a new
DID \(owned by the caller of the library\).
Identity's DID must be either explicitly provided, or taken as the first 16 bit of verkey.
Saves the Identity DID with keys in a secured Wallet, so that it can be used to sign
and encrypt transactions.

* `walletHandle`: Number
* `did`: Json
* __->__ [`did`: String, `verkey`: String] - DID, verkey \(for verification of signature\) and public\_key \(for decryption\)

Errors: Common\*, Wallet\*, Crypto\*

##### replace\_keys\_start\(walletHandle, did, identity\) -&gt; verkey

Generated temporary keys \(signing and encryption keys\) for an existing
DID \(owned by the caller of the library\).

* `walletHandle`: Number
* `did`: String
* `identity`: Json
* __->__ `verkey`: String - verkey \(for verification of signature\) and public\_key \(for decryption\)

Errors: Common\*, Wallet\*, Crypto\*

##### replace\_keys\_apply\(walletHandle, did\) -&gt; void

Apply temporary keys as main for an existing DID \(owned by the caller of the library\).

* `walletHandle`: Number
* `did`: String
* __->__ void

Errors: Common\*, Wallet\*, Crypto\*

##### store\_their\_did\(walletHandle, identity\) -&gt; void

Saves their DID for a pairwise connection in a secured Wallet,
so that it can be used to verify transaction.

* `walletHandle`: Number
* `identity`: Json
* __->__ void

Errors: Common\*, Wallet\*, Crypto\*

##### key\_for\_did\(poolHandle, walletHandle, did\) -&gt; key

Returns ver key \(key id\) for the given DID.

"indy\_key\_for\_did" call follow the idea that we resolve information about their DID from
the ledger with cache in the local wallet. The "indy\_open\_wallet" call has freshness parameter
that is used for checking the freshness of cached pool value.

Note if you don't want to resolve their DID info from the ledger you can use
"indy\_key\_for\_local\_did" call instead that will look only to the local wallet and skip
freshness checking.

Note that "indy\_create\_and\_store\_my\_did" makes similar wallet record as "indy\_create\_key".
As result we can use returned ver key in all generic crypto and messaging functions.

* `poolHandle`: Number
* `walletHandle`: Number
* `did`: String
* __->__ `key`: String - Error Code
cb:
- xcommand\_handle: Command handle to map callback to caller context.
- err: Error code.
- key - The DIDs ver key \(key id\).

Errors: Common\*, Wallet\*, Crypto\*

##### key\_for\_local\_did\(walletHandle, did\) -&gt; key

Returns ver key \(key id\) for the given DID.

"indy\_key\_for\_local\_did" call looks data stored in the local wallet only and skips freshness
checking.

Note if you want to get fresh data from the ledger you can use "indy\_key\_for\_did" call
instead.

Note that "indy\_create\_and\_store\_my\_did" makes similar wallet record as "indy\_create\_key".
As result we can use returned ver key in all generic crypto and messaging functions.

* `walletHandle`: Number
* `did`: String
* __->__ `key`: String - Error Code
cb:
- xcommand\_handle: Command handle to map callback to caller context.
- err: Error code.
- key - The DIDs ver key \(key id\).

Errors: Common\*, Wallet\*, Crypto\*

##### set\_endpoint\_for\_did\(walletHandle, did, address, transportKey\) -&gt; void

Returns endpoint information for the given DID.

* `walletHandle`: Number
* `did`: String
* `address`: String
* `transportKey`: String
* __->__ void - Error Code
cb:
- xcommand\_handle: Command handle to map callback to caller context.
- err: Error code.
- endpoint - The DIDs endpoint.
- transport\_vk - The DIDs transport key \(ver key, key id\).

Errors: Common\*, Wallet\*, Crypto\*

##### get\_endpoint\_for\_did\(walletHandle, poolHandle, did\) -&gt; \[address, transport\_vk\]



* `walletHandle`: Number
* `poolHandle`: Number
* `did`: String
* __->__ [`address`: String, `transportVk`: String]


##### set\_did\_metadata\(walletHandle, did, metadata\) -&gt; void

Saves\/replaces the meta information for the giving DID in the wallet.

* `walletHandle`: Number
* `did`: String
* `metadata`: String
* __->__ void - Error Code
cb:
- xcommand\_handle: command handle to map callback to caller context.
- err: Error code.

Errors: Common\*, Wallet\*, Crypto\*

##### get\_did\_metadata\(walletHandle, did\) -&gt; metadata

Retrieves the meta information for the giving DID in the wallet.

* `walletHandle`: Number
* `did`: String
* __->__ `metadata`: String - Error Code
cb:
- xcommand\_handle: Command handle to map callback to caller context.
- err: Error code.
- metadata - The meta information stored with the DID; Can be null if no metadata was saved for this DID.

Errors: Common\*, Wallet\*, Crypto\*

##### get\_my\_did\_with\_meta\(walletHandle, myDid\) -&gt; did\_with\_meta

Get info about My DID in format: DID, verkey, metadata

* `walletHandle`: Number
* `myDid`: String
* __->__ `didWithMeta`: String


##### list\_my\_dids\_with\_meta\(walletHandle\) -&gt; dids

Lists created DIDs with metadata as JSON array with each DID in format: DID, verkey, metadata

* `walletHandle`: Number
* __->__ `dids`: String


##### abbreviate\_verkey\(did, fullVerkey\) -&gt; verkey

Retrieves abbreviated verkey if it is possible otherwise return full verkey.

* `did`: String
* `fullVerkey`: String
* __->__ `verkey`: String


#### ledger

##### sign\_and\_submit\_request\(poolHandle, walletHandle, submitterDid, request\) -&gt; request\_result\_json

Signs and submits request message to validator pool.

Adds submitter information to passed request json, signs it with submitter
sign key \(see wallet\_sign\), and sends signed request message
to validator pool \(see write\_request\).

* `poolHandle`: Number
* `walletHandle`: Number
* `submitterDid`: String
* `request`: Json
* __->__ `requestResult`: Json

Errors: Common\*, Wallet\*, Ledger\*, Crypto\*

##### submit\_request\(poolHandle, request\) -&gt; request\_result\_json

Publishes request message to validator pool \(no signing, unlike sign\_and\_submit\_request\).

The request is sent to the validator pool as is. It's assumed that it's already prepared.

* `poolHandle`: Number
* `request`: Json
* __->__ `requestResult`: Json

Errors: Common\*, Ledger\*

##### sign\_request\(walletHandle, submitterDid, request\) -&gt; signed\_request\_json

Signs request message.

Adds submitter information to passed request json, signs it with submitter
sign key \(see wallet\_sign\).

* `walletHandle`: Number
* `submitterDid`: String
* `request`: Json
* __->__ `signedRequest`: Json - Signed request json.

Errors: Common\*, Wallet\*, Ledger\*, Crypto\*

##### build\_get\_ddo\_request\(submitterDid, targetDid\) -&gt; request\_result\_json

Builds a request to get a DDO.

* `submitterDid`: String
* `targetDid`: String
* __->__ `requestResult`: Json

Errors: Common\*

##### build\_nym\_request\(submitterDid, targetDid, verkey, alias, role\) -&gt; request\_json

Builds a NYM request.

* `submitterDid`: String
* `targetDid`: String
* `verkey`: String - verification key
* `alias`: String - alias
* `role`: String - Role of a user NYM record
* __->__ `request`: Json

Errors: Common\*

##### build\_attrib\_request\(submitterDid, targetDid, hash, raw, enc\) -&gt; request\_json

Builds an ATTRIB request.

* `submitterDid`: String
* `targetDid`: String
* `hash`: String - Hash of attribute data
* `raw`: String - represented as json, where key is attribute name and value is it's value
* `enc`: String - Encrypted attribute data
* __->__ `request`: Json

Errors: Common\*

##### build\_get\_attrib\_request\(submitterDid, targetDid, hash, raw, enc\) -&gt; request\_json

Builds a GET\_ATTRIB request.

* `submitterDid`: String
* `targetDid`: String
* `hash`: String
* `raw`: String
* `enc`: String
* __->__ `request`: Json

Errors: Common\*

##### build\_get\_nym\_request\(submitterDid, targetDid\) -&gt; request\_json

Builds a GET\_NYM request.

* `submitterDid`: String
* `targetDid`: String
* __->__ `request`: Json

Errors: Common\*

##### build\_schema\_request\(submitterDid, data\) -&gt; request\_json

Builds a SCHEMA request.

* `submitterDid`: String
* `data`: String - name, version, type, attr\_names \(ip, port, keys\)
* __->__ `request`: Json

Errors: Common\*

##### build\_get\_schema\_request\(submitterDid, dest, data\) -&gt; request\_json

Builds a GET\_SCHEMA request.

* `submitterDid`: String
* `dest`: String - Id of Identity stored in secured Wallet.
* `data`: String - name, version
* __->__ `request`: Json

Errors: Common\*

##### build\_claim\_def\_txn\(submitterDid, xref, signatureType, data\) -&gt; request\_json

Builds an CLAIM\_DEF request.

* `submitterDid`: String
* `xref`: Number - Seq. number of schemasignature\_type: signature type \(only CL supported now\)
* `signatureType`: String
* `data`: String - components of a key in json: N, R, S, Z
* __->__ `request`: Json

Errors: Common\*

##### build\_get\_claim\_def\_txn\(submitterDid, xref, signatureType, origin\) -&gt; request\_json

Builds a GET\_CLAIM\_DEF request.

* `submitterDid`: String
* `xref`: Number - Seq. number of schemasignature\_type: signature type \(only CL supported now\)
* `signatureType`: String
* `origin`: String - issuer did
* __->__ `request`: Json

Errors: Common\*

##### build\_node\_request\(submitterDid, targetDid, data\) -&gt; request\_json

Builds a NODE request.

* `submitterDid`: String
* `targetDid`: String
* `data`: String - id of a target NYM record
* __->__ `request`: Json

Errors: Common\*

##### build\_get\_txn\_request\(submitterDid, data\) -&gt; request\_json

Builds a GET\_TXN request.

* `submitterDid`: String
* `data`: Number - seq\_no of transaction in ledger
* __->__ `request`: Json

Errors: Common\*

##### build\_pool\_config\_request\(submitterDid, writes, force\) -&gt; request\_json

Builds a POOL\_CONFIG request.

* `submitterDid`: String
* `writes`: Boolean
* `force`: Boolean
* __->__ `request`: Json

Errors: Common\*

##### build\_pool\_upgrade\_request\(submitterDid, name, version, action, sha256, timeout, schedule, justification, reinstall, force\) -&gt; request\_json

Builds a POOL\_UPGRADE request.

* `submitterDid`: String
* `name`: String
* `version`: String
* `action`: String - Either start or cancel
* `sha256`: String
* `timeout`: Number
* `schedule`: String
* `justification`: String
* `reinstall`: Boolean
* `force`: Boolean
* __->__ `request`: Json

Errors: Common\*

#### pairwise

##### is\_pairwise\_exists\(walletHandle, theirDid\) -&gt; exists

Check if pairwise is exists.

* `walletHandle`: Number
* `theirDid`: String
* __->__ `exists`: Boolean - exists: true - if pairwise is exists, false - otherwise

Errors: Common\*, Wallet\*

##### create\_pairwise\(walletHandle, theirDid, myDid, metadata\) -&gt; void

Creates pairwise.

* `walletHandle`: Number
* `theirDid`: String
* `myDid`: String
* `metadata`: String
* __->__ void

Errors: Common\*, Wallet\*

##### list\_pairwise\(walletHandle\) -&gt; list\_pairwise

Get list of saved pairwise.

* `walletHandle`: Number
* __->__ `listPairwise`: String - list\_pairwise: list of saved pairwise

Errors: Common\*, Wallet\*

##### get\_pairwise\(walletHandle, theirDid\) -&gt; pairwise\_info\_json

Gets pairwise information for specific their\_did.

* `walletHandle`: Number
* `theirDid`: String
* __->__ `pairwiseInfo`: Json - pairwise\_info\_json: did info associated with their did

Errors: Common\*, Wallet\*

##### set\_pairwise\_metadata\(walletHandle, theirDid, metadata\) -&gt; void

Save some data in the Wallet for pairwise associated with Did.

* `walletHandle`: Number
* `theirDid`: String
* `metadata`: String - some extra information for pairwise
* __->__ void

Errors: Common\*, Wallet\*

#### pool

##### create\_pool\_ledger\_config\(configName, config\) -&gt; void

Creates a new local pool ledger configuration that can be used later to connect pool nodes.

* `configName`: String
* `config`: Json? - Pool configuration json. if NULL, then default config will be used. Example:{
    "genesis\_txn": string \(optional\), A path to genesis transaction file. If NULL, then a default one will be used.
                   If file doesn't exists default one will be created.
}
* __->__ void

Errors: Common\*, Ledger\*

##### open\_pool\_ledger\(configName, config\) -&gt; pool\_handle

Opens pool ledger and performs connecting to pool nodes.

Pool ledger configuration with corresponded name must be previously created
with indy\_create\_pool\_ledger\_config method.
It is impossible to open pool with the same name more than once.

config\_name: Name of the pool ledger configuration.
config \(optional\): Runtime pool configuration json.
                        if NULL, then default config will be used. Example:
{
    "refresh\_on\_open": bool \(optional\), Forces pool ledger to be refreshed immediately after opening.
                     Defaults to true.
    "auto\_refresh\_time": int \(optional\), After this time in minutes pool ledger will be automatically refreshed.
                       Use 0 to disable automatic refresh. Defaults to 24\*60.
    "network\_timeout": int \(optional\), Network timeout for communication with nodes in milliseconds.
                      Defaults to 20000.
}

* `configName`: String
* `config`: String
* __->__ `poolHandle`: Number - Handle to opened pool to use in methods that require pool connection.

Errors: Common\*, Ledger\*

##### refresh\_pool\_ledger\(handle\) -&gt; void

Refreshes a local copy of a pool ledger and updates pool nodes connections.

* `handle`: Number - pool handle returned by indy\_open\_pool\_ledger
* __->__ void

Errors: Common\*, Ledger\*

##### list\_pools\(\) -&gt; pools

Lists names of created pool ledgers

* __->__ `pools`: Json


##### close\_pool\_ledger\(handle\) -&gt; void

Closes opened pool ledger, opened nodes connections and frees allocated resources.

* `handle`: Number - pool handle returned by indy\_open\_pool\_ledger.
* __->__ void

Errors: Common\*, Ledger\*

##### delete\_pool\_ledger\_config\(configName\) -&gt; void

Deletes created pool ledger configuration.

* `configName`: String
* __->__ void

Errors: Common\*, Ledger\*

#### wallet

##### create\_wallet\(poolName, name, xtype, config, credentials\) -&gt; void

Creates a new secure wallet with the given unique name.

* `poolName`: String
* `name`: String - Name of the wallet.
* `xtype`: String? - Type of the wallet. Defaults to 'default'.                 Custom types can be registered with indy\_register\_wallet\_type call.
* `config`: String? - Wallet configuration json. List of supported keys are defined by wallet type.                   if NULL, then default config will be used.
* `credentials`: String? - Wallet credentials json. List of supported keys are defined by wallet type.                   if NULL, then default config will be used.
* __->__ void

Errors: Common\*, Wallet\*

##### open\_wallet\(name, runtimeConfig, credentials\) -&gt; handle

Opens the wallet with specific name.

Wallet with corresponded name must be previously created with indy\_create\_wallet method.
It is impossible to open wallet with the same name more than once.

* `name`: String - Name of the wallet.runtime\_config \(optional\): Runtime wallet configuration json. if NULL, then default runtime\_config will be used. Example:
{
    "freshness\_time": string \(optional\), Amount of minutes to consider wallet value as fresh. Defaults to 24\*60.
    ... List of additional supported keys are defined by wallet type.
}
* `runtimeConfig`: String
* `credentials`: String? - Wallet credentials json. List of supported keys are defined by wallet type.                   if NULL, then default credentials will be used.
* __->__ `handle`: Number - Handle to opened wallet to use in methods that require wallet access.

Errors: Common\*, Wallet\*

##### list\_wallets\(\) -&gt; wallets

Lists created wallets as JSON array with each wallet metadata: name, type, name of associated pool

* __->__ `wallets`: Json


##### close\_wallet\(handle\) -&gt; void

Closes opened wallet and frees allocated resources.

* `handle`: Number - wallet handle returned by indy\_open\_wallet.
* __->__ void

Errors: Common\*, Wallet\*

##### delete\_wallet\(name, credentials\) -&gt; void

Deletes created wallet.

* `name`: String - Name of the wallet to delete.
* `credentials`: String? - Wallet credentials json. List of supported keys are defined by wallet type.                   if NULL, then default credentials will be used.
* __->__ void

Errors: Common\*, Wallet\*


[//]: # (CODEGEN-END - don't edit by hand see `codegen/index.js`)

## How to contribute to this wrapper

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
