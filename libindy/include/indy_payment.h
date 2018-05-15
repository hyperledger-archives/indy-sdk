#ifndef __indy__payment__included__
#define __indy__payment__included__

#include "indy_mod.h"
#include "indy_types.h"

#ifdef __cplusplus
extern "C" {
#endif

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

    extern indy_error_t indy_register_payment_method(indy_handle_t  command_handle,
                                                     const char*    payment_method,
                                                     indy_error_t (*createPaymentAddress)(const char* config,
                                                                                          const char ** const payment_address_ptr),
                                                     indy_error_t (*addRequestFees)(const char* req_json,
                                                                                    const char* inputs_json,
                                                                                    const char* outputs_json,
                                                                                    const char ** const req_with_fees_ptr),
                                                     indy_error_t (*parseResponseWithFees)(const char* resp_json,
                                                                                           const char ** const utxo_ptr),
                                                     indy_error_t (*buildGetUtxoRequest)(const char* payment_address,
                                                                                         const char ** const get_utxo_txn_ptr),
                                                     indy_error_t (*parseGetUtxoResponse)(const char* resp_json,
                                                                                          const char ** const utxo_ptr),
                                                     indy_error_t (*buildPaymentReq)(const char* inputs_json,
                                                                                     const char* outputs_json,
                                                                                     const char ** const payment_req_ptr),
                                                     indy_error_t (*parsePaymentResponse)(const char* resp_json,
                                                                                          const char ** const utxo_ptr),
                                                     indy_error_t (*buildMintReq)(const char* outputs_json,
                                                                                  const char ** const mint_req_ptr),
                                                     indy_error_t (*buildSetTxnFeesReq)(const char* fees_json,
                                                                                        const char ** const set_txn_fees_ptr),
                                                     indy_error_t (*buildGetTxnFeesReq)(const char ** const get_txn_fees_ptr),
                                                     indy_error_t (*parseGetTxnFeesResponse)(const char* resp_json,
                                                                                             const char ** const fees_ptr),
                                                     void           (*fn)(indy_handle_t xcommand_handle, indy_error_t err)
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
    /// payment_method: Payment method to use (for example, 'sov')
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

                                                    void           (*cb)(indy_handle_t xcommand_handle,
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

                                                    void           (*cb)(indy_handle_t xcommand_handle,
                                                                         indy_error_t  err,
                                                                         const char*   payment_addresses_json)
                                                    );

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

    extern indy_error_t indy_add_request_fees(indy_handle_t command_handle,
                                              indy_handle_t wallet_handle,
                                              const char *  submitter_did,
                                              const char *  req_json,
                                              const char *  inputs_json,
                                              const char *  outputs_json,

                                              void           (*cb)(indy_handle_t xcommand_handle,
                                                                   indy_error_t  err,
                                                                   const char*   req_with_fees_json,
                                                                   const char*   payment_method)
                                              );

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
    ///      input: <str>, // UTXO input
    ///      amount: <int>, // amount of tokens in this input
    ///      extra: <str>, // optional data from payment transaction
    ///   }]

    extern indy_error_t indy_parse_response_with_fees(indy_handle_t command_handle,
                                                        const char *  payment_method,
                                                        const char *  resp_json,

                                                        void           (*cb)(indy_handle_t xcommand_handle,
                                                                indy_error_t  err,
                                                                const char*   utxo_json)
                                                );

    /// Builds Indy request for getting UTXO list for payment address
    /// according to this payment method.
    ///
    /// #Params
    /// wallet_handle: wallet handle where keys for signature are stored
    /// submitter_did: target payment address
    /// payment_address: target payment address
    ///
    /// #Returns
    /// get_utxo_txn_json - Indy request for getting UTXO list for payment address
    /// payment_method

    extern indy_error_t indy_build_get_utxo_request(indy_handle_t command_handle,
                                                    indy_handle_t wallet_handle,
                                                    const char *  submitter_did,
                                                    const char *  payment_address,

                                                    void           (*cb)(indy_handle_t xcommand_handle,
                                                                         indy_error_t  err,
                                                                         const char*   get_utxo_txn_json,
                                                                         const char*   payment_method)
                                                    );

    /// Parses response for Indy request for getting UTXO list.
    ///
    /// #Params
    /// resp_json: response for Indy request for getting UTXO list
    ///   Note: this param will be used to determine payment_method
    ///
    /// #Returns
    /// utxo_json - parsed (payment method and node version agnostic) utxo info as json:
    ///   [{
    ///      input: <str>, // UTXO input
    ///      amount: <int>, // amount of tokens in this input
    ///      extra: <str>, // optional data from payment transaction
    ///   }]

    extern indy_error_t indy_parse_get_utxo_response(indy_handle_t command_handle,
                                                     const char *  payment_method,
                                                     const char *  resp_json,

                                                     void           (*cb)(indy_handle_t xcommand_handle,
                                                                          indy_error_t  err,
                                                                          const char*   utxo_json)
                                                     );

    /// Builds Indy request for doing tokens payment
    /// according to this payment method.
    ///
    /// This method consumes set of UTXO inputs and outputs.
    ///
    /// Format of inputs is specific for payment method. Usually it should reference payment transaction
    /// with at least one output that corresponds to payment address that user owns.
    ///
    /// #Params
    /// wallet_handle: wallet handle where keys for signature are stored
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

    extern indy_error_t indy_build_payment_req(indy_handle_t command_handle,
                                               indy_handle_t wallet_handle,
                                               const char *  submitter_did,
                                               const char *  inputs_json,
                                               const char *  outputs_json,

                                               void           (*cb)(indy_handle_t xcommand_handle,
                                                                    indy_error_t  err,
                                                                    const char*   payment_req_json,
                                                                    const char*   payment_method)
                                               );

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
    ///      input: <str>, // UTXO input
    ///      amount: <int>, // amount of tokens in this input
    ///      extra: <str>, // optional data from payment transaction
    ///   }]

    extern indy_error_t indy_parse_payment_response(indy_handle_t command_handle,
                                                    const char *  payment_method,
                                                    const char *  resp_json,

                                                    void           (*cb)(indy_handle_t xcommand_handle,
                                                                         indy_error_t  err,
                                                                         const char*   utxo_json)
                                                    );

    /// Builds Indy request for doing tokens minting
    /// according to this payment method.
    ///
    /// #Params
    /// wallet_handle: wallet handle where keys for signature are stored
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

    extern indy_error_t indy_build_mint_req(indy_handle_t command_handle,
                                            indy_handle_t wallet_handle,
                                            const char *  submitter_did,
                                            const char *  outputs_json,

                                            void           (*cb)(indy_handle_t xcommand_handle,
                                                                 indy_error_t  err,
                                                                 const char*   mint_req_json,
                                                                 const char*   payment_method)
                                            );

    /// Builds Indy request for setting fees for transactions in the ledger
    ///
    /// # Params
    /// command_handle
    /// wallet_handle: wallet handle where keys for signature are stored
    /// payment_method
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

                                                    void           (*cb)(indy_handle_t xcommand_handle,
                                                                         indy_error_t  err,
                                                                         const char*   set_txn_fees_json)
                                                    );

    /// Builds Indy get request for getting fees for transactions in the ledger
    ///
    /// # Params
    /// command_handle
    /// wallet_handle: wallet handle where keys for signature are stored
    /// payment_method
    ///
    /// # Return
    /// get_txn_fees_json - Indy request for getting fees for transactions in the ledger

    extern indy_error_t indy_build_get_txn_fees_req(indy_handle_t command_handle,
                                                    indy_handle_t wallet_handle,
                                                    const char *  submitter_did,
                                                    const char *  payment_method,

                                                    void           (*cb)(indy_handle_t xcommand_handle,
                                                                         indy_error_t  err,
                                                                         const char*   get_txn_fees_json)
                                                    );

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

    extern indy_error_t indy_parse_get_txn_fees_response(indy_handle_t command_handle,
                                                         const char *  payment_method,
                                                         const char *  resp_json,

                                                         void           (*cb)(indy_handle_t xcommand_handle,
                                                                              indy_error_t  err,
                                                                              const char*   fees_json)
                                                         );

#ifdef __cplusplus
}
#endif

#endif
