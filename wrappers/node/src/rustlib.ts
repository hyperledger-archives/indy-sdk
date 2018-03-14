import * as ref from 'ref'
import * as StructType from 'ref-struct'

import { VCXRuntime } from './vcx'

export const VcxStatus = StructType({
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
  vcx_init: (commandId: number, configPath: string, cb: any) => number,
  vcx_error_c_message: (errorCode: number) => string,
  // connection
  vcx_connection_connect: (commandId: number, handle: string, data: string, cb: any) => number,
  vcx_connection_create: (commandId: number, data: string, cb: any) => number,
  vcx_connection_create_with_invite: (commandId: number, data: string, invite: string, cb: any) => number,
  vcx_connection_deserialize: (commandId: number, data: string, cb: any) => number,
  vcx_connection_release: (handle: string) => number,
  vcx_connection_serialize: (commandId: number, handle: string, cb: any) => number,
  vcx_connection_update_state: (commandId: number, handle: string, cb: any) => number,
  vcx_connection_get_state: (commandId: number, handle: string, cb: any) => number,
  vcx_connection_invite_details: (commandId: number, handle: string, abbreviated: boolean, cb: any) => number,
  // issuer
  vcx_issuer_claim_release: (handle: string) => number,
  vcx_issuer_claim_deserialize: (commandId: number, data: string, cb: any) => number,
  vcx_issuer_claim_serialize: (commandId: number, handle: string, cb: any) => number,
  vcx_issuer_claim_update_state: (commandId: number, handle: string, cb: any) => number,
  vcx_issuer_claim_get_state: (commandId: number, handle: string, cb: any) => number,
  vcx_issuer_create_claim: any,
  vcx_issuer_send_claim: (commandId: number, claimHandle: string, connectionHandle: string, cb: any) => number,
  vcx_issuer_send_claim_offer: (commandId: number, claimHandle: string, connectionHandle: string, cb: any) => number,
  // proof
  vcx_proof_create: (commandId: number, sourceId: string, attrs: string, predicates: string,
                     name: string, cb: any) => number,
  vcx_proof_deserialize: (commandId: number, data: string, cb: any) => number,
  vcx_get_proof: (commandId: number, proofHandle: string, connectionHandle: string, cb: any) => number,
  vcx_proof_release: (handle: string) => number,
  vcx_proof_send_request: (commandId: number, proofHandle: string, connectionHandle: string, cb: any) => number,
  vcx_proof_serialize: (commandId: number, handle: string, cb: any) => number,
  vcx_proof_update_state: (commandId: number, handle: string, cb: any) => number,
  vcx_proof_get_state: (commandId: number, handle: string, cb: any) => number,
  // mock
  vcx_set_next_agency_response: (messageIndex: number) => void,

  // claimdef
  vcx_claimdef_create: (commandId: number, sourceId: string, claimDefName: string, schemaNo: number,
                        issuerDid: string, revocation: boolean, cb: any) => number
  vcx_claimdef_deserialize: (commandId: number, data: string, cb: any) => number,
  vcx_claimdef_serialize: (commandId: number, handle: string, cb: any) => number,
  vcx_claimdef_release: (handle: string) => number,

  // schema
  vcx_schema_get_attributes: (commandId: number, sourceId: string, schemaNo: number, cb: any) => number,
  vcx_schema_create: (commandId: number, sourceId: string, schemaName: string, schemaData: string,
                      cb: any) => number,
  vcx_schema_get_sequence_no: (commandId: number, handle: string, cb: any) => number,
  vcx_schema_deserialize: (commandId: number, data: string, cb: any) => number,
  vcx_schema_serialize: (commandId: number, handle: string, cb: any) => number,
  vcx_schema_release: (handle: string) => number,

}

// tslint:disable object-literal-sort-keys
export const FFIConfiguration: { [ Key in keyof IFFIEntryPoint ]: any } = {

  vcx_init: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_CONFIG_PATH, FFI_CALLBACK_PTR]],
  vcx_error_c_message: [FFI_STRING, [FFI_ERROR_CODE]],

  // connection
  vcx_connection_connect: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_CONNECTION_HANDLE, FFI_CONNECTION_DATA,
    FFI_CALLBACK_PTR]],
  vcx_connection_create: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_STRING_DATA, FFI_CALLBACK_PTR]],
  vcx_connection_create_with_invite: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_STRING_DATA, FFI_STRING_DATA,
    FFI_CALLBACK_PTR]],
  vcx_connection_deserialize: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_STRING_DATA, FFI_CALLBACK_PTR]],
  vcx_connection_release: [FFI_ERROR_CODE, [FFI_CONNECTION_HANDLE]],
  vcx_connection_serialize: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_CONNECTION_HANDLE, FFI_CALLBACK_PTR]],
  vcx_connection_update_state: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_CONNECTION_HANDLE, FFI_CALLBACK_PTR]],
  vcx_connection_get_state: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_CONNECTION_HANDLE, FFI_CALLBACK_PTR]],
  vcx_connection_invite_details: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_CONNECTION_HANDLE, FFI_BOOL,
    FFI_CALLBACK_PTR]],
  // issuer
  vcx_issuer_claim_deserialize: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_STRING_DATA, FFI_CALLBACK_PTR]],
  vcx_issuer_claim_serialize: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_CLAIM_HANDLE, FFI_CALLBACK_PTR]],
  vcx_issuer_claim_update_state: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_CLAIM_HANDLE, FFI_CALLBACK_PTR]],
  vcx_issuer_claim_get_state: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_CLAIM_HANDLE, FFI_CALLBACK_PTR]],
  vcx_issuer_create_claim: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_SOURCE_ID,
    'int', 'string', 'string', 'string', 'pointer']],
  vcx_issuer_send_claim: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_CLAIM_HANDLE, FFI_CONNECTION_HANDLE,
    FFI_CALLBACK_PTR]],
  vcx_issuer_send_claim_offer: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_CLAIM_HANDLE, FFI_CONNECTION_HANDLE,
    FFI_CALLBACK_PTR]],
  vcx_issuer_claim_release: [FFI_ERROR_CODE, [FFI_CLAIM_HANDLE]],
  // proof
  vcx_proof_create: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_SOURCE_ID, FFI_STRING_DATA, FFI_STRING_DATA,
    FFI_STRING_DATA, FFI_CALLBACK_PTR]],
  vcx_proof_deserialize: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_STRING_DATA, FFI_CALLBACK_PTR]],
  vcx_get_proof: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_PROOF_HANDLE, FFI_CONNECTION_HANDLE,
    FFI_CALLBACK_PTR]],// tslint:disable-line
  vcx_proof_release: [FFI_ERROR_CODE, [FFI_PROOF_HANDLE]],
  vcx_proof_send_request: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_PROOF_HANDLE, FFI_CONNECTION_HANDLE,
    FFI_CALLBACK_PTR]],
  vcx_proof_serialize: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_PROOF_HANDLE, FFI_CALLBACK_PTR]],
  vcx_proof_update_state: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_PROOF_HANDLE, FFI_CALLBACK_PTR]],
  vcx_proof_get_state: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_PROOF_HANDLE, FFI_CALLBACK_PTR]],
  // claimDef
  vcx_claimdef_create: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_SOURCE_ID, FFI_STRING_DATA, FFI_SCHEMA_NUMBER,
    FFI_STRING_DATA, FFI_BOOL, FFI_CALLBACK_PTR]],
  vcx_claimdef_deserialize: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_STRING_DATA, FFI_CALLBACK_PTR]],
  vcx_claimdef_release: [FFI_ERROR_CODE, [FFI_CLAIMDEF_HANDLE]],
  vcx_claimdef_serialize: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_CLAIMDEF_HANDLE, FFI_CALLBACK_PTR]],
  // mock
  vcx_set_next_agency_response: [FFI_VOID, [FFI_UNSIGNED_INT]],
  // schema
  vcx_schema_get_attributes: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_SOURCE_ID, FFI_SCHEMA_NUMBER, FFI_CALLBACK_PTR]],
  vcx_schema_create: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_SOURCE_ID, FFI_STRING_DATA, FFI_STRING_DATA,
    FFI_CALLBACK_PTR]],
  vcx_schema_get_sequence_no: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_SCHEMA_HANDLE, FFI_CALLBACK_PTR]],
  vcx_schema_deserialize: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_STRING_DATA, FFI_CALLBACK_PTR]],
  vcx_schema_release: [FFI_ERROR_CODE, [FFI_SCHEMA_HANDLE]],
  vcx_schema_serialize: [FFI_ERROR_CODE, [FFI_COMMAND_HANDLE, FFI_SCHEMA_HANDLE, FFI_CALLBACK_PTR]]
}

let _rustAPI: IFFIEntryPoint = null
export const initRustAPI = (path?: string) => _rustAPI = new VCXRuntime({ basepath: path }).ffi
export const rustAPI = () => _rustAPI
