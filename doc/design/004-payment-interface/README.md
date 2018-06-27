# Payment Interface

This design proposes to make libindy aware about payments and tokens that can be implemented with Indy infrastructure.

## Goals and ideas

* Libindy should be aware about some details of payments in Indy infrastructure:
  * The idea of payments in general. Transactions might need to be paid for, transactions
    can be used for money/tokens transfer.
  * Concept of different Payment Methods that can be plugged to libindy
    (like Sovrin tokens, Bitcoin tokens and etc...). A payment method in libindy might be
    identified by prefix: “pay:sov” could be the prefix for the Sovrin token payment method,
    and “pay:xyz” could be the prefix for a different payment method.
  * Concept of Payment Address that is common for supported Payment Methods. Different
    payment use different format of Payment Address, but there is agreement on fully resolvable
    payment address format.  This is very much like the notion of the general DID spec with
    sub-specs called DID method specs that are associated with a name.
    Payment addresses would be things like “pay:sov:12345”.
  * The idea of UTXO and that payments take inputs and outputs.
  * General payment errors that might happen (e.g., “insufficient funds”).
* Out-of-box libindy will not provide support of any payment method, but there will be
  API to register payment methods.
* Each payment method should be aware about:
  * Its payment method prefix, and the format of its payment addresses.
  * How to create pure payment transactions, such as those that transfers tokens, mint tokens,
    or lookup balances.
  * How to modify an unsigned non-payment transaction (e.g., NYM) to pay fees.
  * How to modify transaction signing in a way that satisfies its payment method.
  * Possibly, special payment addresses that are significant to its method
    (e.g., the payment address at which the Sovrin Foundation receives fees).
* Libindy should provide generic API for payment addresses creation, building of payment-related transactions,
  assigning fees to transactions. This API will look to registered payment methods and call corresponded
  handlers.
* Payments interface must be interoperable as possible between different payment methods.

![Payment Interface](./payment-interface.svg)

## Payment Method API

Payment Method API will allow to register custom payment method implementation
by calling ```indy_register_payment_method``` call:

```Rust
/// Register custom payment implementation.
///
/// It allows library user to provide custom payment method implementation as set of handlers.
///
/// #Params
/// command_handle: Command handle to map callback to caller context.
/// payment_method: The type of payment method also used as sub-prefix for fully resolvable payment address format ("sov" - for example)
/// create_payment_address: "create_payment_address" operation handler
/// add_request_fees: "add_request_fees" operation handler
/// build_get_utxo_request: "build_get_utxo_request" operation handler
/// parse_get_utxo_response: "parse_get_utxo_response" operation handler
/// build_payment_req: "build_payment_req" operation handler
/// build_mint_req: "build_mint_req" operation handler
///
/// #Returns
/// Error code
#[no_mangle]
pub extern fn indy_register_payment_method(command_handle: i32,
                                           payment_method: *const c_char,

                                           create_payment_address: Option<CreatePaymentAddressCB>,
                                           add_request_fees: Option<AddRequestFeesCB>,
                                           parse_response_with_fees: Option<ParseResponseWithFeesCB>,
                                           build_get_utxo_request: Option<BuildGetUTXORequestCB>,
                                           parse_get_utxo_response: Option<ParseGetUTXOResponseCB>,
                                           build_payment_req: Option<BuildPaymentReqCB>,
                                           parse_payment_response: Option<ParsePaymentResponseCB>,
                                           build_mint_req: Option<BuildMintReqCB>,
                                           build_set_txn_fees_req: Option<BuildSetTxnFeesReqCB>,
                                           build_get_txn_fees_req: Option<BuildGetTxnFeesReqCB>,
                                           parse_get_txn_fees_response: Option<ParseGetTxnFeesResponseCB>,

                                           cb: Option<extern fn(command_handle_: i32,
                                                                err: ErrorCode) -> ErrorCode>) -> ErrorCode {}
```

### Payment Method Handler Interface

Registered functions will be called by libindy as part of processing libindy API calls.
Libindy will pass its own callback to the functions to retrieve result of the 3rd party function implementation.
The list below is type description for registered calls.

