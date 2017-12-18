import * as ref from 'ref'
import * as StructType from 'ref-struct'

import { CXSRuntime } from './cxs'

export const CxsStatus = StructType({
  handle: 'int',
  msg: 'string',
  status: 'int'
})

// FFI Type Strings
export const FFI_ERROR_CODE = 'int'
export const FFI_CONNECTION_HANDLE = 'uint32'
export const FFI_UNSIGNED_INT = 'uint32'
export const FFI_UNSIGNED_INT_PTR = ref.refType('uint32')
export const FFI_STRING = 'string'
export const FFI_CONFIG_PATH = FFI_STRING
export const FFI_STRING_DATA = 'string'
export const FFI_SOURCE_ID = 'string'
export const FFI_CONNECTION_DATA = 'string'
export const FFI_VOID = ref.types.void
export const FFI_CONNECTION_HANDLE_PTR = ref.refType(FFI_CONNECTION_HANDLE)
export const FFI_CALLBACK_PTR = 'pointer'
export const FFI_COMMAND_HANDLE = 'uint32'
export const FFI_CLAIM_HANDLE = 'uint32'
export const FFI_PROOF_HANDLE = 'uint32'

// Rust Lib Native Types
export type rust_did = string
export type rust_error_code = number
export type rust_command_handle = number
export type rust_object_handle = number
export type rust_pool_handle = rust_object_handle
export type rust_wallet_handle = rust_object_handle
export type rust_listener_handle = rust_object_handle
export type rust_connection_handle = rust_object_handle

export interface IFFIEntryPoint {
  cxs_init: (commandId: number, configPath: string, cb: any) => number,
  // connection
  cxs_connection_connect: (commandId: number, handle: string, data: string, cb: any) => number,
  cxs_connection_create: (commandId: number, data: string, cb: any) => number,
  cxs_connection_deserialize: (commandId: number, data: string, cb: any) => number,
  cxs_connection_release: (handle: string) => number,
  cxs_connection_serialize: (commandId: number, handle: string, cb: any) => number,
  cxs_connection_update_state: (commandId: number, handle: string, cb: any) => number,
  // issuer
  cxs_issuer_claim_deserialize: (commandId: number, data: string, cb: any) => number,
  cxs_issuer_claim_serialize: (commandId: number, handle: string, cb: any) => number,
  cxs_issuer_claim_update_state: (commandId: number, handle: string, cb: any) => number,
  cxs_issuer_create_claim: any,
  cxs_issuer_send_claim: (commandId: number, claimHandle: string, connectionHandle: string, cb: any) => number,
  cxs_issuer_send_claim_offer: (commandId: number, claimHandle: string, connectionHandle: string, cb: any) => number,
  // proof
  cxs_proof_create: (commandId: number, sourceId: string, attrs: string, predicates: string,
                     name: string, cb: any) => number,
  cxs_proof_deserialize: (commandId: number, data: string, cb: any) => number,
  cxs_proof_get_proof_offer: (commandId: number, proofHandle: string, connectionHandle: string, cb: any) => number,
  cxs_proof_release: (handle: string) => number,
  cxs_proof_send_request: (commandId: number, proofHandle: string, connectionHandle: string, cb: any) => number,
  cxs_proof_serialize: (commandId: number, handle: string, cb: any) => number,
  cxs_proof_update_state: (commandId: number, handle: string, cb: any) => number,

  free: any
}

// tslint:disable object-literal-sort-keys
export const FFIConfiguration: { [ Key in keyof IFFIEntryPoint ]: any } = {

  cxs_init: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_CONFIG_PATH, FFI_CALLBACK_PTR]],
  // connection
  cxs_connection_connect: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_CONNECTION_HANDLE, FFI_CONNECTION_DATA,
    FFI_CALLBACK_PTR]],
  cxs_connection_create: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_STRING_DATA, FFI_CALLBACK_PTR]],
  cxs_connection_deserialize: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_STRING_DATA, FFI_CALLBACK_PTR]],
  cxs_connection_release: [FFI_ERROR_CODE, [FFI_CONNECTION_HANDLE]],
  cxs_connection_serialize: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_CONNECTION_HANDLE, FFI_CALLBACK_PTR]],
  cxs_connection_update_state: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_CONNECTION_HANDLE, FFI_CALLBACK_PTR]],
  // issuer
  cxs_issuer_claim_deserialize: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_STRING_DATA, FFI_CALLBACK_PTR]],
  cxs_issuer_claim_serialize: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_CLAIM_HANDLE, FFI_CALLBACK_PTR]],
  cxs_issuer_claim_update_state: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_CLAIM_HANDLE, FFI_CALLBACK_PTR]],
  cxs_issuer_create_claim: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_SOURCE_ID,
    'int', 'string', 'string', 'string', 'pointer']],
  cxs_issuer_send_claim: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_CLAIM_HANDLE, FFI_CONNECTION_HANDLE,
    FFI_CALLBACK_PTR]],
  cxs_issuer_send_claim_offer: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_CLAIM_HANDLE, FFI_CONNECTION_HANDLE,
    FFI_CALLBACK_PTR]],
  // proof
  cxs_proof_create: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_SOURCE_ID, FFI_STRING_DATA, FFI_STRING_DATA,
    FFI_STRING_DATA, FFI_CALLBACK_PTR]],
  cxs_proof_deserialize: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_STRING_DATA, FFI_CALLBACK_PTR]],
  cxs_proof_get_proof_offer: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_PROOF_HANDLE, FFI_CONNECTION_HANDLE,
    FFI_CALLBACK_PTR]],
  cxs_proof_release: [FFI_ERROR_CODE, [FFI_PROOF_HANDLE]],
  cxs_proof_send_request: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_PROOF_HANDLE, FFI_CONNECTION_HANDLE,
    FFI_CALLBACK_PTR]],
  cxs_proof_serialize: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_PROOF_HANDLE, FFI_CALLBACK_PTR]],
  cxs_proof_update_state: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_PROOF_HANDLE, FFI_CALLBACK_PTR]],

  free: [FFI_VOID, ['void*']]
}

let _rustAPI: IFFIEntryPoint = null
export const initRustAPI = (path?: string) => _rustAPI = new CXSRuntime({ basepath: path }).ffi
export const rustAPI = () => _rustAPI
