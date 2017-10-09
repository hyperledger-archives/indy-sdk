"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const ref = require("ref");
const StructType = require("ref-struct");
/* tslint: disable */
exports.CxsStatus = StructType({
    handle: 'int',
    msg: 'string',
    status: 'int'
});
// FFI Type Strings
exports.FFI_ERROR_CODE = 'int';
exports.FFI_CONNECTION_HANDLE = 'uint32';
exports.FFI_UNSIGNED_INT = 'uint32';
exports.FFI_UNSIGNED_INT_PTR = ref.refType('uint32');
exports.FFI_STRING = 'string';
exports.FFI_STRING_DATA = 'string';
exports.FFI_CONNECTION_HANDLE_PTR = ref.refType(exports.FFI_CONNECTION_HANDLE);
exports.FFI_CXS_STATUS_PTR = ref.refType(exports.CxsStatus);
class CXSRuntimeConfig {
    constructor(_basepath) {
        this.basepath = _basepath;
    }
}
exports.CXSRuntimeConfig = CXSRuntimeConfig;
exports.FFIConfiguration = {
    // connection.rs
    cxs_connection_connect: [exports.FFI_ERROR_CODE, [exports.FFI_CONNECTION_HANDLE]],
    cxs_connection_create: [exports.FFI_ERROR_CODE, [exports.FFI_STRING_DATA, exports.FFI_CONNECTION_HANDLE_PTR]],
    cxs_connection_get_data: [exports.FFI_STRING_DATA, [exports.FFI_CONNECTION_HANDLE]],
    cxs_connection_get_state: [exports.FFI_ERROR_CODE, [exports.FFI_CONNECTION_HANDLE, exports.FFI_UNSIGNED_INT_PTR]],
    cxs_connection_list_state: [exports.FFI_ERROR_CODE, [exports.FFI_CXS_STATUS_PTR]],
    cxs_connection_release: [exports.FFI_ERROR_CODE, [exports.FFI_CONNECTION_HANDLE]],
    cxs_init: ['int', ['string']]
};
//# sourceMappingURL=rustlib.js.map