"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var ref = require("ref");
var index_1 = require("../index");
var rustlib_1 = require("../rustlib");
var Connection = /** @class */ (function () {
    function Connection(path) {
        this.RUST_API = new index_1.CXSRuntime(new rustlib_1.CXSRuntimeConfig(path)).ffi;
    }
    Connection.prototype.create = function (recipientInfo) {
        var connectionHandlePtr = ref.alloc(ref.types.uint32);
        var result = this.RUST_API.cxs_connection_create(recipientInfo, connectionHandlePtr);
        this.connectionHandle = ref.deref(connectionHandlePtr, ref.types.uint32);
        return result;
    };
    Connection.prototype.connect = function () {
        return this.RUST_API.cxs_connection_connect(this.connectionHandle);
    };
    Connection.prototype.get_data = function () {
        return this.RUST_API.cxs_connection_get_data(this.connectionHandle);
    };
    Connection.prototype.get_state = function () {
        var statusPtr = ref.alloc(ref.types.uint32);
        var result = this.RUST_API.cxs_connection_get_state(this.connectionHandle, statusPtr);
        this.state = ref.deref(statusPtr, ref.types.uint32);
        return result;
    };
    Connection.prototype.release = function () {
        return this.RUST_API.cxs_connection_release(this.connectionHandle);
    };
    Connection.prototype.list_state = function () {
        var CxsStatusPtr = ref.alloc(rustlib_1.FFI_CXS_STATUS_PTR);
        var result = this.RUST_API.cxs_connection_list_state(CxsStatusPtr);
        this.statusList = ref.deref(CxsStatusPtr, rustlib_1.FFI_CXS_STATUS_PTR);
        return result;
    };
    return Connection;
}());
exports.Connection = Connection;
//# sourceMappingURL=connection.js.map