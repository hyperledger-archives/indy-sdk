export type FFIEntryPoint = any

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
    'cxs_init': ['int', []]
}