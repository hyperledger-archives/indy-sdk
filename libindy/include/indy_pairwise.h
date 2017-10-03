#ifndef __indy__signus__included__
#define __indy__signus__included__

#ifdef __cplusplus
extern "C" {
#endif

    /// Check if pairwise is exists.
    ///
    /// #Params
    /// wallet_handle: wallet handler (created by open_wallet).
    /// command_handle: command handle to map callback to user context.
    /// their_did: encrypting DID
    /// my_did: encrypting DID
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// exists: true - if pairwise is exists, false - otherwise
    ///
    /// #Errors
    /// Common*
    /// Wallet*
    /// Crypto*

    extern indy_error_t indy_is_pairwise_exists(indy_handle_t command_handle,
                                                indy_handle_t wallet_handle,
                                                const char *  their_did,

                                                void          (*cb)(indy_handle_t  xcommand_handle,
                                                                    indy_error_t  err,
                                                                     indy_bool_t   exists)
                                               );


    /// Creates pairwise.
    ///
    /// #Params
    /// wallet_handle: wallet handler (created by open_wallet).
    /// command_handle: command handle to map callback to user context.
    /// their_did: encrypting DID
    /// my_did: encrypting DID
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Errors
    /// Common*
    /// Wallet*
    /// Crypto*

    extern indy_error_t indy_create_pairwise(indy_handle_t command_handle,
                                             indy_handle_t wallet_handle,
                                             const char *  their_did,
                                             const char *  my_did,

                                             void          (*cb)(indy_handle_t  xcommand_handle,
                                                                 indy_error_t   err)
                                            );


    /// Get list of saved pairs.
    ///
    /// #Params
    /// wallet_handle: wallet handler (created by open_wallet).
    /// command_handle: command handle to map callback to user context.
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// pairwise_list: list of saved pairs
    ///
    /// #Errors
    /// Common*
    /// Wallet*
    /// Crypto*

    extern indy_error_t indy_list_pairwise(indy_handle_t command_handle,
                                           indy_handle_t wallet_handle,

                                           void          (*cb)(indy_handle_t  xcommand_handle,
                                                               indy_error_t   err,
                                                               const char*    pairwise_list)
                                          );


    /// Gets my did for specific their did.
    ///
    /// #Params
    /// wallet_handle: wallet handler (created by open_wallet).
    /// command_handle: command handle to map callback to user context.
    /// their_did: encoded Did
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// my_did: did associated with their did
    ///
    /// #Errors
    /// Common*
    /// Wallet*
    /// Crypto*

    extern indy_error_t indy_pairwise_get_my_did(indy_handle_t command_handle,
                                                 indy_handle_t wallet_handle,
                                                 const char *  their_did,

                                                 void          (*cb)(indy_handle_t  xcommand_handle,
                                                                     indy_error_t   err,
                                                                     const char*    my_did)
                                                );


    /// Save some data in the Wallet for a given DID .
    ///
    /// #Params
    /// wallet_handle: wallet handler (created by open_wallet).
    /// command_handle: command handle to map callback to user context.
    /// their_did: encoded Did
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Errors
    /// Common*
    /// Wallet*
    /// Crypto*

    extern indy_error_t indy_set_pairwise_metadata(indy_handle_t command_handle,
                                                   indy_handle_t wallet_handle,
                                                   const char *  their_did,
                                                   const char *  metadata,

                                                   void          (*cb)(indy_handle_t  xcommand_handle,
                                                                       indy_error_t   err)
                                                  );


    /// Get some metadata from the Wallet for a given DID.
    ///
    /// #Params
    /// wallet_handle: wallet handler (created by open_wallet).
    /// command_handle: command handle to map callback to user context.
    /// their_did: encoded Did
    /// cb: Callback that takes command result as parameter.
    ///
    ///
    /// #Returns
    /// metadata
    ///
    /// #Errors
    /// Common*
    /// Wallet*
    /// Crypto*

    extern indy_error_t indy_get_pairwise_metadata(indy_handle_t command_handle,
                                                   indy_handle_t wallet_handle,
                                                   const char *  their_did

                                                   void          (*cb)(indy_handle_t  xcommand_handle,
                                                                       indy_error_t   err,
                                                                       const char*    metadata)
                                                  );

#ifdef __cplusplus
}
#endif

#endif
