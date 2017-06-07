extern crate libc;

use api::ErrorCode;
use commands::{Command, CommandExecutor};
use commands::agent::AgentCommand;
use errors::ToErrorCode;
use utils::cstring::CStringUtils;

use self::libc::c_char;

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
#[no_mangle]
pub extern fn sovrin_agent_connect(command_handle: i32,
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
#[no_mangle]
pub extern fn sovrin_agent_listen(command_handle: i32,
                                  wallet_handle: i32,
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
    check_useful_c_callback!(listener_cb, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(connection_cb, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(message_cb, ErrorCode::CommonInvalidParam5);

    let cmd = Command::Agent(AgentCommand::Listen(
        wallet_handle,
        Box::new(move |result| {
            let (err, handle, endpoint) = result_to_err_code_2!(result, 0, String::new());
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
#[no_mangle]
pub extern fn sovrin_agent_send(command_handle: i32,
                                connection_handle: i32,
                                message: *const c_char,
                                cb: Option<extern fn(xcommand_handle: i32,
                                                     err: ErrorCode)>) -> ErrorCode {
    unimplemented!()
}

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
#[no_mangle]
pub extern fn sovrin_agent_close_connection(command_handle: i32,
                                            connection_handle: i32,
                                            cb: Option<extern fn(xcommand_handle: i32,
                                                                 err: ErrorCode)>) -> ErrorCode {
    unimplemented!()
}

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
#[no_mangle]
pub extern fn sovrin_agent_close_listener(command_handle: i32,
                                          listener_handle: i32,
                                          cb: Option<extern fn(xcommand_handle: i32,
                                                               err: ErrorCode)>) -> ErrorCode {
    unimplemented!()
}