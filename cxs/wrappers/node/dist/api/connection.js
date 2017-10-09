"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const ref = require("ref");
const index_1 = require("../index");
const rustlib_1 = require("../rustlib");
class Connection {
    constructor(path) {
        this.RUST_API = new index_1.CXSRuntime(new rustlib_1.CXSRuntimeConfig(path)).ffi;
    }
    create(recipientInfo) {
        const connectionHandlePtr = ref.alloc(ref.types.uint32);
        const result = this.RUST_API.cxs_connection_create(recipientInfo, connectionHandlePtr);
        this.connectionHandle = ref.deref(connectionHandlePtr, ref.types.uint32);
        return result;
    }
    connect() {
        return this.RUST_API.cxs_connection_connect(this.connectionHandle);
    }
    get_data() {
        return this.RUST_API.cxs_connection_get_data(this.connectionHandle);
    }
    get_state() {
        const statusPtr = ref.alloc(ref.types.uint32);
        const result = this.RUST_API.cxs_connection_get_state(this.connectionHandle, statusPtr);
        this.state = ref.deref(statusPtr, ref.types.uint32);
        return result;
    }
    release() {
        return this.RUST_API.cxs_connection_release(this.connectionHandle);
    }
    list_state() {
        const CxsStatusPtr = ref.alloc(rustlib_1.FFI_CXS_STATUS_PTR);
        const result = this.RUST_API.cxs_connection_list_state(CxsStatusPtr);
        this.statusList = ref.deref(CxsStatusPtr, rustlib_1.FFI_CXS_STATUS_PTR);
        return result;
    }
}
exports.Connection = Connection;
//# sourceMappingURL=connection.js.map