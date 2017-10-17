"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const ref = require("ref");
const weak = require("weak");
const index_1 = require("../index");
const rustlib_1 = require("../rustlib");
class Connection {
    constructor(path) {
        this.initRustApi(path);
    }
    create(recipientInfo) {
        const connectionHandlePtr = ref.alloc(ref.types.uint32);
        const result = this.RUST_API.cxs_connection_create(recipientInfo, connectionHandlePtr);
        this.connectionHandle = ref.deref(connectionHandlePtr, ref.types.uint32);
        this.clearOnExit();
        return result;
    }
    connect() {
        return this.RUST_API.cxs_connection_connect(this.connectionHandle);
    }
    getData() {
        return this.RUST_API.cxs_connection_get_data(this.connectionHandle);
    }
    getState() {
        const statusPtr = ref.alloc(ref.types.uint32);
        const result = this.RUST_API.cxs_connection_get_state(this.connectionHandle, statusPtr);
        this.state = ref.deref(statusPtr, ref.types.uint32);
        return result;
    }
    release() {
        return this.RUST_API.cxs_connection_release(this.connectionHandle);
    }
    initRustApi(path) {
        this.RUST_API = new index_1.CXSRuntime(new rustlib_1.CXSRuntimeConfig(path)).ffi;
    }
    // clearOnExit creates a callback that will release the Rust Object
    // when the node Connection object is Garbage collected
    clearOnExit() {
        const weakRef = weak(this);
        const release = this.RUST_API.cxs_connection_release;
        const handle = this.connectionHandle;
        weak.addCallback(weakRef, () => {
            release(handle);
        });
    }
}
exports.Connection = Connection;
//# sourceMappingURL=connection.js.map