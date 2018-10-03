#ifndef __indy__payment__included__
#define __indy__payment__included__

#include "indy_mod.h"
#include "indy_types.h"

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
typedef indy_error_t (*indyCreatePaymentAddressCB)(indy_handle_t command_handle,
                                                   indy_handle_t wallet_handle,
                                                   const char* config,
                                                   indy_err_str_cb cb);

/// Modifies Indy request by adding information how to pay fees for this transaction
/// according to this payment method.
///
/// This method consumes set of inputs and outputs. The difference between inputs balance
/// and outputs balance is the fee for this transaction.
///
/// Not that this method also produces correct fee signatures.
///
/// Format of inputs is specific for payment method. Usually it should reference payment transaction
/// with at least one output that corresponds to payment address that user owns.
///
/// #Params
/// command_handle: command handle to map callback to context
/// wallet_handle: wallet handle
/// submitter_did: (Optional) DID of request sender
/// req_json: initial transaction request as json
/// inputs_json: The list of payment sources as json array:
///   ["source1", ...]
///   Note that each source should reference payment address
/// outputs_json: The list of outputs as json array:
///   [{
///     recipient: <str>, // payment address of recipient
///     amount: <int>, // amount
///   }]
/// extra: // optional information for payment operation
///
/// #Returns
/// req_with_fees_json - modified Indy request with added fees info
typedef indy_error_t (*indyAddRequestFeesCB)(indy_handle_t command_handle,
                                             indy_handle_t wallet_handle,
                                             const char* submitter_did,
                                             const char* req_json,
                                             const char* inputs_json,
                                             const char* outputs_json,
                                             const char* extra,
                                             indy_err_str_cb cb);

/// Parses response for Indy request with fees.
///
/// #Params
/// command_handle: command handle to map callback to context
/// resp_json: response for Indy request with fees
///
/// #Returns
/// receipts_json - parsed (payment method and node version agnostic) receipts info as json:
///   [{
///      receipt: <str>, // receipt that can be used for payment referencing and verification
///      recipient: <str>, //payment address for this recipient
///      amount: <int>, // amount
///      extra: <str>, // optional data from payment transaction
///   }]
typedef indy_error_t (*indyParseResponseWithFeesCB)(indy_handle_t command_handle,
                                                    const char* resp_json,
                                                    indy_err_str_cb cb);

/// Builds Indy request for getting sources list for payment address
/// according to this payment method.
///
/// #Params
/// command_handle: command handle to map callback to context
/// wallet_handle: wallet handle
/// submitter_did: (Optional) DID of request sender
/// payment_address: target payment address
///
/// #Returns
/// get_sources_txn_json - Indy request for getting sources list for payment address
typedef indy_error_t (*indyBuildGetPaymentSourcesRequestCB)(indy_handle_t command_handle,
                                                            indy_handle_t wallet_handle,
                                                            const char* submitter_did,
                                                            const char* payment_address,
                                                            indy_err_str_cb cb);

/// Parses response for Indy request for getting sources list.
///
/// #Params
/// command_handle: command handle to map callback to context
/// resp_json: response for Indy request for getting sources list
///
/// #Returns
/// sources_json - parsed (payment method and node version agnostic) sources info as json:
///   [{
///      source: <str>, // source input
///      paymentAddress: <str>, //payment address for this source
///      amount: <int>, // amount
///      extra: <str>, // optional data from payment transaction
///   }]
typedef indy_error_t (*indyParseGetPaymentSourcesResponseCB)(indy_handle_t command_handle,
                                                             const char* resp_json,
                                                             indy_err_str_cb cb);

/// Builds Indy request for doing payment
/// according to this payment method.
///
/// This method consumes set of inputs and outputs.
///
/// Format of inputs is specific for payment method. Usually it should reference payment transaction
/// with at least one output that corresponds to payment address that user owns.
///
/// #Params
/// command_handle: command handle to map callback to context
/// wallet_handle: wallet handle
/// submitter_did: (Optional) DID of request sender
/// inputs_json: The list of payment sources as json array:
///   ["source1", ...]
///   Note that each source should reference payment address
/// outputs_json: The list of outputs as json array:
///   [{
///     recipient: <str>, // payment address of recipient
///     amount: <int>, // amount
///   }]
/// extra: // optional information for payment operation
///
/// #Returns
/// payment_req_json - Indy request for doing payment
typedef indy_error_t (*indyBuildPaymentReqCB)(indy_handle_t command_handle,
                                              indy_handle_t wallet_handle,
                                              const char* submitter_did,
                                              const char* inputs_json,
                                              const char* outputs_json,
                                              const char* extra,
                                              indy_err_str_cb cb);

