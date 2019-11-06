<!-- markdownlint-disable MD033 -->

# Libindy 1.4 to 1.5 migration Guide

This document is written for developers using Libindy to provide necessary information and
to simplify their transition to Libindy 1.5 from Libindy 1.4. If you are using older Libindy
version you can check migration guides history:

* [Libindy 1.3 to 1.4 migration](https://github.com/hyperledger/indy-sdk/blob/v1.4.0/doc/migration-guide.md)

## Table of contents

* [Notes](#notes)
* [Wallet API](#wallet-api)
* [Non-Secrets API](#non-secrets-api)
* [Payments API](#payments-api)
* [Pool API](#pool-api)
* [Ledger API](#ledger-api)

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

### Wallet API

* In v1.4 libindy allowed to plug different wallet implementations. Plugged wallet in v1.4 handled both security
  and storage layers. Libindy v1.5 restricts plugged interface by handling only storage layer.
  All encryption is performed in libindy. It simplifies plugged wallets and provides warranty of a good security level
  for 3d party wallets implementations.
* Libindy v1.5 changes wallet format to allow efficient and flexible search for entities with pagination support.
  *WARNING* wallet format of libindy v1.5 isn't compatible with a wallet format of libindy v1.4.
* There have been added functions that allow performing Export/Import of Wallet. Note these endpoints are EXPERIMENTAL.
  Function signature and behavior may change in the future releases.
* ```indy_list_wallets``` endpoint is DEPRECATED and will be removed in the next release. The main idea is avoid
  maintaining created wallet list on libindy side. It will allow to access wallets from a cluster and solve
  some problems on mobile platforms. ```indy_create_wallet``` and ```indy_open_wallet``` endpoints will
  also get related changes in the next release.

References:

* [Wallet Storage Design](https://github.com/hyperledger/indy-sdk/tree/master/docs/design/003-wallet-storage)
* [Wallet Export/Import Design](https://github.com/hyperledger/indy-sdk/tree/master/docs/design/009-wallet-export-import)

<table>
  <tr>  
    <th>v1.4.0 - Wallet API</th>
    <th>v1.5.0 - Wallet API</th>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/wallet.rs#L11">
            Register custom wallet storage implementation
        </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_register_wallet_type(
        command_handle: i32,
        xtype: *const c_char,
        create: WalletCreate,
        open: WalletOpen,
        set: WalletSet,
        get: WalletGet,
        get_not_expired: WalletGetNotExpired,
        list: WalletList,
        close: WalletClose,
        delete: WalletDelete,
        free: WalletFree,
        cb: fn(xcommand_handle: i32,
             err: ErrorCode))
      </pre>
    </td>
    <td>
      <pre>
indy_register_wallet_storage(
        command_handle: i32,
        type_: *const c_char,
        create: WalletCreate,
        open: WalletOpen,
        close: WalletClose,
        delete: WalletDelete,
        add_record: WalletAddRecord,
        update_record_value: WalletUpdateRecordValue,
        update_record_tags: WalletUpdateRecordTags,
        add_record_tags: WalletAddRecordTags,
        delete_record_tags: WalletDeleteRecordTags,
        delete_record: WalletDeleteRecord,
        get_record: WalletGetRecord,
        get_record_id: WalletGetRecordId,
        get_record_type: WalletGetRecordType,
        get_record_value: WalletGetRecordValue,
        get_record_tags: WalletGetRecordTags,
        free_record: WalletFreeRecord,
        get_storage_metadata: WalletGetStorageMetadata,
        set_storage_metadata: WalletSetStorageMetadata,
        free_storage_metadata: WalletFreeStorageMetadata,
        search_records: WalletSearchRecords,
        search_all_records: WalletSearchAllRecords,
        get_search_total_count: WalletGetSearchTotalCount,
        fetch_search_next_record: WalletFetchSearchNextRecord,
        free_search: WalletFreeSearch,
        cb: fn(xcommand_handle: i32,
               err: ErrorCode))
      </pre>
    </td>
  </tr>
    <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/wallet.rs#L142">
            Create a new secure wallet
        </a>
    </th>
  <tr>
    <td>
      <pre>
indy_create_wallet(command_handle: i32,
                   pool_name: *const c_char,
                   name: *const c_char,
                   xtype: *const c_char,
                   config: *const c_char,
                   credentials: *const c_char,
                   cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode)>)
      </pre>
    </td>
    <td>
      <pre>
indy_create_wallet(command_handle: i32,
                   pool_name: *const c_char,
                   name: *const c_char,
                   storage_type: *const c_char,
                   config: *const c_char,
                   credentials_json: *const c_char,
                   cb: Option<extern fn(xcommand_handle: i32,
                                        err: ErrorCode)>)
      </pre>
      <b>Note:</b> Signatures look similar, but <i>credentials_json</i> parameter became the required.
      Format of <i>config</i> and <i>credentials_json</i> parameters was changed.
    </td>
  </tr>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/wallet.rs#L210">
            Open the wallet with specific name
        </a>
    </th>
  <tr>
    <td>
      <pre>
indy_open_wallet(command_handle: i32,
                 name: *const c_char,
                 runtime_config: *const c_char,
                 credentials: *const c_char,
                 cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, handle: i32)>)
      </pre>
    </td>
    <td>
      <pre>
indy_open_wallet(command_handle: i32,
                 name: *const c_char,
                 runtime_config: *const c_char,
                 credentials_json: *const c_char,
                 cb: Option<extern fn(xcommand_handle: i32,
                                      err: ErrorCode,
                                      handle: i32)>)
      </pre>
      <b>Note:</b> Signatures look similar, but <i>credentials_json</i> parameter became the required.
      Format of <i>credentials_json</i> parameter was changed.
    </td>
  </tr>
    <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/wallet.rs#L475">
            Deletes created wallet
        </a>
    </th>
  <tr>
    <td>
      <pre>
indy_delete_wallet(command_handle: i32,
                   name: *const c_char,
                   credentials: *const c_char,
                   cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode)>)
      </pre>
    </td>
    <td>
      <pre>
indy_delete_wallet(command_handle: i32,
                   name: *const c_char,
                   credentials_json: *const c_char,
                   cb: Option<extern fn(xcommand_handle: i32,
                                        err: ErrorCode)>)
      </pre>
      <b>Note:</b> Signatures look similar, but <i>credentials_json</i> parameter became the required.
      Format of <i>credentials_json</i> parameter was changed.
    </td>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/wallet.rs#L311">
            Export wallet
        </a>
    </th>
  </tr>
  <tr>
    <td><b>NEW</b></td>
    <td>
      <pre>
indy_export_wallet(command_handle: i32,
                   wallet_handle: i32,
                   export_config_json: *const c_char,
                   cb: fn(xcommand_handle: i32, err: ErrorCode))
        </pre>
    </td>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/wallet.rs#L359">
            Import wallet
        </a>
    </th>
  </tr>
  <tr>
    <td><b>NEW</b></td>
    <td>
      <pre>
indy_import_wallet(command_handle: i32,
                   pool_name: *const c_char,
                   name: *const c_char,
                   storage_type: *const c_char,
                   config: *const c_char,
                   credentials: *const c_char,
                   import_config_json: *const c_char,
                   cb: fn(xcommand_handle: i32, err: ErrorCode))
        </pre>
    </td>
  </tr>
</table>

### Non-Secrets API

Libindy v1.5 introduces set of API endpoints are intended to store and read application specific
identity data in the wallet. This API doesn't have an access to secrets stored in the wallet.

References:

* [Wallet Storage Design](https://github.com/hyperledger/indy-sdk/tree/master/docs/design/003-wallet-storage)

<table>  
  <tr>
    <th>v1.4.0 - Non-Secrets API</th>
    <th>v1.5.0 - Non-Secrets API</th>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/non_secrets.rs#L12">
            Create a new record in the wallet
        </a>
    </th>
  </tr>
  <tr>
    <td><b>NEW</b></td>
    <td>
      <pre>
indy_add_wallet_record(command_handle: i32,
                       wallet_handle: i32,
                       type_: *const c_char,
                       id: *const c_char,
                       value: *const c_char,
                       tags_json: *const c_char,
                       cb: fn(command_handle_: i32,
                              err: ErrorCode))
        </pre>
    </td>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/non_secrets.rs#L71">
            Update a wallet record value
        </a>
    </th>
  </tr>
  <tr>
    <td><b>NEW</b></td>
    <td>
      <pre>
indy_update_wallet_record_value(command_handle: i32,
                                wallet_handle: i32,
                                type_: *const c_char,
                                id: *const c_char,
                                value: *const c_char,
                                cb: fn(command_handle_: i32,
                                       err: ErrorCode))
        </pre>
    </td>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/non_secrets.rs#L116">
            Update a wallet record tags
        </a>
    </th>
  </tr>
  <tr>
    <td><b>NEW</b></td>
    <td>
      <pre>
indy_update_wallet_record_tags(command_handle: i32,
                               wallet_handle: i32,
                               type_: *const c_char,
                               id: *const c_char,
                               tags_json: *const c_char,
                               cb: fn(command_handle_: i32,
                                      err: ErrorCode))
        </pre>
    </td>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/non_secrets.rs#L170">
            Add new tags to the wallet record
        </a>
    </th>
  </tr>
  <tr>
    <td><b>NEW</b></td>
    <td>
      <pre>
indy_add_wallet_record_tags(command_handle: i32,
                            wallet_handle: i32,
                            type_: *const c_char,
                            id: *const c_char,
                            tags_json: *const c_char,
                            cb: fn(command_handle_: i32,
                                   err: ErrorCode))
        </pre>
    </td>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/non_secrets.rs#L226">
            Delete tags from the wallet record
        </a>
    </th>
  </tr>
  <tr>
    <td><b>NEW</b></td>
    <td>
      <pre>
indy_delete_wallet_record_tags(command_handle: i32,
                               wallet_handle: i32,
                               type_: *const c_char,
                               id: *const c_char,
                               tag_names_json: *const c_char,
                               cb: fn(command_handle_: i32,
                                      err: ErrorCode))
        </pre>
    </td>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/non_secrets.rs#L272">
            Delete an existing wallet record in the wallet
        </a>
    </th>
  </tr>
  <tr>
    <td><b>NEW</b></td>
    <td>
      <pre>
indy_delete_wallet_record(command_handle: i32,
                          wallet_handle: i32,
                          type_: *const c_char,
                          id: *const c_char,
                          cb: fn(command_handle_: i32,
                                 err: ErrorCode))
        </pre>
    </td>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/non_secrets.rs#L313">
            Get an wallet record by id
        </a>
    </th>
  </tr>
  <tr>
    <td><b>NEW</b></td>
    <td>
      <pre>
indy_get_wallet_record(command_handle: i32,
                       wallet_handle: i32,
                       type_: *const c_char,
                       id: *const c_char,
                       options_json: *const c_char,
                       cb: fn(command_handle_: i32,
                              err: ErrorCode,
                              record_json: *const c_char))
        </pre>
    </td>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/non_secrets.rs#L373">
            Search for wallet records
        </a>
    </th>
  </tr>
  <tr>
    <td><b>NEW</b></td>
    <td>
      <pre>
indy_open_wallet_search(command_handle: i32,
                        wallet_handle: i32,
                        type_: *const c_char,
                        query_json: *const c_char,
                        options_json: *const c_char,
                        cb: fn(command_handle_: i32,
                               err: ErrorCode,
                               search_handle: i32))
        </pre>
    </td>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/non_secrets.rs#L439">
            Fetch next records for wallet search.
        </a>
    </th>
  </tr>
  <tr>
    <td><b>NEW</b></td>
    <td>
      <pre>
indy_fetch_wallet_search_next_records(command_handle: i32,
                                      wallet_handle: i32,
                                      wallet_search_handle: i32,
                                      count: usize,
                                      cb: fn(command_handle_: i32,
                                             err: ErrorCode,
                                             records_json: *const c_char))
        </pre>
    </td>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/non_secrets.rs#L493">
            Close wallet search
        </a>
    </th>
  </tr>
  <tr>
    <td><b>NEW</b></td>
    <td>
      <pre>
indy_close_wallet_search(command_handle: i32,
                         wallet_search_handle: i32,
                         cb: fn(command_handle_: i32,
                                err: ErrorCode))
        </pre>
    </td>
  </tr>
</table>

### Payments API

This API is intended to provide the ability to register custom payment method and then to perform the main payments operations:

* Creation of payment address
* Listing of payment addresses
* Getting list of UTXO for payment address
* Sending payment transaction
* Adding fees to transactions
* Getting transactions fees amount

Note all endpoints in this group are EXPERIMENTAL. Function signatures and behavior may change
in the future releases.

References:

* [Payment Interface Design](https://github.com/hyperledger/indy-sdk/tree/master/docs/design/004-payment-interface)

<table>
  <tr>
    <th>v1.4.0 - Payments API</th>
    <th>v1.5.0 - Payments API</th>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/payments.rs#L264">
            Register custom payment implementation
        </a>
    </th>
  </tr>
  <tr>
    <td><b>NEW</b></td>
    <td>
      <pre>
indy_register_payment_method(command_handle: i32,
                             payment_method: *const c_char,
                             create_payment_address: CreatePaymentAddressCB,
                             add_request_fees: AddRequestFeesCB,
                             parse_response_with_fees: ParseResponseWithFeesCB,
                             build_get_utxo_request: BuildGetUTXORequestCB,
                             parse_get_utxo_response: ParseGetUTXOResponseCB,
                             build_payment_req: BuildPaymentReqCB,
                             parse_payment_response: ParsePaymentResponseCB,
                             build_mint_req: BuildMintReqCB,
                             build_set_txn_fees_req: BuildSetTxnFeesReqCB,
                             build_get_txn_fees_req: BuildGetTxnFeesReqCB,
                             parse_get_txn_fees_response: ParseGetTxnFeesResponseCB,
                             cb: fn(command_handle_: i32,
                                    err: ErrorCode))
        </pre>
    </td>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/payments.rs#L345">
            Create the payment address for specified payment method
        </a>
    </th>
  </tr>
  <tr>
    <td><b>NEW</b></td>
    <td>
      <pre>
indy_create_payment_address(command_handle: i32,
                            wallet_handle: i32,
                            payment_method: *const c_char,
                            config: *const c_char,
                            cb: fn(command_handle_: i32,
                                   err: ErrorCode,
                                   payment_address: *const c_char))
        </pre>
    </td>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/payments.rs#L404">
            Lists all payment addresses that are stored in the wallet
        </a>
    </th>
  </tr>
  <tr>
    <td><b>NEW</b></td>
    <td>
      <pre>
indy_list_payment_addresses(command_handle: i32,
                            wallet_handle: i32,
                            cb: fn(command_handle_: i32,
                                   err: ErrorCode,
                                   payment_addresses_json: *const c_char))
        </pre>
    </td>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/payments.rs#L444">
            Modifies Indy request by adding information how to pay fees for this transaction
            according to selected payment method
        </a>
    </th>
  </tr>
  <tr>
    <td><b>NEW</b></td>
    <td>
      <pre>
indy_add_request_fees(command_handle: i32,
                      wallet_handle: i32,
                      submitter_did: *const c_char,
                      req_json: *const c_char,
                      inputs_json: *const c_char,
                      outputs_json: *const c_char,
                      cb: fn(command_handle_: i32,
                             err: ErrorCode,
                             req_with_fees_json: *const c_char,
                             payment_method: *const c_char))
        </pre>
    </td>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/payments.rs#L521">
            Parses response for Indy request with fees
        </a>
    </th>
  </tr>
  <tr>
    <td><b>NEW</b></td>
    <td>
      <pre>
indy_parse_response_with_fees(command_handle: i32,
                              payment_method: *const c_char,
                              resp_json: *const c_char,
                              cb: fn(command_handle_: i32,
                                     err: ErrorCode,
                                     utxo_json: *const c_char))
        </pre>
    </td>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/payments.rs#L568">
            Builds Indy request for getting UTXO list for payment address
            according to this payment method
        </a>
    </th>
  </tr>
  <tr>
    <td><b>NEW</b></td>
    <td>
      <pre>
indy_build_get_utxo_request(command_handle: i32,
                            wallet_handle: i32,
                            submitter_did: *const c_char,
                            payment_address: *const c_char,
                            cb: fn(command_handle_: i32,
                                   err: ErrorCode,
                                   get_utxo_txn_json: *const c_char,
                                   payment_method: *const c_char))
        </pre>
    </td>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/payments.rs#L618">
            Parses response for Indy request for getting UTXO list
        </a>
    </th>
  </tr>
  <tr>
    <td><b>NEW</b></td>
    <td>
      <pre>
indy_parse_get_utxo_response(command_handle: i32,
                             payment_method: *const c_char,
                             resp_json: *const c_char,
                             cb: fn(command_handle_: i32,
                                    err: ErrorCode,
                                    utxo_json: *const c_char))
        </pre>
    </td>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/payments.rs#L666">
            Builds Indy request for doing tokens payment according to this payment method
        </a>
    </th>
  </tr>
  <tr>
    <td><b>NEW</b></td>
    <td>
      <pre>
indy_build_payment_req(command_handle: i32,
                       wallet_handle: i32,
                       submitter_did: *const c_char,
                       inputs_json: *const c_char,
                       outputs_json: *const c_char,
                       cb: fn(command_handle_: i32,
                              err: ErrorCode,
                              payment_req_json: *const c_char,
                              payment_method: *const c_char))
        </pre>
    </td>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/payments.rs#L723">
            Parses response for Indy request for payment txn
        </a>
    </th>
  </tr>
  <tr>
    <td><b>NEW</b></td>
    <td>
      <pre>
indy_parse_payment_response(command_handle: i32,
                            payment_method: *const c_char,
                            resp_json: *const c_char,
                            cb: fn(command_handle_: i32,
                                   err: ErrorCode,
                                   utxo_json: *const c_char))
        </pre>
    </td>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/payments.rs#L782">
            Builds Indy request for doing tokens minting according to this payment method
        </a>
    </th>
  </tr>
  <tr>
    <td><b>NEW</b></td>
    <td>
      <pre>
indy_build_mint_req(command_handle: i32,
                    wallet_handle: i32,
                    submitter_did: *const c_char,
                    outputs_json: *const c_char,
                    cb: fn(command_handle_: i32,
                           err: ErrorCode,
                           mint_req_json: *const c_char,
                           payment_method: *const c_char))
        </pre>
    </td>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/payments.rs#L837">
            Builds Indy request for setting fees for transactions in the ledger
        </a>
    </th>
  </tr>
  <tr>
    <td><b>NEW</b></td>
    <td>
      <pre>
indy_build_set_txn_fees_req(command_handle: i32,
                            wallet_handle: i32,
                            submitter_did: *const c_char,
                            payment_method: *const c_char,
                            fees_json: *const c_char,
                            cb: fn(command_handle_: i32,
                                   err: ErrorCode,
                                   set_txn_fees_json: *const c_char))
        </pre>
    </td>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/payments.rs#L892">
            Builds Indy get request for getting fees for transactions in the ledger
        </a>
    </th>
  </tr>
  <tr>
    <td><b>NEW</b></td>
    <td>
      <pre>
indy_build_get_txn_fees_req(command_handle: i32,
                            wallet_handle: i32,
                            submitter_did: *const c_char,
                            payment_method: *const c_char,
                            cb: fn(command_handle_: i32,
                                   err: ErrorCode,
                                   get_txn_fees_json: *const c_char))
        </pre>
    </td>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/payments.rs#L939">
            Parses response for Indy request for getting fees
        </a>
    </th>
  </tr>
  <tr>
    <td><b>NEW</b></td>
    <td>
      <pre>
indy_parse_get_txn_fees_response(command_handle: i32,
                                 payment_method: *const c_char,
                                 resp_json: *const c_char,
                                 cb: fn(command_handle_: i32,
                                        err: ErrorCode,
                                        fees_json: *const c_char))
        </pre>
    </td>
  </tr>
</table>

### Pool API

The next stable release 1.4 of Indy-Node contains breaking changes related to the transaction format.
The changes are done for better versioning support (to support compatibility in the future) and more
readable format of data (separating payload data, metadata and signature).

LibIndy 1.5 supports both Indy Node protocols Indy Node v1.3 (version 1) and v1.4 (version 2).
There is a global property PROTOCOL_VERSION that used in every request to the pool:

* PROTOCOL_VERSION=1 for IndyNode 1.3
* PROTOCOL_VERSION=2 for IndyNode 1.4

To switch between protocol versions used libindy 1.5 provides new ```indy_set_protocol_version``` endpoint:

* By default LibIndy 1.5 is compatible with Indy Node 1.3, and not 1.4
* LibIndy 1.5 can become compatible with Indy Node 1.4 if ```indy_set_protocol_version(2)```
  is called during app initialization.
* An application can freely update to LibIndy 1.5 and still use stable Node 1.3
* If an app wants to work with the latest master or Stable Node 1.4, then it needs to
  support breaking changes (there are not so many, mostly a new reply for write txns as txn format is changed).
  See References.

References:

* [Indy-Node 1.3 to 1.4 Migration Guide](https://github.com/hyperledger/indy-node/blob/master/docs/1.3_to_1.4_migration_guide.md)

<table>
  <tr>
    <th>v1.4.0 - Pool API</th>
    <th>v1.5.0 - Pool API</th>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/pool.rs#L263">
            Set protocol version
        </a>
    </th>
  </tr>
  <tr>
    <td><b>NEW</b></td>
    <td>
      <pre>
indy_set_protocol_version(command_handle: i32,
                          protocol_version: usize,
                          cb: fn(xcommand_handle: i32,
                                 err: ErrorCode))
        </pre>
    </td>
  </tr>
</table>

### Ledger API

* ```indy_multi_sign_request``` was added to Libindy Pool API to allow signing of requests by multiple identity owners.
* ```indy_build_get_validator_info_request``` was added to allow building of VALIDATOR_INFO action request. See References.
* ```indy_build_pool_restart_request``` was added to allow building of POOL_RESTART action request. See References.
* ```indy_register_transaction_parser_for_sp``` endpoint was added to allow usage of
  StateProof optimization in Client-Node communication with a custom transactions that can be added by
  Node plugins.
* ```indy_build_get_txn_request``` endpoint was changed to allow reading of non-domain ledgers transactions

References:

* [Indy Node Action Requests Design](https://github.com/hyperledger/indy-node/blob/master/docs/requests.md#action-requests)

<table>
  <tr>
    <th colspan="2">v1.4.0 - Ledger API</th>
    <th colspan="2">v1.5.0 - Ledger API</th>
  </tr>  
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/ledger.rs#L178">
            Multi signs request message
        </a>
    </th>
  </tr>
  <tr>
    <td><b>NEW</b></td>
    <td>
      <pre>
indy_multi_sign_request(command_handle: i32,
                        wallet_handle: i32,
                        submitter_did: *const c_char,
                        request_json: *const c_char,
                        cb: fn(xcommand_handle: i32,
                               err: ErrorCode,
                               signed_request_json: *const c_char))
        </pre>
    </td>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/ledger.rs#L892">
            Builds a GET_VALIDATOR_INFO request
        </a>
    </th>
  </tr>
  <tr>
    <td><b>NEW</b></td>
    <td>
      <pre>
indy_build_get_validator_info_request(command_handle: i32,
                                      submitter_did: *const c_char,
                                      cb: fn(xcommand_handle: i32,
                                             err: ErrorCode,
                                             request_json: *const c_char))
        </pre>
    </td>
  </tr>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/ledger.rs#L1029">
            Builds a POOL_RESTART request
        </a>
    </th>
  </tr>
  <tr>
    <td><b>NEW</b></td>
    <td colspan="2">
      <pre>
indy_build_pool_restart_request(command_handle: i32,
                                submitter_did: *const c_char,
                                action: *const c_char,
                                datetime: *const c_char,
                                cb: fn(xcommand_handle: i32,
                                       err: ErrorCode,
                                       request_json: *const c_char))
        </pre>
    </td>
  </tr>  
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/ledger.rs#L1644">
            Register custom callbacks for parsing of state proofs
        </a>
    </th>
  </tr>
  <tr>
    <td><b>NEW</b></td>
    <td colspan="2">
      <pre>
indy_register_transaction_parser_for_sp(command_handle: i32,
                                        txn_type: *const c_char,
                                        parser: CustomTransactionParser,
                                        free: CustomFree,
                                        cb: fn(command_handle_: i32,
                                               err: ErrorCode))
        </pre>
    </td>
  </tr>
  <tr>
    <th colspan="2">
      <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/ledger.rs#L925">
        Builds a GET_TXN request. Request to get any transaction by its seq_no
      </a>
    </th>
  </tr>
  <tr>
    <td>
      <pre>
indy_build_get_txn_request(
            command_handle: i32,
            submitter_did: *const c_char,
            seq_no: i32,
            cb: fn(xcommand_handle: i32,
                   err: ErrorCode,
                   request_json: *const c_char))
      </pre>
    </td>
    <td>
      <pre>
indy_build_get_txn_request(
            command_handle: i32,
            submitter_did: *const c_char,
            ledger_type: *const c_char,
            seq_no: i32,
            cb: fn(xcommand_handle: i32,
                   err: ErrorCode,
                   request_json: *const c_char))
      </pre>
      <b>Note:</b> ledger_type parameter was added to provide a string identifier of a target ledger.
    </td>
  </tr>
</table>