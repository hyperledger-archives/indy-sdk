#ifndef __sorvin_agent__included__
#define __sorvin_agent__included__

#ifdef __cplusplus
extern "C" {
#endif

/// Establishes agent to agent connection.
///
/// Information about sender Identity must be saved in the wallet with sovrin_create_and_store_my_did
/// call before establishing of connection.
///
/// Information about receiver Identity can be saved in the wallet with sovrin_store_their_did
/// call before establishing of connection. If there is no corresponded wallet record for receiver Identity
/// than this call will lookup Identity Ledger and cache this information in the wallet.
///
/// Note that messages encryption/decryption will be performed automatically.
///
/// #Params
/// command_handle: Command handle to map callback to caller context.
/// wallet_handle: Wallet handle (created by open_wallet).
/// sender_did: Id of sender Identity stored in secured Wallet.
/// receiver_did: Id of receiver Identity.
/// connection_cb: Callback that will be called after establishing of connection or on error.
/// message_cb: Callback that will be called on receiving of an incomming message.
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

extern sorvin_error_t sovrin_agent_connect(sorvin_handle_t command_handle,
                                           sorvin_handle_t wallet_handle,
                                           const char *    sender_did,
                                           const char *    receiver_did,

                                           void  (*connection_cb)(sovrin_handle_t xcommand_handle,
                                                                  sovrin_error_t  err,
                                                                  sovrin_handle_t connection_handle),
                                                                                                             );
                                           void     (*message_cb)(sovrin_handle_t xconnection_handle,
                                                                  sovrin_error_t  err,
                                                                  const char *    message)
                                           );

/// Starts listening of agent connections.
///
/// On incomming connection listener performs wallet lookup to find corresponded receiver Identity
/// information. Information about receiver Identity must be saved in the wallet with
/// sovrin_create_and_store_my_did call before establishing of connection.
///
/// Information about sender Identity for incomming connection validation can be saved in the wallet
/// with sovrin_store_their_did call before establishing of connection. If there is no corresponded
/// wallet record for sender Identity than listener will lookup Identity Ledger and cache this
/// information in the wallet.
///
/// Note that messages encryption/decryption will be performed automatically.
///
/// #Params
/// command_handle: command handle to map callback to caller context.
/// wallet_handle: wallet handle (created by open_wallet).
/// listener_cb: Callback that will be called after listening started or on error.
/// connection_cb: Callback that will be called after establishing of incomming connection.
/// message_cb: Callback that will be called on receiving of an incomming message.
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

extern sorvin_error_t sovrin_agent_listen(sorvin_handle_t command_handle,
                                          sorvin_handle_t wallet_handle,

                                          void     (*listener_cb)(sovrin_handle_t xcommand_handle,
                                                                  sovrin_error_t  err,
                                                                  sovrin_handle_t listener_handle),

                                          void     (*connection_cb)(sovrin_handle_t xlistener_handle,
                                                                  sovrin_error_t  err,
                                                                  sovrin_handle_t connection_handle,
                                                                  const char *    sender_did,
                                                                  const char *    receiver_did),

                                          void     (*listener_cb)(sovrin_handle_t xconnection_handle,
                                                                  sovrin_error_t  err,
                                                                  const char *    message)
                                          );

/// Sends message to connected agent.
///
/// Note that this call works for both incoming and outgoing connections.
/// Note that messages encryption/decryption will be performed automatically.
///
/// #Params
/// command_handle: command handle to map callback to caller context.
/// connection_handle: Connection handle returned by sovrin_agent_connect or sovrin_agent_listen calls.
/// message: Message to send.
/// cb: Callback that will be called after message sent or on error.
///
/// #Returns
/// err: Error code
/// cb:
/// - xcommand_handle: Command handle to map callback to caller context.
/// - err: Error code
///
/// #Errors

extern sorvin_error_t sovrin_agent_send(sorvin_handle_t command_handle,
                                        sorvin_handle_t connection_handle,
                                        const char *    message,

                                        void     (*cb)(sovrin_handle_t xcommand_handle,
                                                       sovrin_error_t  err)
                                       );

/// Closes agent connection.
///
/// Note that this call works for both incoming and outgoing connections.
///
/// #Params
/// command_handle: command handle to map callback to caller context.
/// connection_handle: Connection handle returned by sovrin_agent_connect or sovrin_agent_listen calls.
/// cb: Callback that will be called after connection closed or on error.
///
/// #Returns
/// Error code
/// cb:
/// - xcommand_handle: command handle to map callback to caller context.
/// - err: Error code
///
/// #Errors

extern sorvin_error_t sovrin_agent_close_connection(sorvin_handle_t command_handle,
                                                    sorvin_handle_t connection_handle,

                                                    void     (*cb)(sovrin_handle_t xcommand_handle,
                                                                   sovrin_error_t  err)
                                                    );


/// Closes listener and stops listening for agent connections.
///
/// Note that all opened incomming connections will be closed automatically.
///
/// #Params
/// command_handle: command handle to map callback to caller context.
/// listener_handle: Listener handle returned by sovrin_agent_listen call.
/// cb: Callback that will be called after listener closed or on error.
///
/// #Returns
/// Error code
/// cb:
/// - xcommand_handle: command handle to map callback to caller context.
/// - err: Error code
///
/// #Errors

extern sorvin_error_t sovrin_agent_close_connection(sorvin_handle_t command_handle,
                                                    sorvin_handle_t listener_handle,

                                                    void     (*cb)(sovrin_handle_t xcommand_handle,
                                                                   sovrin_error_t  err)
                                                    );

#ifdef __cplusplus
}
#endif

#endif