/// Parses response for Indy request for payment txn.
///
/// #Params
/// command_handle: command handle to map callback to context
/// resp_json: response for Indy request for payment txn
///
/// #Returns
/// receipts_json - parsed (payment method and node version agnostic) receipts info as json:
///   [{
///      receipt: <str>, // receipt that can be used for payment referencing and verification
///      recipient: <str>, //payment address for this receipt
///      amount: <int>, // amount
///      extra: <str>, // optional data from payment transaction
///   }]
typedef indy_error_t (*indyParsePaymentResponseCB)(indy_handle_t command_handle,
                                                   const char* resp_json,
                                                   indy_err_str_cb cb);

/// Builds Indy request for doing minting
/// according to this payment method.
///
/// #Params
/// command_handle: command handle to map callback to context
/// wallet_handle: wallet handle
/// submitter_did: (Optional) DID of request sender
/// outputs_json: The list of outputs as json array:
///   [{
///     recipient: <str>, // payment address of recipient
///     amount: <int>, // amount
///   }]
/// extra: // optional information for payment operation
///
/// #Returns
/// mint_req_json - Indy request for doing minting
typedef indy_error_t (*indyBuildMintReqCB)(indy_handle_t command_handle,
                                           indy_handle_t wallet_handle,
                                           const char* submitter_did,
                                           const char* outputs_json,
                                           const char* extra,
                                           indy_err_str_cb cb);

/// Builds Indy request for setting fees for transactions in the ledger
///
/// # Params
/// command_handle: command handle to map callback to context
/// wallet_handle: wallet handle
/// submitter_did: (Optional) DID of request sender
/// fees_json {
///   txnType1: amount1,
///   txnType2: amount2,
///   .................
///   txnTypeN: amountN,
/// }
///
/// # Return
/// set_txn_fees_json - Indy request for setting fees for transactions in the ledger
typedef indy_error_t (*indyBuildSetTxnFeesReqCB)(indy_handle_t command_handle,
                                                 indy_handle_t wallet_handle,
                                                 const char* submitter_did,
                                                 const char* fees_json,
                                                 indy_err_str_cb cb);

/// Builds Indy get request for getting fees for transactions in the ledger
///
/// # Params
/// command_handle: command handle to map callback to context
/// wallet_handle: wallet handle
/// submitter_did: (Optional) DID of request sender
///
/// # Return
/// get_txn_fees_json - Indy request for getting fees for transactions in the ledger
typedef indy_error_t (*indyBuildGetTxnFeesReqCB)(indy_handle_t command_handle,
                                                 indy_handle_t wallet_handle,
                                                 const char* submitter_did,
                                                 indy_err_str_cb cb);

/// Parses response for Indy request for getting fees
///
/// # Params
/// command_handle: command handle to map callback to context
/// resp_json: response for Indy request for getting fees
///
/// # Return
/// fees_json {
///   txnType1: amount1,
///   txnType2: amount2,
///   .................
///   txnTypeN: amountN,
/// }
typedef indy_error_t (*indyParseGetTxnFeesResponseCB)(indy_handle_t command_handle,
                                                      const char* resp_json,
                                                      indy_err_str_cb cb);

/// Builds Indy request for getting information to verify the payment receipt
///
/// # Params
/// command_handle: command handle to map callback to context
/// wallet_handle: wallet handle
/// submitter_did: (Optional) DID of request sender
/// receipt: payment receipt to verify
///
/// # Return
/// verify_txn_json -- request to be sent to ledger
typedef indy_error_t (*indyBuildVerifyPaymentReqCB)(indy_handle_t command_handle,
                                                    indy_handle_t wallet_handle,
                                                    const char* submitter_did,
                                                    const char* receipt,
                                                    indy_err_str_cb cb);

/// Parses Indy response with information to verify receipt
///
/// # Params
/// command_handle: command handle to map callback to context
/// resp_json: response for Indy request for information to verify the payment receipt
///
/// # Return
/// txn_json: {
///     sources: [<str>, ]
///     receipts: [ {
///         recipient: <str>, // payment address of recipient
///         receipt: <str>, // receipt that can be used for payment referencing and verification
///         amount: <int>, // amount
///     }, ]
///     extra: <str>, //optional data
/// }
typedef indy_error_t (*indyParseVerifyPaymentResponseCB)(indy_handle_t command_handle,
                                                         const char* resp_json,
                                                         indy_err_str_cb cb);