```Rust
/// Create the payment address for this payment method.
///
/// This method generates private part of payment address
/// and stores it in a secure place. Ideally it should be
/// secret in libindy wallet (see crypto module).
///
/// Note that payment method should be able to resolve this
/// secret by fully resolvable payment address format.
///
/// #Params
/// command_handle: command handle to map callback to context
/// wallet_handle: wallet handle where keys for signature are stored
/// config: payment address config as json:
///   {
///     seed: <str>, // allows deterministic creation of payment address
///   }
///
/// #Returns
/// payment_address - public identifier of payment address in fully resolvable payment address format
type CreatePaymentAddressCB = extern fn(command_handle: i32,
                                        wallet_handle: i32,
                                        config: *const c_char,
                                        cb: Option<extern fn(command_handle_: i32,
                                                             err: ErrorCode,
                                                             payment_address: *const c_char) -> ErrorCode>) -> ErrorCode;

/// Modifies Indy request by adding information how to pay fees for this transaction
/// according to this payment method.
///
/// This method consumes set of UTXO inputs and outputs. The difference between inputs balance
/// and outputs balance is the fee for this transaction.
///
/// Not that this method also produces correct fee signatures.
///
/// Format of inputs is specific for payment method. Usually it should reference payment transaction
/// with at least one output that corresponds to payment address that user owns.
///
/// #Params
/// wallet_handle: wallet handle
/// submitter_did : DID of request sender
/// req_json: initial transaction request as json
/// inputs_json: The list of UTXO inputs as json array:
///   ["input1", ...]
///   Note that each input should reference paymentAddress
/// outputs_json: The list of UTXO outputs as json array:
///   [{
///     paymentAddress: <str>, // payment address used as output
///     amount: <int>, // amount of tokens to transfer to this payment address
///     extra: <str>, // optional data
///   }]
///
/// #Returns
/// req_with_fees_json - modified Indy request with added fees info
type AddRequestFeesCB = extern fn(command_handle: i32,
                                  wallet_handle: i32,
                                  submitter_did: *const c_char,
                                  req_json: *const c_char,
                                  inputs_json: *const c_char,
                                  outputs_json: *const c_char,
                                  cb: Option<extern fn(command_handle_: i32,
                                                       err: ErrorCode,
                                                       req_with_fees_json: *const c_char) -> ErrorCode>) -> ErrorCode;

/// Parses response for Indy request with fees.
///
/// #Params
/// command_handle
/// resp_json: response for Indy request with fees
///   Note: this param will be used to determine payment_method
///
/// #Returns
/// utxo_json - parsed (payment method and node version agnostic) utxo info as json:
///   [{
///      txo: <str>, // UTXO input
///      paymentAddress: <str>, //payment address for this UTXO
///      amount: <int>, // amount of tokens in this input
///      extra: <str>, // optional data from payment transaction
///   }]
type ParseResponseWithFeesCB = extern fn(command_handle: i32,
                                         resp_json: *const c_char,
                                         cb: Option<extern fn(command_handle_: i32,
                                                              err: ErrorCode,
                                                              utxo_json: *const c_char) -> ErrorCode>) -> ErrorCode;
                                                       
/// Builds Indy request for getting UTXO list for payment address
/// according to this payment method.
///
/// #Params
/// wallet_handle: wallet handle
/// submitter_did : DID of request sender
/// payment_address: target payment address
///
/// #Returns
/// get_utxo_txn_json - Indy request for getting UTXO list for payment address
type BuildGetUTXORequestCB = extern fn(command_handle: i32,
                                       wallet_handle: i32,
                                       submitter_did: *const c_char,
                                       payment_address: *const c_char,
                                       cb: Option<extern fn(command_handle_: i32,
                                                            err: ErrorCode,
                                                            get_utxo_txn_json: *const c_char) -> ErrorCode>) -> ErrorCode;

/// Parses response for Indy request for getting UTXO list.
///
/// #Params
/// resp_json: response for Indy request for getting UTXO list
///
/// #Returns
/// utxo_json - parsed (payment method and node version agnostic) utxo info as json:
///   [{
///      txo: <str>, // UTXO input
///      paymentAddress: <str>, //payment address for this UTXO
///      amount: <int>, // amount of tokens in this input
///      extra: <str>, // optional data from payment transaction
///   }]
type ParseGetUTXOResponseCB = extern fn(command_handle: i32,
                                        resp_json: *const c_char,
                                        cb: Option<extern fn(command_handle_: i32,
                                                             err: ErrorCode,
                                                             utxo_json: *const c_char) -> ErrorCode>) -> ErrorCode;

/// Builds Indy request for doing tokens payment
/// according to this payment method.
///
/// This method consumes set of UTXO inputs and outputs.
///
/// Format of inputs is specific for payment method. Usually it should reference payment transaction
/// with at least one output that corresponds to payment address that user owns.
///
/// #Params
/// wallet_handle: wallet handle
/// submitter_did : DID of request sender
/// inputs_json: The list of UTXO inputs as json array:
///   ["input1", ...]
///   Note that each input should reference paymentAddress
/// outputs_json: The list of UTXO outputs as json array:
///   [{
///     paymentAddress: <str>, // payment address used as output
///     amount: <int>, // amount of tokens to transfer to this payment address
///     extra: <str>, // optional data
///   }]
///
/// #Returns
/// payment_req_json - Indy request for doing tokens payment
type BuildPaymentReqCB = extern fn(command_handle: i32,
                                   wallet_handle: i32,
                                   submitter_did: *const c_char,
                                   inputs_json: *const c_char,
                                   outputs_json: *const c_char,
                                   cb: Option<extern fn(command_handle_: i32,
                                                        err: ErrorCode,
                                                        payment_req_json: *const c_char) -> ErrorCode>) -> ErrorCode;

/// Parses response for Indy request for payment txn.
///
/// #Params
/// command_handle
/// resp_json: response for Indy request for payment txn
///   Note: this param will be used to determine payment_method
///
/// #Returns
/// utxo_json - parsed (payment method and node version agnostic) utxo info as json:
///   [{
///      txo: <str>, // UTXO input
///      paymentAddress: <str>, //payment address for this UTXO
///      amount: <int>, // amount of tokens in this input
///      extra: <str>, // optional data from payment transaction
///   }]
type ParsePaymentResponseCB = extern fn(command_handle: i32,
                                        resp_json: *const c_char,
                                        cb: Option<extern fn(command_handle_: i32,
                                                             err: ErrorCode,
                                                             utxo_json: *const c_char) -> ErrorCode>) -> ErrorCode;

/// Builds Indy request for doing tokens minting
/// according to this payment method.
///
/// #Params
/// wallet_handle: wallet handle
/// submitter_did : DID of request sender
/// outputs_json: The list of UTXO outputs as json array:
///   [{
///     paymentAddress: <str>, // payment address used as output
///     amount: <int>, // amount of tokens to transfer to this payment address
///     extra: <str>, // optional data
///   }]
///
/// #Returns
/// mint_req_json - Indy request for doing tokens minting
type BuildMintReqCB = extern fn(command_handle: i32,
                                wallet_handle: i32,
                                submitter_did: *const c_char,
                                outputs_json: *const c_char,
                                cb: Option<extern fn(command_handle_: i32,
                                                     err: ErrorCode,
                                                     mint_req_json: *const c_char) -> ErrorCode>) -> ErrorCode;

/// Builds Indy request for setting fees for transactions in the ledger
/// 
/// # Params
/// command_handle
/// wallet_handle: wallet handle
/// submitter_did : DID of request sender
/// fees_json {
///   txnType1: amount1,
///   txnType2: amount2,
///   .................
///   txnTypeN: amountN,
/// }
///
/// # Return
/// set_txn_fees_json - Indy request for setting fees for transactions in the ledger
type BuildSetTxnFeesReqCB = extern fn(command_handle: i32,
                                      wallet_handle: i32,
                                      submitter_did: *const c_char,
                                      fees_json: *const c_char,
                                      cb: Option<extern fn(command_handle_: i32,
                                                           err: ErrorCode,
                                                           set_txn_fees_json: *const c_char) -> ErrorCode>) -> ErrorCode;

/// Builds Indy get request for getting fees for transactions in the ledger
/// 
/// # Params
/// command_handle
/// wallet_handle: wallet handle
/// submitter_did : DID of request sender
/// 
/// # Return
/// get_txn_fees_json - Indy request for getting fees for transactions in the ledger
type BuildGetTxnFeesReqCB = extern fn(command_handle: i32,
                                      wallet_handle: i32,
                                      submitter_did: *const c_char,
                                      cb: Option<extern fn(command_handle_: i32,
                                                           err: ErrorCode,
                                                           get_txn_fees_json: *const c_char) -> ErrorCode>) -> ErrorCode;

/// Parses response for Indy request for getting fees
///
/// # Params
/// command_handle
/// payment_method
/// resp_json: response for Indy request for getting fees
/// 
/// # Return
/// fees_json {
///   txnType1: amount1,
///   txnType2: amount2,
///   .................
///   txnTypeN: amountN,
/// }                                           
type ParseGetTxnFeesResponseCB = extern fn(command_handle: i32,
                                           resp_json: *const c_char,
                                           cb: Option<extern fn(command_handle_: i32,
                                                                err: ErrorCode,
                                                                fees_json: *const c_char) -> ErrorCode>) -> ErrorCode;                                                       

```

