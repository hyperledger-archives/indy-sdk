extern crate libc;

use api::ErrorCode;
use errors::ToErrorCode;
use commands::{Command, CommandExecutor};
use commands::authz::AuthzCommand;
use utils::cstring::CStringUtils;

use self::libc::c_char;


#[no_mangle]
pub  extern fn indy_create_and_store_new_policy(command_handle: i32,
                                                wallet_handle: i32,
                                                cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                    policy_address: *const c_char)>) -> ErrorCode {
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam2);

    let result = CommandExecutor::instance()
        .send(Command::Authz(AuthzCommand::CreateAndStorePolicy(
            wallet_handle,
            Box::new(move |result| {
                let (err, policy) = result_to_err_code_1!(result, String::new());
                let policy = CStringUtils::string_to_cstring(policy);
                cb(command_handle, err, policy.as_ptr())
            })
        )));

    result_to_err_code!(result)
}


#[no_mangle]
pub  extern fn indy_add_new_agent_to_policy(command_handle: i32,
                                            wallet_handle: i32,
                                            policy_address: *const c_char,
                                            verkey: *const c_char,
                                            add_commitment: bool,
                                      cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                           vk: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(policy_address, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(verkey, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    let result = CommandExecutor::instance()
        .send(Command::Authz(AuthzCommand::AddAgentToStoredPolicy(
            wallet_handle,
            policy_address,
            verkey,
            add_commitment,
            Box::new(move |result| {
                let (err, policy) = result_to_err_code_1!(result, String::new());
                let policy = CStringUtils::string_to_cstring(policy);
                cb(command_handle, err, policy.as_ptr())
            })
        )));

    result_to_err_code!(result)
}


#[no_mangle]
pub  extern fn indy_update_agent_witness(command_handle: i32,
                                            wallet_handle: i32,
                                            policy_address: *const c_char,
                                            verkey: *const c_char,
                                            witness: *const c_char,
                                            cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                 vk: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(policy_address, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(verkey, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(witness, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    let result = CommandExecutor::instance()
        .send(Command::Authz(AuthzCommand::UpdateAgentWitness(
            wallet_handle,
            policy_address,
            verkey,
            witness,
            Box::new(move |result| {
                let (err, policy) = result_to_err_code_1!(result, String::new());
                let policy = CStringUtils::string_to_cstring(policy);
                cb(command_handle, err, policy.as_ptr())
            })
        )));

    result_to_err_code!(result)
}


#[no_mangle]
pub  extern fn indy_generate_witness(command_handle: i32,
                                     initial_witness: *const c_char,
                                     witness_array: *const c_char,
                               cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                    policy_json: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(initial_witness, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(witness_array, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    let result = CommandExecutor::instance()
        .send(Command::Authz(AuthzCommand::ComputeWitness(
            initial_witness,
            witness_array,
            Box::new(move |result| {
                let (err, new_witness) = result_to_err_code_1!(result, String::new());
                let new_witness = CStringUtils::string_to_cstring(new_witness);
                cb(command_handle, err, new_witness.as_ptr())
            })
        )));

    result_to_err_code!(result)
}


#[no_mangle]
pub  extern fn indy_get_policy(command_handle: i32,
                                                wallet_handle: i32,
                                                policy_address: *const c_char,
                                                cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                     policy_json: *const c_char)>) -> ErrorCode {
    check_useful_c_str!(policy_address, ErrorCode::CommonInvalidParam2);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam3);

    let result = CommandExecutor::instance()
        .send(Command::Authz(AuthzCommand::GetPolicy(
            wallet_handle,
            policy_address,
            Box::new(move |result| {
                let (err, policy) = result_to_err_code_1!(result, String::new());
                let policy = CStringUtils::string_to_cstring(policy);
                cb(command_handle, err, policy.as_ptr())
            })
        )));

    result_to_err_code!(result)
}
