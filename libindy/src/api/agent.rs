extern crate libc;

use api::ErrorCode;
use commands::{Command, CommandExecutor};
use commands::agent::AgentCommand;
use errors::ToErrorCode;
use utils::cstring::CStringUtils;

use self::libc::c_char;

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
#[no_mangle]
pub extern fn indy_agent_connect(command_handle: i32,
                                 pool_handle: i32,
                                 wallet_handle: i32,
                                 sender_did: *const c_char,
                                 receiver_did: *const c_char,
                                 connection_cb: Option<extern fn(xcommand_handle: i32,
                                                                 err: ErrorCode,
                                                                 connection_handle: i32)>,
                                 message_cb: Option<extern fn(xconnection_handle: i32,
                                                              err: ErrorCode,
                                                              message: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(sender_did, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(receiver_did, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(connection_cb, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(message_cb, ErrorCode::CommonInvalidParam6);

    let result = CommandExecutor::instance().send(
        Command::Agent(
            AgentCommand::Connect(
                pool_handle,
                wallet_handle,
                sender_did,
                receiver_did,
                Box::new(move |result| {
                    let (err, handle) = result_to_err_code_1!(result, 0);
                    connection_cb(command_handle, err, handle);
                }),
                Box::new(move |result| {
                    let (err, handle, msg) = result_to_err_code_2!(result, 0, String::new());
                    let msg = CStringUtils::string_to_cstring(msg);
                    message_cb(handle, err, msg.as_ptr());
                })
            )
        )
    );

    result_to_err_code!(result)
}

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
#[no_mangle]
pub extern fn indy_agent_listen(command_handle: i32,
                                endpoint: *const c_char,
                                listener_cb: Option<extern fn(xcommand_handle: i32,
                                                              err: ErrorCode,
                                                              listener_handle: i32)>,
                                connection_cb: Option<extern fn(xlistener_handle: i32,
                                                                err: ErrorCode,
                                                                connection_handle: i32,
                                                                sender_did: *const c_char,
                                                                receiver_did: *const c_char)>,
                                message_cb: Option<extern fn(xconnection_handle: i32,
                                                             err: ErrorCode,
                                                             message: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(endpoint, ErrorCode::CommonInvalidParam2);
    check_useful_c_callback!(listener_cb, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(connection_cb, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(message_cb, ErrorCode::CommonInvalidParam5);

    let cmd = Command::Agent(AgentCommand::Listen(
        endpoint,
        Box::new(move |result| {
            let (err, handle) = result_to_err_code_1!(result, 0);
            listener_cb(command_handle, err, handle);
        }),
        Box::new(move |result| {
            let (err, listener_handle, conn_handle, sender_did, receiver_did) =
                result_to_err_code_4!(result, 0, 0, String::new(), String::new());
            connection_cb(listener_handle, err, conn_handle,
                          CStringUtils::string_to_cstring(sender_did).as_ptr(),
                          CStringUtils::string_to_cstring(receiver_did).as_ptr());
        }),
        Box::new(move |result| {
            let (err, handle, msg) = result_to_err_code_2!(result, 0, String::new());
            let msg = CStringUtils::string_to_cstring(msg);
            message_cb(handle, err, msg.as_ptr());
        })
    ));

    let result = CommandExecutor::instance().send(cmd);

    result_to_err_code!(result)
}

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
#[no_mangle]
pub extern fn indy_agent_add_identity(command_handle: i32,
                                      listener_handle: i32,
                                      pool_handle: i32,
                                      wallet_handle: i32,
                                      did: *const c_char,
                                      add_identity_cb: Option<extern fn(xcommand_handle: i32,
                                                                        err: ErrorCode)>) -> ErrorCode {
    check_useful_c_str!(did, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(add_identity_cb, ErrorCode::CommonInvalidParam6);

    let cmd = Command::Agent(AgentCommand::ListenerAddIdentity(
        listener_handle,
        pool_handle,
        wallet_handle,
        did,
        Box::new(move |result| {
            let result = result_to_err_code!(result);
            add_identity_cb(command_handle, result);
        }),
    ));

    let result = CommandExecutor::instance().send(cmd);

    result_to_err_code!(result)
}

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
#[no_mangle]
pub extern fn indy_agent_remove_identity(command_handle: i32,
                                         listener_handle: i32,
                                         wallet_handle: i32,
                                         did: *const c_char,
                                         rm_identity_cb: Option<extern fn(xcommand_handle: i32,
                                                                          err: ErrorCode)>) -> ErrorCode {
    check_useful_c_str!(did, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(rm_identity_cb, ErrorCode::CommonInvalidParam5);

    let cmd = Command::Agent(AgentCommand::ListenerRmIdentity(
        listener_handle,
        wallet_handle,
        did,
        Box::new(move |result| {
            let result = result_to_err_code!(result);
            rm_identity_cb(command_handle, result);
        }),
    ));

    let result = CommandExecutor::instance().send(cmd);

    result_to_err_code!(result)
}

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
#[no_mangle]
pub extern fn indy_agent_send(command_handle: i32,
                              connection_handle: i32,
                              message: *const c_char,
                              cb: Option<extern fn(xcommand_handle: i32,
                                                   err: ErrorCode)>) -> ErrorCode {
    check_useful_opt_c_str!(message, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    let cmd = Command::Agent(AgentCommand::Send(
        connection_handle,
        message,
        Box::new(move |result| {
            cb(command_handle, result_to_err_code!(result))
        })
    ));

    let res = CommandExecutor::instance().send(cmd);
    result_to_err_code!(res)
}

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
#[no_mangle]
pub extern fn indy_agent_close_connection(command_handle: i32,
                                          connection_handle: i32,
                                          cb: Option<extern fn(xcommand_handle: i32,
                                                               err: ErrorCode)>) -> ErrorCode {
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    let cmd = Command::Agent(AgentCommand::CloseConnection(
        connection_handle,
        Box::new(move |result| {
            cb(command_handle, result_to_err_code!(result))
        })
    ));

    let res = CommandExecutor::instance().send(cmd);
    result_to_err_code!(res)
}

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
#[no_mangle]
pub extern fn indy_agent_close_listener(command_handle: i32,
                                        listener_handle: i32,
                                        cb: Option<extern fn(xcommand_handle: i32,
                                                             err: ErrorCode)>) -> ErrorCode {
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    let cmd = Command::Agent(AgentCommand::CloseListener(
        listener_handle,
        Box::new(move |result| {
            cb(command_handle, result_to_err_code!(result))
        })
    ));

    let res = CommandExecutor::instance().send(cmd);
    result_to_err_code!(res)
}
