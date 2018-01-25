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
export const FFI_BOOL = 'bool'
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
export const FFI_CLAIMDEF_HANDLE = 'uint32'
export const FFI_SCHEMA_HANDLE = 'uint32'
export const FFI_SCHEMA_NUMBER = 'uint32'

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
  cxs_connection_invite_details: (commandId: number, handle: string, abbreviated: boolean, cb: any) => number,
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
  cxs_get_proof: (commandId: number, proofHandle: string, connectionHandle: string, cb: any) => number,
  cxs_proof_release: (handle: string) => number,
  cxs_proof_send_request: (commandId: number, proofHandle: string, connectionHandle: string, cb: any) => number,
  cxs_proof_serialize: (commandId: number, handle: string, cb: any) => number,
  cxs_proof_update_state: (commandId: number, handle: string, cb: any) => number,
  // mock
  cxs_set_next_agency_response: (messageIndex: number) => void,

  // claimdef
  cxs_claimdef_create: (commandId: number, sourceId: string, claimDefName: string, schemaNo: number,
                        issuerDid: string, revocation: boolean, cb: any) => number
  cxs_claimdef_deserialize: (commandId: number, data: string, cb: any) => number,
  cxs_claimdef_serialize: (commandId: number, handle: string, cb: any) => number,
  cxs_claimdef_release: (handle: string) => number,

  // schema
  cxs_schema_get_attributes: (commandId: number, sourceId: string, schemaNo: number, cb: any) => number,
  cxs_schema_create: (commandId: number, sourceId: string, schemaName: string, schemaData: string,
                      cb: any) => number,
  cxs_schema_get_sequence_no: (commandId: number, handle: string, cb: any) => number,
  cxs_schema_deserialize: (commandId: number, data: string, cb: any) => number,
  cxs_schema_serialize: (commandId: number, handle: string, cb: any) => number,
  cxs_schema_release: (handle: string) => number,

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
  cxs_connection_invite_details: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_CONNECTION_HANDLE, FFI_BOOL,
    FFI_CALLBACK_PTR]],
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
  cxs_get_proof: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_PROOF_HANDLE, FFI_CONNECTION_HANDLE,
    FFI_CALLBACK_PTR]],// tslint:disable-line
  cxs_proof_release: [FFI_ERROR_CODE, [FFI_PROOF_HANDLE]],
  cxs_proof_send_request: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_PROOF_HANDLE, FFI_CONNECTION_HANDLE,
    FFI_CALLBACK_PTR]],
  cxs_proof_serialize: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_PROOF_HANDLE, FFI_CALLBACK_PTR]],
  cxs_proof_update_state: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_PROOF_HANDLE, FFI_CALLBACK_PTR]],
  // claimDef
  cxs_claimdef_create: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_SOURCE_ID, FFI_STRING_DATA, FFI_SCHEMA_NUMBER,
    FFI_STRING_DATA, FFI_BOOL, FFI_CALLBACK_PTR]],
  cxs_claimdef_deserialize: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_STRING_DATA, FFI_CALLBACK_PTR]],
  cxs_claimdef_release: [FFI_ERROR_CODE, [FFI_CLAIMDEF_HANDLE]],
  cxs_claimdef_serialize: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_CLAIMDEF_HANDLE, FFI_CALLBACK_PTR]],
  // mock
  cxs_set_next_agency_response: [FFI_VOID, [FFI_UNSIGNED_INT]],
  // schema
  cxs_schema_get_attributes: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_SOURCE_ID, FFI_SCHEMA_NUMBER, FFI_CALLBACK_PTR]],
  cxs_schema_create: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_SOURCE_ID, FFI_STRING_DATA, FFI_STRING_DATA,
    FFI_CALLBACK_PTR]],
  cxs_schema_get_sequence_no: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_SCHEMA_HANDLE, FFI_CALLBACK_PTR]],
  cxs_schema_deserialize: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_STRING_DATA, FFI_CALLBACK_PTR]],
  cxs_schema_release: [FFI_ERROR_CODE, [FFI_SCHEMA_HANDLE]],
  cxs_schema_serialize: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_SCHEMA_HANDLE, FFI_CALLBACK_PTR]]
}

let _rustAPI: IFFIEntryPoint = null
export const initRustAPI = (path?: string) => _rustAPI = new CXSRuntime({ basepath: path }).ffi
export const rustAPI = () => _rustAPI
