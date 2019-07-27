<!-- markdownlint-disable MD033 -->

# Libindy 1.9 to 1.10 migration Guide

This document is written for developers using Libindy to provide necessary information and
to simplify their transition to Libindy 1.10 from Libindy 1.9. If you are using older Libindy
version you can check migration guides history:

* [Libindy 1.3 to 1.4 migration](https://github.com/hyperledger/indy-sdk/blob/v1.4.0/doc/migration-guide.md)
* [Libindy 1.4 to 1.5 migration](https://github.com/hyperledger/indy-sdk/blob/v1.5.0/doc/migration-guide-1.4.0-1.5.0.md)
* [Libindy 1.5 to 1.6 migration](https://github.com/hyperledger/indy-sdk/blob/v1.6.0/doc/migration-guide-1.5.0-1.6.0.md)
* [Libindy 1.6 to 1.7 migration](https://github.com/hyperledger/indy-sdk/blob/v1.7.0/doc/migration-guide-1.6.0-1.7.0.md)
* [Libindy 1.7 to 1.8 migration](https://github.com/hyperledger/indy-sdk/blob/v1.8.0/doc/migration-guide-1.7.0-1.8.0.md)
* [Libindy 1.8 to 1.9 migration](https://github.com/hyperledger/indy-sdk/blob/v1.9.0/doc/migration-guide-1.8.0-1.9.0.md)

## Table of contents

* [Notes](#notes)
* [Libindy 1.9 to 1.10 migration](#libindy-19-to-110-migration-guide)
    * [Ledger API](#libindy-api)
    * [Anoncreds API](#anoncreds-api)
* [Libindy 1.10.0 to 1.10.1 migration](#libindy-1100-to-1101-migration-guide)

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

## Libindy 1.9 to 1.10 migration Guide

### Ledger API

#### Changes

<table>
    <tr>  
      <th>v1.9.0 - Ledger API</th>
      <th>v1.10.0 - Ledger API</th>
    </tr>
    <tr>
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.10.0/libindy/src/api/ledger.rs#L1929">
              Builds a AUTH_RULES request. Request to change multiple authentication rules for a ledger transaction.
          </a>
      </th>
    <tr>
    <tr>
      <td>
          <b>NEW</b>
      </td>
      <td>
      <pre>
indy_build_auth_rules_request(command_handle: CommandHandle,
                              submitter_did: *const c_char,
                              rules: *const c_char,
                              cb: fn(command_handle_: CommandHandle,
                                     err: ErrorCode,
                                     request_json: *const c_char))
      </pre>
      </td>
    </tr>
</table>

### Anoncreds API

The main idea of changes performed in Anoncreds API is to provide a way to configure what tags to build 
on credential storage in prover wallet, tailoring data profile in storage to application search needs.
So, Two new *Experimental* functions were added to Libindy API to achieve this goal.

<table>
    <tr>  
      <th>v1.9.0 - Cache API</th>
      <th>v1.10.0 - Cache API</th>
    </tr>
    <tr>
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.10.0/libindy/src/api/anoncreds.rs#L735">
              Set credential attribute tagging policy. <br>
              Writes a non-secret record marking attributes to tag, and optionally <br>
              updates tags on existing credentials on the credential definition to match.
          </a>
      </th>
    <tr>
    <tr>
      <td>
          <b>NEW</b>
      </td>
      <td>
          <pre>
indy_prover_set_credential_attr_tag_policy(command_handle: i32,
                                           wallet_handle: WalletHandle,
                                           cred_def_id: *const c_char,
                                           tag_attrs_json: *const c_char,
                                           retroactive: bool,
                                           cb: Option<extern fn(xcommand_handle: i32,
                                                                err: ErrorCode)>)
          </pre>
      </td>
    </tr>
    <tr>
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.10.0/libindy/src/api/anoncreds.rs#L809">
              Get credential attribute tagging policy by credential definition id
          </a>
      </th>
    <tr>
    <tr>
      <td>
          <b>NEW</b>
      </td>
      <td>
          <pre>
indy_prover_get_credential_attr_tag_policy(command_handle: i32,
                                           wallet_handle: WalletHandle,
                                           cred_def_id: *const c_char,
                                           cb: Option<extern fn(xcommand_handle: i32,
                                                                err: ErrorCode,
                                                                catpol_json: *const c_char)>)
          </pre>
      </td>
    </tr>
</table>

## Libindy 1.10.0 to 1.10.1 migration Guide

The Libindy 1.10.1 release contains fixes that don't affect API functions. 