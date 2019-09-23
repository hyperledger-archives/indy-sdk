<!-- markdownlint-disable MD033 -->

# Libindy 1.10 to 1.11 migration Guide

This document is written for developers using Libindy to provide necessary information and
to simplify their transition to Libindy 1.11 from Libindy 1.10. If you are using older Libindy
version you can check migration guides history:

* [Libindy 1.3 to 1.4 migration](https://github.com/hyperledger/indy-sdk/blob/v1.4.0/doc/migration-guide.md)
* [Libindy 1.4 to 1.5 migration](https://github.com/hyperledger/indy-sdk/blob/v1.5.0/doc/migration-guide-1.4.0-1.5.0.md)
* [Libindy 1.5 to 1.6 migration](https://github.com/hyperledger/indy-sdk/blob/v1.6.0/doc/migration-guide-1.5.0-1.6.0.md)
* [Libindy 1.6 to 1.7 migration](https://github.com/hyperledger/indy-sdk/blob/v1.7.0/doc/migration-guide-1.6.0-1.7.0.md)
* [Libindy 1.7 to 1.8 migration](https://github.com/hyperledger/indy-sdk/blob/v1.8.0/doc/migration-guide-1.7.0-1.8.0.md)
* [Libindy 1.8 to 1.9 migration](https://github.com/hyperledger/indy-sdk/blob/v1.9.0/docs/migration-guides/migration-guide-1.8.0-1.9.0.md)
* [Libindy 1.9 to 1.10 migration](https://github.com/hyperledger/indy-sdk/blob/v1.10.0/docs/migration-guides/migration-guide-1.9.0-1.10.0.md)
* [Libindy 1.10 to 1.11 migration](https://github.com/hyperledger/indy-sdk/blob/v1.11.0/docs/migration-guides/migration-guide-1.10.0-1.11.0.md)

## Table of contents

* [Notes](#notes)
* [Libindy 1.11 to 1.12 migration](#libindy-111-to-112-migration)
    * [DID API](#did-api)
    * [Anoncreds API](#anoncreds-api)
    
## Notes

Migration information is organized in tables, there are mappings for each Libindy API part of how older version functionality maps to a newer one.
Functions from older version are listed in the left column, and the equivalent newer version function is placed in the right column:

* If some function had been added, the word 'NEW' would be placed in the left column.
* If some function had been deleted, the word 'DELETED' would be placed in the right column.
* If some function had been deprecated, the word 'DEPRECATED' would be placed in the right column.
* If some function had been changed, the current format would be placed in the right column.
* If some function had not been changed, the symbol '=' would be placed in the right column.
* To get more details about current format of a function click on the description above it.
* Bellow are signatures of functions in Libindy C API.
  The params of ```cb``` (except command_handle and err) will be result values of the similar function in any Libindy wrapper.

## Libindy 1.11 to 1.12 migration

### DID API

In this release we have introduced fully-qualified DIDs. All functions right now accept both fully-qualified DIDs and unqualified DIDs.

<table>
    <tr>  
      <th>v1.11.0 - DID API</th>
      <th>v1.12.0 - DID API</th>
    </tr>
    <tr>
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.12.0/libindy/src/api/did.rs#L729">
              Qualifies did for some namespace
          </a>
      </th>
    <tr>
    <tr>
      <td>
          <b>NEW</b>
      </td>
      <td>
      <pre>
pub extern fn indy_qualify_did(command_handle: CommandHandle,
                               wallet_handle: WalletHandle,
                               did: *const c_char,
                               prefix: *const c_char,
                               cb: Option<extern fn(command_handle_: CommandHandle,
                                                                 err: ErrorCode,
                                                                 full_qualified_did: *const c_char)>)
      </pre>
      </td>
    </tr>
</table>

Also, contents of `config` param for `indy_create_and_store_my_did` has been extended -- you can specify the needed method by `method_name` param, otherwise the default value will be used. 
    
### Anoncreds API

As we have released Fully-Qualified DIDs, we can make proof request with restrictions in fully-qualified form and in unqualified form. You should correspond the docs for [`indy_prover_create_proof`](https://github.com/hyperledger/indy-sdk/blob/v1.12.0/libindy/src/api/anoncreds.rs) on how to use the version to make a proof request.

### Ledger API

Although we have released Fully-Qualified DIDs, all ledger-related functions will return unqualified DIDs. However you can create ledger requests with both Fully-Qualified DIDs and the old ones.  