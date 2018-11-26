#ifndef __indy__pairwise__included__
#define __indy__pairwise__included__

#include "indy_types.h"
#include "indy_mod.h"

#ifdef __cplusplus
extern "C" {
#endif

    /// Check if pairwise is exists.
    ///
    /// #Params
    /// wallet_handle: wallet handler (created by open_wallet).
    /// command_handle: command handle to map callback to user context.
    /// their_did: encrypted DID
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// exists: true - if pairwise is exists, false - otherwise
    ///
    /// #Errors
    /// Common*
    /// Wallet*

    extern indy_error_t indy_is_pairwise_exists(indy_handle_t command_handle,
                                                indy_handle_t wallet_handle,
                                                const char *  their_did,

                                                void          (*cb)(indy_handle_t  command_handle_,
                                                                    indy_error_t  err,
                                                                    indy_bool_t   exists)
                                               );


    /// Creates pairwise.
    ///
    /// #Params
    /// wallet_handle: wallet handler (created by open_wallet).
    /// command_handle: command handle to map callback to user context.
    /// their_did: encrypting DID
    /// my_did: encrypted DID
    /// metadata Optional: extra information for pairwise
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Error code
    ///
    /// #Errors
    /// Common*
    /// Wallet*

    extern indy_error_t indy_create_pairwise(indy_handle_t command_handle,
                                             indy_handle_t wallet_handle,
                                             const char *  their_did,
                                             const char *  my_did,
                                             const char *  metadata,

                                             void          (*cb)(indy_handle_t  command_handle_,
                                                                 indy_error_t   err)
                                            );


    /// Get list of saved pairwise.
    ///
    /// #Params
    /// wallet_handle: wallet handler (created by open_wallet).
    /// command_handle: command handle to map callback to user context.
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// list_pairwise: list of saved pairwise
    ///
    /// #Errors
    /// Common*
    /// Wallet*

    extern indy_error_t indy_list_pairwise(indy_handle_t command_handle,
                                           indy_handle_t wallet_handle,

                                           void          (*cb)(indy_handle_t  command_handle_,
                                                               indy_error_t   err,
                                                               const char*    list_pairwise)
                                          );


    /// Gets pairwise information for specific their_did.
    ///
    /// #Params
    /// wallet_handle: wallet handler (created by open_wallet).
    /// command_handle: command handle to map callback to user context.
    /// their_did: encoded Did
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// pairwise_info_json: did info associated with their did
    ///
    /// #Errors
    /// Common*
    /// Wallet*

    extern indy_error_t indy_get_pairwise(indy_handle_t command_handle,
                                          indy_handle_t wallet_handle,
                                          const char *  their_did,

                                          void          (*cb)(indy_handle_t  command_handle_,
                                                              indy_error_t   err,
                                                              const char*    pairwise_info_json)
                                         );


    /// Save some data in the Wallet for pairwise associated with Did.
    ///
    /// #Params
    /// wallet_handle: wallet handler (created by open_wallet).
    /// command_handle: command handle to map callback to user context.
    /// their_did: encoded Did
    /// metadata: some extra information for pairwise
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Error code
    ///
    /// #Errors
    /// Common*
    /// Wallet*

    extern indy_error_t indy_set_pairwise_metadata(indy_handle_t command_handle,
                                                   indy_handle_t wallet_handle,
                                                   const char *  their_did,
                                                   const char *  metadata,

                                                   void          (*cb)(indy_handle_t  command_handle_,
                                                                       indy_error_t   err)
                                                  );


#ifdef __cplusplus
}
#endif

#endif
