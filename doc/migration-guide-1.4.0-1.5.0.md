# Libindy migration Guide from v.1.4.0 to 1.5.0

## A Developer Guide for Libindy migration

This document is written for developers using Libindy 1.4.0 to provide necessary information and
to simplify their transition to API of Libindy 1.5.0.

* [Notes](#notes)
* [Api]()
    * [Wallet API](#wallet-api)
    * [Non-Secrets API](#non-secrets-api)
    * [Payments API](#payments-api)
    * [Pool API](#pool-api)
    * [Ledger API](#ledger-api)

### Notes

In the following tables, there are function that have been changed or added in Libindy 1.5.0. 

* To get more details about current format of a function click on the description above it.
* Bellow are signatures of functions in Libindy C API.
 The params of <b>cb</b> (except command_handle and err) will be result values of the similar function in any Libindy wrapper.


### Wallet API
Performed significant changes related to Wallet Storage. The complete Wallet Design document can be found [here](https://github.com/hyperledger/indy-sdk/tree/master/doc/design/003-wallet-storage).
* Changed API of Plugged Wallet storage to extend set of commands related to working with stored data. 
<table>  
  <th>v1.4.0 - Wallet API</th>
  <th>v1.5.0 - Wallet API</th>
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
</table>

<br>

* Plugged wallet used to handle both security and storage layers. Now all encryption performs on Libindy level. \
Because of this, `credentials_json` became the required parameter for the following functions:
    * [indy_create_wallet](https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/wallet.rs#L142) - Creates a new secure wallet
    * [indy_open_wallet](https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/wallet.rs#L210) - Opens the wallet with specific name
    * [indy_delete_wallet](https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/wallet.rs#L475) - Deletes created wallet
    
The parameter `credentials_json` has the following format:
```
{
    "key": string,
    "rekey": Optional<string>,
    "storage": Optional<object>  List of supported keys are defined by wallet type.
}
```
* The format of storing data has been changed to support efficient search.
* There have been added functions that allow performing Export/Import of Wallet.

<table>  
  <th>v1.5.0 - Wallet API</th>
  <tr>
    <th>
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/wallet.rs#L311">
            Export wallet
        </a>
    </th>
    </tr>
    <tr>
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
    <th>
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/wallet.rs#L359">
            Import wallet
        </a>
    </th>
    </tr>
    <tr>
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
This API is intended to store and read application specific identity data in the wallet.\
This API have not an access to secrets stored by Secret Entities API.

<table>  
  <th>v1.5.0 - Non-Secrets API</th>
  <tr>
    <th>
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/non_secrets.rs#L12">
            Create a new record in the wallet
        </a>
    </th>
        </tr>
        <tr>
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
    <th>
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/non_secrets.rs#L71">
            Update a wallet record value
        </a>
    </th>
        </tr>
        <tr>
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
    <th>
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/non_secrets.rs#L116">
            Update a wallet record tags
        </a>
    </th>
        </tr>
        <tr>
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
    <th>
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/non_secrets.rs#L170">
            Add new tags to the wallet record
        </a>
    </th>
        </tr>
        <tr>
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
    <th>
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/non_secrets.rs#L226">
            Delete tags from the wallet record
        </a>
    </th>
        </tr>
        <tr>
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
    <th>
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/non_secrets.rs#L272">
            Delete an existing wallet record in the wallet
        </a>
    </th>
        </tr>
        <tr>
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
    <th>
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/non_secrets.rs#L313">
            Get an wallet record by id
        </a>
    </th>
        </tr>
        <tr>
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
    <th>
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/non_secrets.rs#L373">
            Search for wallet records
        </a>
    </th>
        </tr>
        <tr>
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
    <th>
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/non_secrets.rs#L439">
            Fetch next records for wallet search.
        </a>
    </th>
        </tr>
        <tr>
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
    <th>
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/non_secrets.rs#L493">
            Close wallet search
        </a>
    </th>
        </tr>
        <tr>
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

The complete Payment design document can be found [here](https://github.com/hyperledger/indy-sdk/tree/master/doc/design/004-payment-interface)

<table>  
  <th colspan="2">v1.5.0 - Payments API</th>
  <tr>
    <th>
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/payments.rs#L264">
            Register custom payment implementation
        </a>
    </th>
    </tr>
    <tr>
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
    <th>
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/payments.rs#L345">
            Create the payment address for specified payment method
        </a>
    </th>
    </tr>
    <tr>
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
    <th>
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/payments.rs#L404">
            Lists all payment addresses that are stored in the wallet
        </a>
    </th>
    </tr>
    <tr>
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
    <th>
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/payments.rs#L444">
            Modifies Indy request by adding information how to pay fees for this transaction
            according to selected payment method
        </a>
    </th>
    </tr>
    <tr>
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
    <th>
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/payments.rs#L521">
            Parses response for Indy request with fees
        </a>
    </th>
    </tr>
    <tr>
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
    <th>
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/payments.rs#L568">
            Builds Indy request for getting UTXO list for payment address
            according to this payment method
        </a>
    </th>
    </tr>
    <tr>
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
    <th>
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/payments.rs#L618">
            Parses response for Indy request for getting UTXO list
        </a>
    </th>
    </tr>
    <tr>
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
    <th>
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/payments.rs#L666">
            Builds Indy request for doing tokens payment according to this payment method
        </a>
    </th>
    </tr>
    <tr>
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
    <th>
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/payments.rs#L723">
            Parses response for Indy request for payment txn
        </a>
    </th>
    </tr>
    <tr>
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
    <th>
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/payments.rs#L782">
            Builds Indy request for doing tokens minting according to this payment method
        </a>
    </th>
    </tr>
    <tr>
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
    <th>
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/payments.rs#L837">
            Builds Indy request for setting fees for transactions in the ledger
        </a>
    </th>
    </tr>
    <tr>
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
    <th>
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/payments.rs#L892">
            Builds Indy get request for getting fees for transactions in the ledger
        </a>
    </th>
    </tr>
    <tr>
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
    <th>
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/payments.rs#L939">
            Parses response for Indy request for getting fees
        </a>
    </th>
    </tr>
    <tr>
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
LibIndy 1.5.0 is compatible with two version of IndyNode 1.3, and 1.4.\
By default LibIndy 1.5.0 works with IndyNode 1.3.\
There is a global property PROTOCOL_VERSION that used in every request to the pool:
* PROTOCOL_VERSION=1 for IndyNode 1.3
* PROTOCOL_VERSION=2 for IndyNode 1.4

The function `indy_set_protocol_version` has been added to Libindy Pool API.\
This function should be called during app initialization to set PROTOCOL_VERSION.

<table>
  <th>v1.5.0 - Pool API</th>
  <tr>
    <th>
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/pool.rs#L263">
            Set protocol version
        </a>
    </th>
    </tr>
    <tr>
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

<table>
  <th colspan="2">v1.5.0 - Ledger API</th>
  <tr>
    <th colspan="2">
        <a href="https://github.com/hyperledger/indy-sdk/blob/master/libindy/src/api/ledger.rs#L178">
            Multi signs request message
        </a>
    </th>
    </tr>
    <tr>
    <td colspan="2">
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
    <td colspan="2">
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
    <th>v1.4.0 - Ledger API</th>
    <th>v1.5.0 - Ledger API</th>
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
    </td>
  </tr>
</table>