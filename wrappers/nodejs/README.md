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

This is still experimental, please submit issues to: https://github.com/Picolab/indy-sdk/issues

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

These errors are based on libindy error codes defined [here](https://github.com/hyperledger/indy-sdk/blob/master/libindy/include/indy_mod.h).

* `err.indy_code` - the code number from libindy
* `err.indy_name` - the name string for the code

[//]: # (CODEGEN-START - don't edit by hand see `codegen/index.js`)
#### issuer\_create\_and\_store\_claim\_def(walletHandle, issuerDid, schema, signatureType, createNonRevoc) -> claimDef
* `walletHandle`: Number
* `issuerDid`: String
* `schema`: Json
* `signatureType`: String
* `createNonRevoc`: Boolean
* __->__ `claimDef`: Json

#### issuer\_create\_and\_store\_revoc\_reg(walletHandle, issuerDid, schema, maxClaimNum) -> revocReg
* `walletHandle`: Number
* `issuerDid`: String
* `schema`: Json
* `maxClaimNum`: Number
* __->__ `revocReg`: Json

#### issuer\_create\_claim\_offer(walletHandle, schema, issuerDid, proverDid) -> claimOffer
* `walletHandle`: Number
* `schema`: Json
* `issuerDid`: String
* `proverDid`: String
* __->__ `claimOffer`: Json

#### issuer\_create\_claim(walletHandle, claimReq, claim, userRevocIndex) -> [revocRegUpdate, xclaim]
* `walletHandle`: Number
* `claimReq`: Json
* `claim`: Json
* `userRevocIndex`: Number
* __->__ [`revocRegUpdate`: Json, `xclaim`: Json]

#### issuer\_revoke\_claim(walletHandle, issuerDid, schema, userRevocIndex) -> revocRegUpdate
* `walletHandle`: Number
* `issuerDid`: String
* `schema`: Json
* `userRevocIndex`: Number
* __->__ `revocRegUpdate`: Json

#### prover\_store\_claim\_offer(walletHandle, claimOffer) -> void
* `walletHandle`: Number
* `claimOffer`: Json

#### prover\_get\_claim\_offers(walletHandle, filter) -> claimOffers
* `walletHandle`: Number
* `filter`: Json
* __->__ `claimOffers`: Json

#### prover\_create\_master\_secret(walletHandle, masterSecretName) -> void
* `walletHandle`: Number
* `masterSecretName`: String

#### prover\_create\_and\_store\_claim\_req(walletHandle, proverDid, claimOffer, claimDef, masterSecretName) -> claimReq
* `walletHandle`: Number
* `proverDid`: String
* `claimOffer`: Json
* `claimDef`: Json
* `masterSecretName`: String
* __->__ `claimReq`: Json

#### prover\_store\_claim(walletHandle, claims, revReg) -> void
* `walletHandle`: Number
* `claims`: Json
* `revReg`: Json

#### prover\_get\_claims(walletHandle, filter) -> claims
* `walletHandle`: Number
* `filter`: Json
* __->__ `claims`: Json

#### prover\_get\_claims\_for\_proof\_req(walletHandle, proofRequest) -> claims
* `walletHandle`: Number
* `proofRequest`: Json
* __->__ `claims`: Json

#### prover\_create\_proof(walletHandle, proofReq, requestedClaims, schemas, masterSecretName, claimDefs, revocRegs) -> proof
* `walletHandle`: Number
* `proofReq`: Json
* `requestedClaims`: Json
* `schemas`: Json
* `masterSecretName`: String
* `claimDefs`: Json
* `revocRegs`: Json
* __->__ `proof`: Json

#### verifier\_verify\_proof(proofRequest, proof, schemas, claimDefsJsons, revocRegs) -> valid
* `proofRequest`: Json
* `proof`: Json
* `schemas`: Json
* `claimDefsJsons`: Json
* `revocRegs`: Json
* __->__ `valid`: Boolean

#### create\_key(walletHandle, key) -> vk
* `walletHandle`: Number
* `key`: Json
* __->__ `vk`: String

#### set\_key\_metadata(walletHandle, verkey, metadata) -> void
* `walletHandle`: Number
* `verkey`: String
* `metadata`: String

#### get\_key\_metadata(walletHandle, verkey) -> metadata
* `walletHandle`: Number
* `verkey`: String
* __->__ `metadata`: String

#### crypto\_sign(walletHandle, myVk, messageRaw) -> signatureRaw
* `walletHandle`: Number
* `myVk`: String
* `messageRaw`: Buffer
* __->__ `signatureRaw`: Buffer

#### crypto\_verify(theirVk, messageRaw, signatureRaw) -> valid
* `theirVk`: String
* `messageRaw`: Buffer
* `signatureRaw`: Buffer
* __->__ `valid`: Boolean

#### crypto\_auth\_crypt(walletHandle, myVk, theirVk, messageRaw) -> encryptedMsgRaw
* `walletHandle`: Number
* `myVk`: String
* `theirVk`: String
* `messageRaw`: Buffer
* __->__ `encryptedMsgRaw`: Buffer

#### crypto\_auth\_decrypt(walletHandle, myVk, encryptedMsgRaw) -> [theirVk, decryptedMsgRaw]
* `walletHandle`: Number
* `myVk`: String
* `encryptedMsgRaw`: Buffer
* __->__ [`theirVk`: String, `decryptedMsgRaw`: Buffer]

#### crypto\_anon\_crypt(theirVk, messageRaw) -> encryptedMsgRaw
* `theirVk`: String
* `messageRaw`: Buffer
* __->__ `encryptedMsgRaw`: Buffer

#### crypto\_anon\_decrypt(walletHandle, myVk, encryptedMsg) -> decryptedMsgRaw
* `walletHandle`: Number
* `myVk`: String
* `encryptedMsg`: Buffer
* __->__ `decryptedMsgRaw`: Buffer

#### create\_and\_store\_my\_did(walletHandle, did) -> [did, verkey]
* `walletHandle`: Number
* `did`: Json
* __->__ [`did`: String, `verkey`: String]

#### replace\_keys\_start(walletHandle, did, identity) -> verkey
* `walletHandle`: Number
* `did`: String
* `identity`: Json
* __->__ `verkey`: String

#### replace\_keys\_apply(walletHandle, did) -> void
* `walletHandle`: Number
* `did`: String

#### store\_their\_did(walletHandle, identity) -> void
* `walletHandle`: Number
* `identity`: Json

#### key\_for\_did(poolHandle, walletHandle, did) -> key
* `poolHandle`: Number
* `walletHandle`: Number
* `did`: String
* __->__ `key`: String

#### key\_for\_local\_did(walletHandle, did) -> key
* `walletHandle`: Number
* `did`: String
* __->__ `key`: String

#### set\_endpoint\_for\_did(walletHandle, did, address, transportKey) -> void
* `walletHandle`: Number
* `did`: String
* `address`: String
* `transportKey`: String

#### get\_endpoint\_for\_did(walletHandle, poolHandle, did) -> [address, transportVk]
* `walletHandle`: Number
* `poolHandle`: Number
* `did`: String
* __->__ [`address`: String, `transportVk`: String]

#### set\_did\_metadata(walletHandle, did, metadata) -> void
* `walletHandle`: Number
* `did`: String
* `metadata`: String

#### get\_did\_metadata(walletHandle, did) -> metadata
* `walletHandle`: Number
* `did`: String
* __->__ `metadata`: String

#### get\_my\_did\_with\_meta(walletHandle, myDid) -> didWithMeta
* `walletHandle`: Number
* `myDid`: String
* __->__ `didWithMeta`: String

#### list\_my\_dids\_with\_meta(walletHandle) -> dids
* `walletHandle`: Number
* __->__ `dids`: String

#### abbreviate\_verkey(did, fullVerkey) -> verkey
* `did`: String
* `fullVerkey`: String
* __->__ `verkey`: String

#### sign\_and\_submit\_request(poolHandle, walletHandle, submitterDid, request) -> requestResult
* `poolHandle`: Number
* `walletHandle`: Number
* `submitterDid`: String
* `request`: Json
* __->__ `requestResult`: Json

#### submit\_request(poolHandle, request) -> requestResult
* `poolHandle`: Number
* `request`: Json
* __->__ `requestResult`: Json

#### sign\_request(walletHandle, submitterDid, request) -> signedRequest
* `walletHandle`: Number
* `submitterDid`: String
* `request`: Json
* __->__ `signedRequest`: Json

#### build\_get\_ddo\_request(submitterDid, targetDid) -> requestResult
* `submitterDid`: String
* `targetDid`: String
* __->__ `requestResult`: Json

#### build\_nym\_request(submitterDid, targetDid, verkey, alias, role) -> request
* `submitterDid`: String
* `targetDid`: String
* `verkey`: String
* `alias`: String
* `role`: String
* __->__ `request`: Json

#### build\_attrib\_request(submitterDid, targetDid, hash, raw, enc) -> request
* `submitterDid`: String
* `targetDid`: String
* `hash`: String
* `raw`: String
* `enc`: String
* __->__ `request`: Json

#### build\_get\_attrib\_request(submitterDid, targetDid, hash, raw, enc) -> request
* `submitterDid`: String
* `targetDid`: String
* `hash`: String
* `raw`: String
* `enc`: String
* __->__ `request`: Json

#### build\_get\_nym\_request(submitterDid, targetDid) -> request
* `submitterDid`: String
* `targetDid`: String
* __->__ `request`: Json

#### build\_schema\_request(submitterDid, data) -> request
* `submitterDid`: String
* `data`: String
* __->__ `request`: Json

#### build\_get\_schema\_request(submitterDid, dest, data) -> request
* `submitterDid`: String
* `dest`: String
* `data`: String
* __->__ `request`: Json

#### build\_claim\_def\_txn(submitterDid, xref, signatureType, data) -> request
* `submitterDid`: String
* `xref`: Number
* `signatureType`: String
* `data`: String
* __->__ `request`: Json

#### build\_get\_claim\_def\_txn(submitterDid, xref, signatureType, origin) -> request
* `submitterDid`: String
* `xref`: Number
* `signatureType`: String
* `origin`: String
* __->__ `request`: Json

#### build\_node\_request(submitterDid, targetDid, data) -> request
* `submitterDid`: String
* `targetDid`: String
* `data`: String
* __->__ `request`: Json

#### build\_get\_txn\_request(submitterDid, data) -> request
* `submitterDid`: String
* `data`: Number
* __->__ `request`: Json

#### build\_pool\_config\_request(submitterDid, writes, force) -> request
* `submitterDid`: String
* `writes`: Boolean
* `force`: Boolean
* __->__ `request`: Json

#### build\_pool\_upgrade\_request(submitterDid, name, version, action, sha256, timeout, schedule, justification, reinstall, force) -> request
* `submitterDid`: String
* `name`: String
* `version`: String
* `action`: String
* `sha256`: String
* `timeout`: Number
* `schedule`: String
* `justification`: String
* `reinstall`: Boolean
* `force`: Boolean
* __->__ `request`: Json

#### is\_pairwise\_exists(walletHandle, theirDid) -> exists
* `walletHandle`: Number
* `theirDid`: String
* __->__ `exists`: Boolean

#### create\_pairwise(walletHandle, theirDid, myDid, metadata) -> void
* `walletHandle`: Number
* `theirDid`: String
* `myDid`: String
* `metadata`: String

#### list\_pairwise(walletHandle) -> listPairwise
* `walletHandle`: Number
* __->__ `listPairwise`: String

#### get\_pairwise(walletHandle, theirDid) -> pairwiseInfo
* `walletHandle`: Number
* `theirDid`: String
* __->__ `pairwiseInfo`: Json

#### set\_pairwise\_metadata(walletHandle, theirDid, metadata) -> void
* `walletHandle`: Number
* `theirDid`: String
* `metadata`: String

#### create\_pool\_ledger\_config(configName, config) -> void
* `configName`: String
* `config`: Json

#### open\_pool\_ledger(configName, config) -> poolHandle
* `configName`: String
* `config`: String
* __->__ `poolHandle`: Number

#### refresh\_pool\_ledger(handle) -> void
* `handle`: Number

#### list\_pools() -> pools
* __->__ `pools`: Json

#### close\_pool\_ledger(handle) -> void
* `handle`: Number

#### delete\_pool\_ledger\_config(configName) -> void
* `configName`: String

#### create\_wallet(poolName, name, xtype, config, credentials) -> void
* `poolName`: String
* `name`: String
* `xtype`: String
* `config`: String
* `credentials`: String

#### open\_wallet(name, runtimeConfig, credentials) -> handle
* `name`: String
* `runtimeConfig`: String
* `credentials`: String
* __->__ `handle`: Number

#### list\_wallets() -> wallets
* __->__ `wallets`: Json

#### close\_wallet(handle) -> void
* `handle`: Number

#### delete\_wallet(name, credentials) -> void
* `name`: String
* `credentials`: String


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

Much of the cpp code and README documentation is generated by scripts in the [codegen](https://github.com/Picolab/indy-sdk/tree/master/wrappers/nodejs/codegen) folder.
