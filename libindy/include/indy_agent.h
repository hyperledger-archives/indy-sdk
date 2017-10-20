#ifndef __indy_agent__included__
#define __indy_agent__included__

#ifdef __cplusplus
extern "C" {
#endif

extern indy_error_t indy_prep_msg(indy_handle_t          command_handle,
                                  indy_handle_t          wallet_handle,
                                  const char *const      sender_vk,
                                  const char *const      recipient_vk,
                                  const indy_u8_t *const msg_data,
                                  indy_u32_t             msg_len,

                                  void                   (cb*)(indy_handle_t          command_handle,
                                                               indy_error_t           err,
                                                               const indy_u8_t *const encrypted_msg
                                                               indy_u32_t             encrypted_len)
                                 );

extern indy_error_t indy_prep_anonymous_msg(indy_handle_t          command_handle,
                                            const char *const      recipient_vk,
                                            const indy_u8_t *const msg_data,
                                            indy_u32_t             msg_len,

                                            void                   (cb*)(indy_handle_t          command_handle,
                                                                         indy_error_t           err,
                                                                         const indy_u8_t *const encrypted_msg
                                                                         indy_u32_t             encrypted_len)
                                 );

extern indy_error_t indy_parse_msg(indy_handle_t          command_handle,
                                   const char *const      recipient_vk,
                                   const indy_u8_t *const encrypted_msg,
                                   indy_u32_t             encrypted_len,

                                   void                   (cb*)(indy_handle_t          command_handle,
                                                                indy_error_t           err,
                                                                const char *const      sender_vk,
                                                                const indy_u8_t *const msg_data,
                                                                indy_u32_t             msg_len)
                                  );

/// Establishes agent to agent connection.
///
/// Information about sender Identity must be saved in the wallet with indy_create_and_store_my_did
/// call before establishing of connection.
///
/// Information about receiver Identity can be saved in the wallet with indy_store_their_did
/// call before establishing of connection. If there is no corresponded wallet record for receiver Identity
/// than this call will lookup Identity Ledger and cache this information in the wallet.
///
/// Note that messages encryption/decryption will be performed automatically.
///
/// #Params
/// command_handle: Command handle to map callback to caller context.
/// pool_handle: Pool handle (created by open_pool_ledger).
/// wallet_handle: Wallet handle (created by open_wallet).
/// sender_did: Id of sender Identity stored in secured Wallet.
/// receiver_did: Id of receiver Identity.
/// connection_cb: Callback that will be called after establishing of connection or on error.
///     Will be called exactly once with result of connect operation.
/// message_cb: Callback that will be called on receiving of an incoming message.
///     Can be called multiply times: once for each incoming message.
///
/// #Returns
/// Error code
/// connection_cb:
/// - xcommand_handle: command handle to map callback to caller context.
/// - err: Error code.
/// - connection_handle: Connection handle to use for messages sending and mapping of incomming messages to this connection.
/// message_cb:
/// - xconnection_handle: Connection handle. Indetnifies connection.
/// - err: Error code.
/// - message: Received message.

extern indy_error_t indy_agent_connect(indy_handle_t command_handle,
                                       indy_handle_t pool_handle,
                                       indy_handle_t wallet_handle,
                                       const char *  sender_did,
                                       const char *  receiver_did,

                                       void  (*connection_cb)(indy_handle_t xcommand_handle,
                                                              indy_error_t  err,
                                                              indy_handle_t connection_handle),

                                       void     (*message_cb)(indy_handle_t xconnection_handle,
                                                              indy_error_t  err,
                                                              const char *  message)
                                       );

/// Starts listening of agent connections.
///
/// Listener will accept only connections to registered DIDs by indy_agent_add_identity call.
///
/// Information about sender Identity for incomming connection validation can be saved in the wallet
/// with indy_store_their_did call before establishing of connection. If there is no corresponded
/// wallet record for sender Identity than listener will lookup Identity Ledger and cache this
/// information in the wallet.
///
/// Note that messages encryption/decryption will be performed automatically.
///
/// #Params
/// command_handle: command handle to map callback to caller context.
/// endpoint: endpoint to use in starting listener.
/// listener_cb: Callback that will be called after listening started or on error.
///     Will be called exactly once with result of start listen operation.
/// connection_cb: Callback that will be called after establishing of incoming connection.
///     Can be called multiply times: once for each incoming connection.
/// message_cb: Callback that will be called on receiving of an incoming message.
///     Can be called multiply times: once for each incoming message.
///
/// #Returns
/// Error code
/// listener_cb:
/// - xcommand_handle: command handle to map callback to caller context.
/// - err: Error code
/// - listener_handle: Listener handle to use for mapping of incomming connections to this listener.
/// connection_cb:
/// - xlistener_handle: Listener handle. Identifies listener.
/// - err: Error code
/// - connection_handle: Connection handle to use for messages sending and mapping of incomming messages to to this connection.
/// - sender_did: Id of sender Identity stored in secured Wallet.
/// - receiver_did: Id of receiver Identity.
/// message_cb:
/// - xconnection_handle: Connection handle. Indetnifies connection.
/// - err: Error code.
/// - message: Received message.

extern indy_error_t indy_agent_listen(indy_handle_t command_handle,
                                      const char *  endpoint,

                                      void     (*listener_cb)(indy_handle_t xcommand_handle,
                                                              indy_error_t  err,
                                                              indy_handle_t listener_handle),

                                      void   (*connection_cb)(indy_handle_t xlistener_handle,
                                                              indy_error_t  err,
                                                              indy_handle_t connection_handle,
                                                              const char *  sender_did,
                                                              const char *  receiver_did),

                                      void      (*message_cb)(indy_handle_t xconnection_handle,
                                                              indy_error_t  err,
                                                              const char *  message)
                                      );

/// Add identity to listener.
///
/// Performs wallet lookup to find corresponded receiver Identity information.
/// Information about receiver Identity must be saved in the wallet with
/// indy_create_and_store_my_did call before this call.
///
/// After successfully add_identity listener will start to accept incoming connection to added DID.
///
///
/// #Params
/// command_handle: command handle to map callback to caller context.
/// listener_handle: listener handle (created by indy_agent_listen).
/// pool_handle: pool handle (created by open_pool_ledger).
/// wallet_handle: wallet handle (created by open_wallet).
/// did: DID of identity.
///
/// add_identity_cb: Callback that will be called after identity added or on error.
///     Will be called exactly once with result of start listen operation.
///
/// #Returns
/// Error code
/// add_identity_cb:
/// - xcommand_handle: command handle to map callback to caller context.
/// - err: Error code

extern indy_error_t indy_agent_add_identity(indy_handle_t command_handle,
                                            indy_handle_t listener_handle,
                                            indy_handle_t pool_handle,
                                            indy_handle_t wallet_handle,
                                            const char *  did,

                                            void (*add_identity_cb)(indy_handle_t xcommand_handle,
                                                                    indy_error_t  err)
                                            );

/// Remove identity from listener.
///
/// Performs wallet lookup to find corresponded receiver Identity information.
/// Information about receiver Identity must be saved in the wallet with
/// indy_create_and_store_my_did call before this call.
///
/// After successfully rm_identity listener will stop to accept incoming connection to removed DID.
///
///
/// #Params
/// command_handle: command handle to map callback to caller context.
/// listener_handle: listener handle (created by indy_agent_listen).
/// wallet_handle: wallet handle (created by open_wallet).
/// did: DID of identity.
///
/// rm_identity_cb: Callback that will be called after identity removed or on error.
///     Will be called exactly once with result of start listen operation.
///
/// #Returns
/// Error code
/// rm_identity_cb:
/// - xcommand_handle: command handle to map callback to caller context.
/// - err: Error code

extern indy_error_t indy_agent_remove_identity(indy_handle_t command_handle,
                                               indy_handle_t listener_handle,
                                               indy_handle_t wallet_handle,
                                               const char *  did,

                                               void (*rm_identity_cb)(indy_handle_t xcommand_handle,
                                                                      indy_error_t  err)
                                               );

/// Sends message to connected agent.
///
/// Note that this call works for both incoming and outgoing connections.
/// Note that messages encryption/decryption will be performed automatically.
///
/// #Params
/// command_handle: command handle to map callback to caller context.
/// connection_handle: Connection handle returned by indy_agent_connect or indy_agent_listen calls.
/// message: Message to send.
/// cb: Callback that will be called after message sent or on error. Will be called exactly once.
///
/// #Returns
/// err: Error code
/// cb:
/// - xcommand_handle: Command handle to map callback to caller context.
/// - err: Error code
///
/// #Errors

extern indy_error_t indy_agent_send(indy_handle_t command_handle,
                                    indy_handle_t connection_handle,
                                    const char *  message,

                                    void     (*cb)(indy_handle_t xcommand_handle,
                                                   indy_error_t  err)
                                   );

/// Closes agent connection.
///
/// Note that this call works for both incoming and outgoing connections.
///
/// #Params
/// command_handle: command handle to map callback to caller context.
/// connection_handle: Connection handle returned by indy_agent_connect or indy_agent_listen calls.
/// cb: Callback that will be called after connection closed or on error. Will be called exactly once.
///
/// #Returns
/// Error code
/// cb:
/// - xcommand_handle: command handle to map callback to caller context.
/// - err: Error code
///
/// #Errors

extern indy_error_t indy_agent_close_connection(indy_handle_t command_handle,
                                                indy_handle_t connection_handle,

                                                void     (*cb)(indy_handle_t xcommand_handle,
                                                               indy_error_t  err)
                                                );


/// Closes listener and stops listening for agent connections.
///
/// Note that all opened incomming connections will be closed automatically.
///
/// #Params
/// command_handle: command handle to map callback to caller context.
/// listener_handle: Listener handle returned by indy_agent_listen call.
/// cb: Callback that will be called after listener closed or on error. Will be called exactly once.
///
/// #Returns
/// Error code
/// cb:
/// - xcommand_handle: command handle to map callback to caller context.
/// - err: Error code
///
/// #Errors

extern indy_error_t indy_agent_close_listener(indy_handle_t command_handle,
                                              indy_handle_t listener_handle,

                                              void     (*cb)(indy_handle_t xcommand_handle,
                                                             indy_error_t  err)
                                              );

#ifdef __cplusplus
}
#endif

#endif