## Payment API

Some API calls have dedicated parameter to determine payment_method.
Another part of calls use assumptions about input parameters format to determine payment_method

```Rust
/// Create the payment address for specified payment method
///
///
/// This method generates private part of payment address
/// and stores it in a secure place. Ideally it should be
/// secret in libindy wallet (see crypto module).
///
/// Note that payment method should be able to resolve this
/// secret by fully resolvable payment address format.
///
/// #Params
/// command_handle: command handle to map callback to context
/// wallet_handle: wallet handle where to save new address
/// payment_method: Payment method to use (for example, 'sov')
/// config: payment address config as json:
///   {
///     seed: <str>, // allows deterministic creation of payment address
///   }
///
/// #Returns
/// payment_address - public identifier of payment address in fully resolvable payment address format
pub extern fn indy_create_payment_address(command_handle: i32,
                                          wallet_handle: i32,
                                          payment_method: *const c_char,
                                          config: *const c_char,
                                          cb: Option<extern fn(command_handle_: i32,
                                                               err: ErrorCode,
                                                               payment_address: *const c_char) -> ErrorCode>) -> ErrorCode {}
                                                               
/// Lists all payment addresses that are stored in the wallet
///
/// #Params
/// command_handle: command handle to map callback to context
/// wallet_handle: wallet to search for payment_addresses in
///
/// #Returns
/// payment_addresses_json - json array of string with json addresses
pub extern fn indy_list_payment_addresses(command_handle: i32,
                                          wallet_handle: i32,
                                          cb: Option<extern fn(command_handle_: i32,
                                                               err: ErrorCode,
                                                               payment_addresses_json: *const c_char)>) -> ErrorCode {}

/// Modifies Indy request by adding information how to pay fees for this transaction
/// according to selected payment method.
///
/// Payment selection is performed by looking to o
///
/// This method consumes set of UTXO inputs and outputs. The difference between inputs balance
/// and outputs balance is the fee for this transaction.
///
/// Not that this method also produces correct fee signatures.
///
/// Format of inputs is specific for payment method. Usually it should reference payment transaction
/// with at least one output that corresponds to payment address that user owns.
///
/// #Params
/// wallet_handle: wallet handle
/// submitter_did : DID of request sender
/// req_json: initial transaction request as json
/// inputs_json: The list of UTXO inputs as json array:
///   ["input1", ...]
///   Notes:
///     - each input should reference paymentAddress
///     - this param will be used to determine payment_method
/// outputs_json: The list of UTXO outputs as json array:
///   [{
///     paymentAddress: <str>, // payment address used as output
///     amount: <int>, // amount of tokens to transfer to this payment address
///     extra: <str>, // optional data
///   }]
///
/// #Returns
/// req_with_fees_json - modified Indy request with added fees info
/// payment_method
pub extern fn indy_add_request_fees(command_handle: i32,
                                    wallet_handle: i32,
                                    submitter_did: *const c_char,
                                    req_json: *const c_char,
                                    inputs_json: *const c_char,
                                    outputs_json: *const c_char,
                                    cb: Option<extern fn(command_handle_: i32,
                                                         err: ErrorCode,
                                                         req_with_fees_json: *const c_char,
                                                         payment_method: *const c_char) -> ErrorCode>) -> ErrorCode {}

/// Parses response for Indy request with fees.
///
/// #Params
/// command_handle
/// payment_method
/// resp_json: response for Indy request with fees
///   Note: this param will be used to determine payment_method
///
/// #Returns
/// utxo_json - parsed (payment method and node version agnostic) utxo info as json:
///   [{
///      txo: <str>, // UTXO input
///      paymentAddress: <str>, //payment address for this UTXO
///      amount: <int>, // amount of tokens in this input
///      extra: <str>, // optional data from payment transaction
///   }]
pub extern fn indy_parse_response_with_fees(command_handle: i32,
                                            payment_method: *const c_char,
                                            resp_json: *const c_char,
                                            cb: Option<extern fn(command_handle_: i32,
                                                                 err: ErrorCode,
                                                                 utxo_json: *const c_char) -> ErrorCode>) -> ErrorCode {}

/// Builds Indy request for getting UTXO list for payment address
/// according to this payment method.
///
/// #Params
/// wallet_handle: wallet handle
/// submitter_did : DID of request sender
/// payment_address: target payment address
///
/// #Returns
/// get_utxo_txn_json - Indy request for getting UTXO list for payment address
/// payment_method
pub extern fn indy_build_get_utxo_request(command_handle: i32,
                                          wallet_handle: i32,
                                          submitter_did: *const c_char,
                                          payment_address: *const c_char,
                                          cb: Option<extern fn(command_handle_: i32,
                                                               err: ErrorCode,
                                                               get_utxo_txn_json: *const c_char,
                                                               payment_method: *const c_char) -> ErrorCode>) -> ErrorCode {}

/// Parses response for Indy request for getting UTXO list.
///
/// #Params
/// resp_json: response for Indy request for getting UTXO list
///   Note: this param will be used to determine payment_method
///
/// #Returns
/// utxo_json - parsed (payment method and node version agnostic) utxo info as json:
///   [{
///      txo: <str>, // UTXO input
///      paymentAddress: <str>, //payment address for this UTXO
///      amount: <int>, // amount of tokens in this input
///      extra: <str>, // optional data from payment transaction
///   }]
pub extern fn indy_parse_get_utxo_response(command_handle: i32,
                                           payment_method: *const c_char,
                                           resp_json: *const c_char,
                                           cb: Option<extern fn(command_handle_: i32,
                                                                err: ErrorCode,
                                                                utxo_json: *const c_char) -> ErrorCode>) -> ErrorCode {}

/// Builds Indy request for doing tokens payment
/// according to this payment method.
///
/// This method consumes set of UTXO inputs and outputs.
///
/// Format of inputs is specific for payment method. Usually it should reference payment transaction
/// with at least one output that corresponds to payment address that user owns.
///
/// #Params
/// wallet_handle: wallet handle
/// submitter_did : DID of request sender
/// inputs_json: The list of UTXO inputs as json array:
///   ["input1", ...]
///   Note that each input should reference paymentAddress
/// outputs_json: The list of UTXO outputs as json array:
///   [{
///     paymentAddress: <str>, // payment address used as output
///     amount: <int>, // amount of tokens to transfer to this payment address
///     extra: <str>, // optional data
///   }]
///
/// #Returns
/// payment_req_json - Indy request for doing tokens payment
/// payment_method
pub extern fn indy_build_payment_req(command_handle: i32,
                                     wallet_handle: i32,
                                     submitter_did: *const c_char,
                                     inputs_json: *const c_char,
                                     outputs_json: *const c_char,
                                     cb: Option<extern fn(command_handle_: i32,
                                                          err: ErrorCode,
                                                          payment_req_json: *const c_char,
                                                          payment_method: *const c_char) -> ErrorCode>) -> ErrorCode {}

/// Parses response for Indy request for payment txn.
///
/// #Params
/// command_handle
/// payment_method
/// resp_json: response for Indy request for payment txn
///   Note: this param will be used to determine payment_method
///
/// #Returns
/// utxo_json - parsed (payment method and node version agnostic) utxo info as json:
///   [{
///      txo: <str>, // UTXO input
///      paymentAddress: <str>, //payment address for this UTXO
///      amount: <int>, // amount of tokens in this input
///      extra: <str>, // optional data from payment transaction
///   }]
pub extern fn indy_parse_payment_response(command_handle: i32,
                                          payment_method: *const c_char,
                                          resp_json: *const c_char,
                                          cb: Option<extern fn(command_handle_: i32,
                                                               err: ErrorCode,
                                                               utxo_json: *const c_char) -> ErrorCode>) -> ErrorCode {}

/// Builds Indy request for doing tokens minting
/// according to this payment method.
///
/// #Params
/// wallet_handle: wallet handle
/// submitter_did : DID of request sender
/// outputs_json: The list of UTXO outputs as json array:
///   [{
///     paymentAddress: <str>, // payment address used as output
///     amount: <int>, // amount of tokens to transfer to this payment address
///     extra: <str>, // optional data
///   }]
///
/// #Returns
/// mint_req_json - Indy request for doing tokens minting
/// payment_method
pub extern fn indy_build_mint_req(command_handle: i32,
                                  wallet_handle: i32,
                                  submitter_did: *const c_char,
                                  outputs_json: *const c_char,
                                  cb: Option<extern fn(command_handle_: i32,
                                                       err: ErrorCode,
                                                       mint_req_json: *const c_char,
                                                       payment_method: *const c_char) -> ErrorCode>) -> ErrorCode {}

/// Builds Indy request for setting fees for transactions in the ledger
/// 
/// # Params
/// command_handle
/// wallet_handle: wallet handle
/// submitter_did : DID of request sender
/// payment_method
/// fees_json {
///   txnType1: amount1,
///   txnType2: amount2,
///   .................
///   txnTypeN: amountN,
/// }
/// # Return
/// set_txn_fees_json - Indy request for setting fees for transactions in the ledger
pub extern fn indy_build_set_txn_fees_req(command_handle: i32,
                                          wallet_handle: i32,
                                          submitter_did: *const c_char,
                                          payment_method: *const c_char,
                                          fees_json: *const c_char,
                                          cb: Option<extern fn(command_handle_: i32,
                                                               err: ErrorCode,
                                                               set_txn_fees_json: *const c_char) -> ErrorCode>) -> ErrorCode {}

/// Builds Indy get request for getting fees for transactions in the ledger
/// 
/// # Params
/// command_handle
/// wallet_handle: wallet handle
/// submitter_did : DID of request sender
/// payment_method
///
/// # Return
/// get_txn_fees_json - Indy request for getting fees for transactions in the ledger
pub extern fn indy_build_get_txn_fees_req(command_handle: i32,
                                          wallet_handle: i32,
                                          submitter_did: *const c_char,
                                          payment_method: *const c_char,
                                          cb: Option<extern fn(command_handle_: i32,
                                                               err: ErrorCode,
                                                               get_txn_fees_json: *const c_char) -> ErrorCode>) -> ErrorCode {}

/// Parses response for Indy request for getting fees
///
/// # Params
/// command_handle
/// payment_method
/// resp_json: response for Indy request for getting fees
///
/// # Return
/// fees_json {
///   txnType1: amount1,
///   txnType2: amount2,
///   .................
///   txnTypeN: amountN,
/// }
pub extern fn indy_parse_get_txn_fees_response(command_handle: i32,
                                               payment_method: *const c_char,
                                               resp_json: *const c_char,
                                               cb: Option<extern fn(command_handle_: i32,
                                                                    err: ErrorCode,
                                                                    fees_json: *const c_char) -> ErrorCode>) -> ErrorCode {}

```