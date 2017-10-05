import * as ref from 'ref'
import * as Struct from 'ref-struct'

export type FFIEntryPoint = any

var CxsStatus = Struct({
    'handle': 'int',
    'status': 'int',
    'msg': 'string',
})

//FFI Type Strings
export const ffi_error_code = 'int'
export const ffi_connection_handle = 'uint32'
export const ffi_unsigned_int = 'uint32'
export const ffi_unsigned_int_ptr = ref.refType('uint32')
export const ffi_string = 'string'
export const ffi_string_data = 'string'
export const ffi_connection_handle_ptr = ref.refType(ffi_connection_handle)
export const ffi_CxsStatus = ref.refType(CxsStatus)

// Rust Lib Native Types
export type rust_did = string
export type rust_error_code = number
export type rust_command_handle = number
export type rust_object_handle = number
export type rust_pool_handle = rust_object_handle
export type rust_wallet_handle = rust_object_handle
export type rust_listener_handle = rust_object_handle
export type rust_connection_handle = rust_object_handle

export interface FFIInterfaceConfig {
    libraryPath?:string
}


export const FFIConfiguration = {
    'cxs_init': [ffi_error_code, []],

    //connection.rs
    'cxs_connection_create': [ffi_error_code, [ffi_string_data, ffi_connection_handle_ptr]],
    'cxs_connection_connect': [ffi_error_code, [ffi_connection_handle]],
    'cxs_connection_get_data': [ffi_string_data, [ffi_connection_handle]],
    'cxs_connection_get_state': [ffi_error_code, [ffi_connection_handle, ffi_unsigned_int_ptr]],
    'cxs_connection_release': [ffi_error_code, [ffi_connection_handle]],
    // 'cxs_connection_list_state': [ffi_error_code, [ffi_CxsStatus]],

}