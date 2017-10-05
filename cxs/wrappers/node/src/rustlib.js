"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var ref = require("ref");
var Struct = require("ref-struct");
var CxsStatus = Struct({
    'handle': 'int',
    'status': 'int',
    'msg': 'string',
});
//FFI Type Strings
exports.ffi_error_code = 'int';
exports.ffi_connection_handle = 'uint32';
exports.ffi_unsigned_int = 'uint32';
exports.ffi_unsigned_int_ptr = ref.refType('uint32');
exports.ffi_string = 'string';
exports.ffi_string_data = 'string';
exports.ffi_connection_handle_ptr = ref.refType(exports.ffi_connection_handle);
exports.ffi_CxsStatus = ref.refType(CxsStatus);
exports.FFIConfiguration = {
    'cxs_init': [exports.ffi_error_code, []],
    //connection.rs
    'cxs_connection_create': [exports.ffi_error_code, [exports.ffi_string_data, exports.ffi_connection_handle_ptr]],
    'cxs_connection_connect': [exports.ffi_error_code, [exports.ffi_connection_handle]],
    'cxs_connection_get_data': [exports.ffi_string_data, [exports.ffi_connection_handle]],
    'cxs_connection_get_state': [exports.ffi_error_code, [exports.ffi_connection_handle, exports.ffi_unsigned_int_ptr]],
    'cxs_connection_release': [exports.ffi_error_code, [exports.ffi_connection_handle]],
};
