#ifndef __indy__payment__included__
#define __indy__payment__included__

#include "indy_mod.h"
#include "indy_types.h"

#ifdef __cplusplus
extern "C" {
#endif

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

                                                    void           (*cb)(indy_handle_t command_handle_,
                                                                         indy_error_t  err,
                                                                         const char*   payment_address)
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

                                                    void           (*cb)(indy_handle_t command_handle_,
                                                                         indy_error_t  err,
                                                                         const char*   payment_addresses_json)
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

                                              void           (*cb)(indy_handle_t command_handle_,
                                                                   indy_error_t  err,
                                                                   const char*   req_with_fees_json,
                                                                   const char*   payment_method)
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

                                                        void           (*cb)(indy_handle_t command_handle_,
                                                                indy_error_t  err,
                                                                const char*   receipts_json)
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

                                                               void           (*cb)(indy_handle_t command_handle_,
                                                                                    indy_error_t  err,
                                                                                    const char*   get_sources_txn_json,
                                                                                    const char*   payment_method)
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

                                                                void           (*cb)(indy_handle_t command_handle_,
                                                                                     indy_error_t  err,
                                                                                     const char*   sources_json)
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

                                               void           (*cb)(indy_handle_t command_handle_,
                                                                    indy_error_t  err,
                                                                    const char*   payment_req_json,
                                                                    const char*   payment_method)
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

                                                    void           (*cb)(indy_handle_t command_handle_,
                                                                         indy_error_t  err,
                                                                         const char*   receipts_json)
                                                    );

    /// Append payment extra JSON with TAA acceptance data
    ///
    /// EXPERIMENTAL
    ///
    /// This function may calculate digest by itself or consume it as a parameter.
    /// If all text, version and taa_digest parameters are specified, a check integrity of them will be done.
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context.
    /// extra_json: (optional) original extra json.
    /// text and version - (optional) raw data about TAA from ledger.
    ///     These parameters should be passed together.
    ///     These parameters are required if taa_digest parameter is omitted.
    /// taa_digest - (optional) digest on text and version. This parameter is required if text and version parameters are omitted.
    /// mechanism - mechanism how user has accepted the TAA
    /// time - UTC timestamp when user has accepted the TAA
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Updated request result as json.
    ///
    /// #Errors
    /// Common*

    extern indy_error_t indy_prepare_payment_extra_with_acceptance_data(indy_handle_t command_handle,
                                                                        const char *  extra_json,
                                                                        const char *  text,
                                                                        const char *  version,
                                                                        const char *  taa_digest,
                                                                        const char *  mechanism,
                                                                        indy_u64_t  time,

                                                                        void           (*cb)(indy_handle_t command_handle_,
                                                                                             indy_error_t  err,
                                                                                             const char*   extra_with_acceptance)
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

                                            void           (*cb)(indy_handle_t command_handle_,
                                                                 indy_error_t  err,
                                                                 const char*   mint_req_json,
                                                                 const char*   payment_method)
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

                                                    void           (*cb)(indy_handle_t command_handle_,
                                                                         indy_error_t  err,
                                                                         const char*   set_txn_fees_json)
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

                                                    void           (*cb)(indy_handle_t command_handle_,
                                                                         indy_error_t  err,
                                                                         const char*   get_txn_fees_json)
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

                                                         void           (*cb)(indy_handle_t command_handle_,
                                                                              indy_error_t  err,
                                                                              const char*   fees_json)
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

                                                      void           (*cb)(indy_handle_t command_handle_,
                                                                           indy_error_t  err,
                                                                           const char*   verify_txn_json,
                                                                           const char*   payment_method)
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

                                                           void           (*cb)(indy_handle_t command_handle_,
                                                                                indy_error_t  err,
                                                                                const char*   txn_json)
                                                           );

#ifdef __cplusplus
}
#endif

#endif