#ifdef __cplusplus
extern "C" {
#endif

    /// Registers custom wallet storage implementation.
    ///
    /// It allows library user to provide custom wallet implementation.
    ///
    /// #Params
    /// command_handle: Command handle to map callback to caller context.
    /// type_: Wallet type name.
    /// create: WalletType create operation handler
    /// open: WalletType open operation handler
    /// close: Wallet close operation handler
    /// delete: WalletType delete operation handler
    /// add_record: WalletType add record operation handler
    /// update_record_value: WalletType update record value operation handler
    /// update_record_tags: WalletType update record tags operation handler
    /// add_record_tags: WalletType add record tags operation handler
    /// delete_record_tags: WalletType delete record tags operation handler
    /// delete_record: WalletType delete record operation handler
    /// get_record: WalletType get record operation handler
    /// get_record_id: WalletType get record id operation handler
    /// get_record_type: WalletType get record type operation handler
    /// get_record_value: WalletType get record value operation handler
    /// get_record_tags: WalletType get record tags operation handler
    /// free_record: WalletType free record operation handler
    /// search_records: WalletType search records operation handler
    /// search_all_records: WalletType search all records operation handler
    /// get_search_total_count: WalletType get search total count operation handler
    /// fetch_search_next_record: WalletType fetch search next record operation handler
    /// free_search: WalletType free search operation handler
    /// free: Handler that allows to de-allocate strings allocated in caller code
    ///
    /// #Returns
    /// Error code
    extern indy_error_t indy_register_payment_method(indy_handle_t  command_handle,
                                                      const char*    payment_method,
                                                      indyCreatePaymentAddressCB create_payment_address_cb,
                                                      indyAddRequestFeesCB add_request_fees_cb,
                                                      indyParseResponseWithFeesCB parse_response_with_fees_cb,
                                                      indyBuildGetPaymentSourcesRequestCB build_get_payment_sources_request_cb,
                                                      indyParseGetPaymentSourcesResponseCB parse_get_payment_sources_response_cb,
                                                      indyBuildPaymentReqCB build_payment_req_cb,
                                                      indyParsePaymentResponseCB parse_payment_response_cb,
                                                      indyBuildMintReqCB build_mint_req_cb,
                                                      indyBuildSetTxnFeesReqCB build_set_txn_fees_req_cb,
                                                      indyBuildGetTxnFeesReqCB build_get_txn_fees_req_cb,
                                                      indyParseGetTxnFeesResponseCB parse_get_txn_fees_response_cb,
                                                      indyBuildVerifyPaymentReqCB build_verify_payment_req_cb,
                                                      indyParseVerifyPaymentResponseCB parse_verify_payment_response_cb,
                                                      indy_empty_cb cb
                                                      );

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
    /// payment_method: payment method to use (for example, 'sov')
    /// config: payment address config as json:
    ///   {
    ///     seed: <str>, // allows deterministic creation of payment address
    ///   }
    ///
    /// #Returns
    /// payment_address - public identifier of payment address in fully resolvable payment address format
    
    extern indy_error_t indy_create_payment_address(indy_handle_t command_handle,
                                                    indy_handle_t wallet_handle,
                                                    const char *  payment_method,
                                                    const char *  config,
                                                    indy_str_cb cb
                                                    );

    /// Lists all payment addresses that are stored in the wallet
    ///
    /// #Params
    /// command_handle: command handle to map callback to context
    /// wallet_handle: wallet to search for payment_addresses in
    ///
    /// #Returns
    /// payment_addresses_json - json array of string with json addresses

    extern indy_error_t indy_list_payment_addresses(indy_handle_t command_handle,
                                                    indy_handle_t wallet_handle,
                                                    indy_str_cb cb
                                                    );

    /// Modifies Indy request by adding information how to pay fees for this transaction
    /// according to this payment method.
    ///
    /// This method consumes set of inputs and outputs. The difference between inputs balance
    /// and outputs balance is the fee for this transaction.
    ///
    /// Not that this method also produces correct fee signatures.
    ///
    /// Format of inputs is specific for payment method. Usually it should reference payment transaction
    /// with at least one output that corresponds to payment address that user owns.
    ///
    /// #Params
    /// command_handle: Command handle to map callback to caller context.
    /// wallet_handle: wallet handle
    /// submitter_did: (Optional) DID of request sender
    /// req_json: initial transaction request as json
    /// inputs_json: The list of payment sources as json array:
    ///   ["source1", ...]
    ///     - each input should reference paymentAddress
    ///     - this param will be used to determine payment_method
    /// outputs_json: The list of outputs as json array:
    ///   [{
    ///     recipient: <str>, // payment address of recipient
    ///     amount: <int>, // amount
    ///   }]
    /// extra: // optional information for payment operation
    ///
    /// #Returns
    /// req_with_fees_json - modified Indy request with added fees info
    /// payment_method - used payment method

    extern indy_error_t indy_add_request_fees(indy_handle_t command_handle,
                                              indy_handle_t wallet_handle,
                                              const char *  submitter_did,
                                              const char *  req_json,
                                              const char *  inputs_json,
                                              const char *  outputs_json,
                                              const char *  extra,
                                              indy_str_str_cb cb
                                              );

    /// Parses response for Indy request with fees.
    ///
    /// #Params
    /// command_handle: Command handle to map callback to caller context.
    /// payment_method: payment method to use
    /// resp_json: response for Indy request with fees
    ///   Note: this param will be used to determine payment_method
    ///
    /// #Returns
    /// receipts_json - parsed (payment method and node version agnostic) receipts info as json:
    ///   [{
    ///      receipt: <str>, // receipt that can be used for payment referencing and verification
    ///      recipient: <str>, //payment address of recipient
    ///      amount: <int>, // amount
    ///      extra: <str>, // optional data from payment transaction
    ///   }]

    extern indy_error_t indy_parse_response_with_fees(indy_handle_t command_handle,
                                                        const char *  payment_method,
                                                        const char *  resp_json,
                                                        indy_str_cb cb
                                                     );

    /// Builds Indy request for getting sources list for payment address
    /// according to this payment method.
    ///
    /// #Params
    /// command_handle: Command handle to map callback to caller context.
    /// wallet_handle: wallet handle
    /// submitter_did: (Optional) DID of request sender
    /// payment_address: target payment address
    ///
    /// #Returns
    /// get_sources_txn_json - Indy request for getting sources list for payment address
    /// payment_method - used payment method

    extern indy_error_t indy_build_get_payment_sources_request(indy_handle_t command_handle,
                                                               indy_handle_t wallet_handle,
                                                               const char *  submitter_did,
                                                               const char *  payment_address,
                                                               indy_str_str_cb cb
                                                               );

    /// Parses response for Indy request for getting sources list.
    ///
    /// #Params
    /// command_handle: Command handle to map callback to caller context.
    /// payment_method: payment method to use.
    /// resp_json: response for Indy request for getting sources list
    ///   Note: this param will be used to determine payment_method
    ///
    /// #Returns
    /// sources_json - parsed (payment method and node version agnostic) sources info as json:
    ///   [{
    ///      source: <str>, // source input
    ///      paymentAddress: <str>, //payment address for this source
    ///      amount: <int>, // amount
    ///      extra: <str>, // optional data from payment transaction
    ///   }]

    extern indy_error_t indy_parse_get_payment_sources_response(indy_handle_t command_handle,
                                                                const char *  payment_method,
                                                                const char *  resp_json,
                                                                indy_str_cb cb
                                                                );

    /// Builds Indy request for doing payment
    /// according to this payment method.
    ///
    /// This method consumes set of inputs and outputs.
    ///
    /// Format of inputs is specific for payment method. Usually it should reference payment transaction
    /// with at least one output that corresponds to payment address that user owns.
    ///
    /// #Params
    /// command_handle: Command handle to map callback to caller context.
    /// wallet_handle: wallet handle
    /// submitter_did: (Optional) DID of request sender
    /// inputs_json: The list of payment sources as json array:
    ///   ["source1", ...]
    ///   Note that each source should reference payment address
    /// outputs_json: The list of outputs as json array:
    ///   [{
    ///     recipient: <str>, // payment address of recipient
    ///     amount: <int>, // amount
    ///   }]
    /// extra: // optional information for payment operation
    ///
    /// #Returns
    /// payment_req_json - Indy request for doing payment
    /// payment_method - used payment method

    extern indy_error_t indy_build_payment_req(indy_handle_t command_handle,
                                               indy_handle_t wallet_handle,
                                               const char *  submitter_did,
                                               const char *  inputs_json,
                                               const char *  outputs_json,
                                               const char *  extra,
                                               indy_str_str_cb cb
                                               );

    /// Parses response for Indy request for payment txn.
    ///
    /// #Params
    /// command_handle: Command handle to map callback to caller context.
    /// payment_method: payment method to use
    /// resp_json: response for Indy request for payment txn
    ///   Note: this param will be used to determine payment_method
    ///
    /// #Returns
    /// receipts_json - parsed (payment method and node version agnostic) receipts info as json:
    ///   [{
    ///      receipt: <str>, // receipt that can be used for payment referencing and verification
    ///      recipient: <str>, // payment address of recipient
    ///      amount: <int>, // amount
    ///      extra: <str>, // optional data from payment transaction
    ///   }]

    extern indy_error_t indy_parse_payment_response(indy_handle_t command_handle,
                                                    const char *  payment_method,
                                                    const char *  resp_json,
                                                    indy_str_cb cb
                                                    );

    /// Builds Indy request for doing minting
    /// according to this payment method.
    ///
    /// #Params
    /// command_handle: Command handle to map callback to caller context.
    /// wallet_handle: wallet handle
    /// submitter_did: (Optional) DID of request sender
    /// outputs_json: The list of outputs as json array:
    ///   [{
    ///     recipient: <str>, // payment address of recipient
    ///     amount: <int>, // amount
    ///   }]
    /// extra: // optional information for payment operation
    ///
    /// #Returns
    /// mint_req_json - Indy request for doing minting
    /// payment_method - used payment method

    extern indy_error_t indy_build_mint_req(indy_handle_t command_handle,
                                            indy_handle_t wallet_handle,
                                            const char *  submitter_did,
                                            const char *  outputs_json,
                                            const char *  extra,
                                            indy_str_str_cb cb
                                            );

    /// Builds Indy request for setting fees for transactions in the ledger
    ///
    /// # Params
    /// command_handle: Command handle to map callback to caller context.
    /// wallet_handle: wallet handle
    /// submitter_did: (Optional) DID of request sender
    /// payment_method: payment method to use
    /// fees_json {
    ///   txnType1: amount1,
    ///   txnType2: amount2,
    ///   .................
    ///   txnTypeN: amountN,
    /// }
    /// # Return
    /// set_txn_fees_json - Indy request for setting fees for transactions in the ledger

    extern indy_error_t indy_build_set_txn_fees_req(indy_handle_t command_handle,
                                                    indy_handle_t wallet_handle,
                                                    const char *  submitter_did,
                                                    const char *  payment_method,
                                                    const char *  fees_json,
                                                    indy_str_cb cb
                                                    );

    /// Builds Indy get request for getting fees for transactions in the ledger
    ///
    /// # Params
    /// command_handle: Command handle to map callback to caller context.
    /// wallet_handle: wallet handle
    /// submitter_did: (Optional) DID of request sender
    /// payment_method: payment method to use
    ///
    /// # Return
    /// get_txn_fees_json - Indy request for getting fees for transactions in the ledger

    extern indy_error_t indy_build_get_txn_fees_req(indy_handle_t command_handle,
                                                    indy_handle_t wallet_handle,
                                                    const char *  submitter_did,
                                                    const char *  payment_method,
                                                    indy_str_cb cb
                                                    );

    /// Parses response for Indy request for getting fees
    ///
    /// # Params
    /// command_handle: Command handle to map callback to caller context.
    /// payment_method: payment method to use
    /// resp_json: response for Indy request for getting fees
    ///
    /// # Return
    /// fees_json {
    ///   txnType1: amount1,
    ///   txnType2: amount2,
    ///   .................
    ///   txnTypeN: amountN,
    /// }

    extern indy_error_t indy_parse_get_txn_fees_response(indy_handle_t command_handle,
                                                         const char *  payment_method,
                                                         const char *  resp_json,
                                                         indy_str_cb cb
                                                         );

    /// Builds Indy request for information to verify the payment receipt
    ///
    /// # Params
    /// command_handle: Command handle to map callback to caller context.
    /// wallet_handle: wallet handle
    /// submitter_did: (Optional) DID of request sender
    /// receipt: payment receipt to verify
    ///
    /// # Return
    /// verify_txn_json: Indy request for verification receipt
    /// payment_method: used payment method

    extern indy_error_t indy_build_verify_payment_req(indy_handle_t command_handle,
                                                      indy_handle_t wallet_handle,
                                                      const char *  submitter_did,
                                                      const char *  receipt,
                                                      indy_str_str_cb cb
                                                      );

    /// Parses Indy response with information to verify receipt
    ///
    /// # Params
    /// command_handle: Command handle to map callback to caller context.
    /// payment_method: payment method to use
    /// resp_json: response of the ledger for verify txn
    ///
    /// # Return
    /// txn_json: {
    ///     sources: [<str>, ]
    ///     receipts: [ {
    ///         recipient: <str>, // payment address of recipient
    ///         receipt: <str>, // receipt that can be used for payment referencing and verification
    ///         amount: <int>, // amount
    ///     } ],
    ///     extra: <str>, //optional data
    /// }

    extern indy_error_t indy_parse_verify_payment_response(indy_handle_t command_handle,
                                                           const char *  payment_method,
                                                           const char *  resp_json,
                                                           indy_str_cb cb
                                                           );

#ifdef __cplusplus
}
#endif

#endif
