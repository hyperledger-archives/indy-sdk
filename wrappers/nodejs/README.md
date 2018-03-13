# Indy SDK for Node.js

[![stability - experimental](https://img.shields.io/badge/stability-experimental-orange.svg)](https://nodejs.org/api/documentation.html#documentation_stability_index)

Native bindings for [Hyperledger Indy](https://www.hyperledger.org/projects/hyperledger-indy).

## Installing

This module has a native compile step. It compiles C++ code and dynamically links to `libindy` on your system.

You will need:

* C++ build tools and Python 2. See [this](https://github.com/nodejs/node-gyp#installation) for platform recommendations.
* `libindy` on your system in a shared library path. (i.e. `/usr/lib/libindy.so` for linux)

Then you can install via npm:

```sh
npm install --save indy-sdk
```

This is still experimental, please submit issues to: https://github.com/Picolab/indy-sdk/issues

## API

```js
var indy = require('indy-sdk')

var did = '...'
var full_verkey = '...'

indy.abbreviate_verkey(did, full_verkey, function(err, verkey){
  ..
})

// if you do not provide a callback, a Promise is returned

var verkey = await indy.abbreviate_verkey(did, full_verkey)
```

### IndyError

These errors are based on libindy error codes defined [here](https://github.com/hyperledger/indy-sdk/blob/master/libindy/include/indy_mod.h).

* `err.indy_code` - the code number from libindy
* `err.indy_name` - the name string for the code

[//]: # (CODEGEN-START - don't edit by hand see `codegen/index.js`)
#### issuer\_create\_and\_store\_claim\_def(wallet\_handle, issuer\_did, schema\_json, signature\_type, create\_non\_revoc, cb(err, claim\_def\_json))
* `wallet_handle`: Number
* `issuer_did`: String
* `schema_json`: String
* `signature_type`: String
* `create_non_revoc`: Boolean
* __->__ `claim_def_json`: String

#### issuer\_create\_and\_store\_revoc\_reg(wallet\_handle, issuer\_did, schema\_json, max\_claim\_num, cb(err, revoc\_reg\_json))
* `wallet_handle`: Number
* `issuer_did`: String
* `schema_json`: String
* `max_claim_num`: Number
* __->__ `revoc_reg_json`: String

#### issuer\_create\_claim\_offer(wallet\_handle, schema\_json, issuer\_did, prover\_did, cb(err, claim\_offer\_json))
* `wallet_handle`: Number
* `schema_json`: String
* `issuer_did`: String
* `prover_did`: String
* __->__ `claim_offer_json`: String

#### issuer\_create\_claim(wallet\_handle, claim\_req\_json, claim\_json, user\_revoc\_index, cb(err, [revoc\_reg\_update\_json, xclaim\_json]))
* `wallet_handle`: Number
* `claim_req_json`: String
* `claim_json`: String
* `user_revoc_index`: Number
* __->__ [`revoc_reg_update_json`: String, `xclaim_json`: String]

#### issuer\_revoke\_claim(wallet\_handle, issuer\_did, schema\_json, user\_revoc\_index, cb(err, revoc\_reg\_update\_json))
* `wallet_handle`: Number
* `issuer_did`: String
* `schema_json`: String
* `user_revoc_index`: Number
* __->__ `revoc_reg_update_json`: String

#### prover\_store\_claim\_offer(wallet\_handle, claim\_offer\_json, cb(err))
* `wallet_handle`: Number
* `claim_offer_json`: String

#### prover\_get\_claim\_offers(wallet\_handle, filter\_json, cb(err, claim\_offers\_json))
* `wallet_handle`: Number
* `filter_json`: String
* __->__ `claim_offers_json`: String

#### prover\_create\_master\_secret(wallet\_handle, master\_secret\_name, cb(err))
* `wallet_handle`: Number
* `master_secret_name`: String

#### prover\_create\_and\_store\_claim\_req(wallet\_handle, prover\_did, claim\_offer\_json, claim\_def\_json, master\_secret\_name, cb(err, claim\_req\_json))
* `wallet_handle`: Number
* `prover_did`: String
* `claim_offer_json`: String
* `claim_def_json`: String
* `master_secret_name`: String
* __->__ `claim_req_json`: String

#### prover\_store\_claim(wallet\_handle, claims\_json, rev\_reg\_json, cb(err))
* `wallet_handle`: Number
* `claims_json`: String
* `rev_reg_json`: String

#### prover\_get\_claims(wallet\_handle, filter\_json, cb(err, claims\_json))
* `wallet_handle`: Number
* `filter_json`: String
* __->__ `claims_json`: String

#### prover\_get\_claims\_for\_proof\_req(wallet\_handle, proof\_request\_json, cb(err, claims\_json))
* `wallet_handle`: Number
* `proof_request_json`: String
* __->__ `claims_json`: String

#### prover\_create\_proof(wallet\_handle, proof\_req\_json, requested\_claims\_json, schemas\_json, master\_secret\_name, claim\_defs\_json, revoc\_regs\_json, cb(err, proof\_json))
* `wallet_handle`: Number
* `proof_req_json`: String
* `requested_claims_json`: String
* `schemas_json`: String
* `master_secret_name`: String
* `claim_defs_json`: String
* `revoc_regs_json`: String
* __->__ `proof_json`: String

#### verifier\_verify\_proof(proof\_request\_json, proof\_json, schemas\_json, claim\_defs\_jsons, revoc\_regs\_json, cb(err, valid))
* `proof_request_json`: String
* `proof_json`: String
* `schemas_json`: String
* `claim_defs_jsons`: String
* `revoc_regs_json`: String
* __->__ `valid`: Boolean

#### create\_key(wallet\_handle, key\_json, cb(err, vk))
* `wallet_handle`: Number
* `key_json`: String
* __->__ `vk`: String

#### set\_key\_metadata(wallet\_handle, verkey, metadata, cb(err))
* `wallet_handle`: Number
* `verkey`: String
* `metadata`: String

#### get\_key\_metadata(wallet\_handle, verkey, cb(err, metadata))
* `wallet_handle`: Number
* `verkey`: String
* __->__ `metadata`: String

#### crypto\_sign(wallet\_handle, my\_vk, message\_raw, cb(err, signature\_raw))
* `wallet_handle`: Number
* `my_vk`: String
* `message_raw`: Buffer
* __->__ `signature_raw`: Buffer

#### crypto\_verify(their\_vk, message\_raw, signature\_raw, cb(err, valid))
* `their_vk`: String
* `message_raw`: Buffer
* `signature_raw`: Buffer
* __->__ `valid`: Boolean

#### crypto\_auth\_crypt(wallet\_handle, my\_vk, their\_vk, message\_raw, cb(err, encrypted\_msg\_raw))
* `wallet_handle`: Number
* `my_vk`: String
* `their_vk`: String
* `message_raw`: Buffer
* __->__ `encrypted_msg_raw`: Buffer

#### crypto\_auth\_decrypt(wallet\_handle, my\_vk, encrypted\_msg\_raw, cb(err, [their\_vk, decrypted\_msg\_raw]))
* `wallet_handle`: Number
* `my_vk`: String
* `encrypted_msg_raw`: Buffer
* __->__ [`their_vk`: String, `decrypted_msg_raw`: Buffer]

#### crypto\_anon\_crypt(their\_vk, message\_raw, cb(err, encrypted\_msg\_raw))
* `their_vk`: String
* `message_raw`: Buffer
* __->__ `encrypted_msg_raw`: Buffer

#### crypto\_anon\_decrypt(wallet\_handle, my\_vk, encrypted\_msg, cb(err, decrypted\_msg\_raw))
* `wallet_handle`: Number
* `my_vk`: String
* `encrypted_msg`: Buffer
* __->__ `decrypted_msg_raw`: Buffer

#### create\_and\_store\_my\_did(wallet\_handle, did\_json, cb(err, [did, verkey]))
* `wallet_handle`: Number
* `did_json`: String
* __->__ [`did`: String, `verkey`: String]

#### replace\_keys\_start(wallet\_handle, did, identity\_json, cb(err, verkey))
* `wallet_handle`: Number
* `did`: String
* `identity_json`: String
* __->__ `verkey`: String

#### replace\_keys\_apply(wallet\_handle, did, cb(err))
* `wallet_handle`: Number
* `did`: String

#### store\_their\_did(wallet\_handle, identity\_json, cb(err))
* `wallet_handle`: Number
* `identity_json`: String

#### key\_for\_did(pool\_handle, wallet\_handle, did, cb(err, key))
* `pool_handle`: Number
* `wallet_handle`: Number
* `did`: String
* __->__ `key`: String

#### key\_for\_local\_did(wallet\_handle, did, cb(err, key))
* `wallet_handle`: Number
* `did`: String
* __->__ `key`: String

#### set\_endpoint\_for\_did(wallet\_handle, did, address, transport\_key, cb(err))
* `wallet_handle`: Number
* `did`: String
* `address`: String
* `transport_key`: String

#### get\_endpoint\_for\_did(wallet\_handle, pool\_handle, did, cb(err, [address, transport\_vk]))
* `wallet_handle`: Number
* `pool_handle`: Number
* `did`: String
* __->__ [`address`: String, `transport_vk`: String]

#### set\_did\_metadata(wallet\_handle, did, metadata, cb(err))
* `wallet_handle`: Number
* `did`: String
* `metadata`: String

#### get\_did\_metadata(wallet\_handle, did, cb(err, metadata))
* `wallet_handle`: Number
* `did`: String
* __->__ `metadata`: String

#### get\_my\_did\_with\_meta(wallet\_handle, my\_did, cb(err, did\_with\_meta))
* `wallet_handle`: Number
* `my_did`: String
* __->__ `did_with_meta`: String

#### list\_my\_dids\_with\_meta(wallet\_handle, cb(err, dids))
* `wallet_handle`: Number
* __->__ `dids`: String

#### abbreviate\_verkey(did, full\_verkey, cb(err, verkey))
* `did`: String
* `full_verkey`: String
* __->__ `verkey`: String

#### sign\_and\_submit\_request(pool\_handle, wallet\_handle, submitter\_did, request\_json, cb(err, request\_result\_json))
* `pool_handle`: Number
* `wallet_handle`: Number
* `submitter_did`: String
* `request_json`: String
* __->__ `request_result_json`: String

#### submit\_request(pool\_handle, request\_json, cb(err, request\_result\_json))
* `pool_handle`: Number
* `request_json`: String
* __->__ `request_result_json`: String

#### sign\_request(wallet\_handle, submitter\_did, request\_json, cb(err, signed\_request\_json))
* `wallet_handle`: Number
* `submitter_did`: String
* `request_json`: String
* __->__ `signed_request_json`: String

#### build\_get\_ddo\_request(submitter\_did, target\_did, cb(err, request\_result\_json))
* `submitter_did`: String
* `target_did`: String
* __->__ `request_result_json`: String

#### build\_nym\_request(submitter\_did, target\_did, verkey, alias, role, cb(err, request\_json))
* `submitter_did`: String
* `target_did`: String
* `verkey`: String
* `alias`: String
* `role`: String
* __->__ `request_json`: String

#### build\_attrib\_request(submitter\_did, target\_did, hash, raw, enc, cb(err, request\_json))
* `submitter_did`: String
* `target_did`: String
* `hash`: String
* `raw`: String
* `enc`: String
* __->__ `request_json`: String

#### build\_get\_attrib\_request(submitter\_did, target\_did, hash, raw, enc, cb(err, request\_json))
* `submitter_did`: String
* `target_did`: String
* `hash`: String
* `raw`: String
* `enc`: String
* __->__ `request_json`: String

#### build\_get\_nym\_request(submitter\_did, target\_did, cb(err, request\_json))
* `submitter_did`: String
* `target_did`: String
* __->__ `request_json`: String

#### build\_schema\_request(submitter\_did, data, cb(err, request\_json))
* `submitter_did`: String
* `data`: String
* __->__ `request_json`: String

#### build\_get\_schema\_request(submitter\_did, dest, data, cb(err, request\_json))
* `submitter_did`: String
* `dest`: String
* `data`: String
* __->__ `request_json`: String

#### build\_claim\_def\_txn(submitter\_did, xref, signature\_type, data, cb(err, request\_json))
* `submitter_did`: String
* `xref`: Number
* `signature_type`: String
* `data`: String
* __->__ `request_json`: String

#### build\_get\_claim\_def\_txn(submitter\_did, xref, signature\_type, origin, cb(err, request\_json))
* `submitter_did`: String
* `xref`: Number
* `signature_type`: String
* `origin`: String
* __->__ `request_json`: String

#### build\_node\_request(submitter\_did, target\_did, data, cb(err, request\_json))
* `submitter_did`: String
* `target_did`: String
* `data`: String
* __->__ `request_json`: String

#### build\_get\_txn\_request(submitter\_did, data, cb(err, request\_json))
* `submitter_did`: String
* `data`: Number
* __->__ `request_json`: String

#### build\_pool\_config\_request(submitter\_did, writes, force, cb(err, request\_json))
* `submitter_did`: String
* `writes`: Boolean
* `force`: Boolean
* __->__ `request_json`: String

#### build\_pool\_upgrade\_request(submitter\_did, name, version, action, sha256, timeout, schedule, justification, reinstall, force, cb(err, request\_json))
* `submitter_did`: String
* `name`: String
* `version`: String
* `action`: String
* `sha256`: String
* `timeout`: Number
* `schedule`: String
* `justification`: String
* `reinstall`: Boolean
* `force`: Boolean
* __->__ `request_json`: String

#### is\_pairwise\_exists(wallet\_handle, their\_did, cb(err, exists))
* `wallet_handle`: Number
* `their_did`: String
* __->__ `exists`: Boolean

#### create\_pairwise(wallet\_handle, their\_did, my\_did, metadata, cb(err))
* `wallet_handle`: Number
* `their_did`: String
* `my_did`: String
* `metadata`: String

#### list\_pairwise(wallet\_handle, cb(err, list\_pairwise))
* `wallet_handle`: Number
* __->__ `list_pairwise`: String

#### get\_pairwise(wallet\_handle, their\_did, cb(err, pairwise\_info\_json))
* `wallet_handle`: Number
* `their_did`: String
* __->__ `pairwise_info_json`: String

#### set\_pairwise\_metadata(wallet\_handle, their\_did, metadata, cb(err))
* `wallet_handle`: Number
* `their_did`: String
* `metadata`: String

#### create\_pool\_ledger\_config(config\_name, config, cb(err))
* `config_name`: String
* `config`: String

#### open\_pool\_ledger(config\_name, config, cb(err, pool\_handle))
* `config_name`: String
* `config`: String
* __->__ `pool_handle`: Number

#### refresh\_pool\_ledger(handle, cb(err))
* `handle`: Number

#### list\_pools(cb(err, pools))
* __->__ `pools`: String

#### close\_pool\_ledger(handle, cb(err))
* `handle`: Number

#### delete\_pool\_ledger\_config(config\_name, cb(err))
* `config_name`: String

#### create\_wallet(pool\_name, name, xtype, config, credentials, cb(err))
* `pool_name`: String
* `name`: String
* `xtype`: String
* `config`: String
* `credentials`: String

#### open\_wallet(name, runtime\_config, credentials, cb(err, handle))
* `name`: String
* `runtime_config`: String
* `credentials`: String
* __->__ `handle`: Number

#### list\_wallets(cb(err, wallets))
* __->__ `wallets`: String

#### close\_wallet(handle, cb(err))
* `handle`: Number

#### delete\_wallet(name, credentials, cb(err))
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
# needed for the build step
cp -r ../../libindy/include/ .

# this will install dependencies and do the initial build
npm i

# run the tests!
RUST_LOG=trace TEST_POOL_IP=10.0.0.2 npm test

# now edit and rerun tests
# if you need to recompile the native bindings
npm run rebuild -s
```

Much of the cpp code and README documentation is generated by scripts in the [codegen/](https://github.com/Picolab/indy-sdk/tree/master/wrappers/nodejs/codegen) folder. They are based on `libindy`'s  header files. See [src/api.json](https://github.com/Picolab/indy-sdk/blob/master/wrappers/nodejs/src/api.json)
