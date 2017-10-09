"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var ref = require("ref");
var weak = require("weak");
var index_1 = require("../index");
var rustlib_1 = require("../rustlib");
var Connection = /** @class */ (function () {
    function Connection(path) {
        this.initRustApi(path);
    }
    Connection.prototype.create = function (recipientInfo) {
        var connectionHandlePtr = ref.alloc(ref.types.uint32);
        var result = this.RUST_API.cxs_connection_create(recipientInfo, connectionHandlePtr);
        this.connectionHandle = ref.deref(connectionHandlePtr, ref.types.uint32);
        this.clearOnExit();
        return result;
    };
    Connection.prototype.connect = function () {
        return this.RUST_API.cxs_connection_connect(this.connectionHandle);
    };
    Connection.prototype.getData = function () {
        return this.RUST_API.cxs_connection_get_data(this.connectionHandle);
    };
    Connection.prototype.getState = function () {
        var statusPtr = ref.alloc(ref.types.uint32);
        var result = this.RUST_API.cxs_connection_get_state(this.connectionHandle, statusPtr);
        this.state = ref.deref(statusPtr, ref.types.uint32);
        return result;
    };
    Connection.prototype.release = function () {
        return this.RUST_API.cxs_connection_release(this.connectionHandle);
    };
    Connection.prototype.initRustApi = function (path) {
        this.RUST_API = new index_1.CXSRuntime(new rustlib_1.CXSRuntimeConfig(path)).ffi;
    };
    // clearOnExit creates a callback that will release the Rust Object
    // when the node Connection object is Garbage collected
    Connection.prototype.clearOnExit = function () {
        var weakRef = weak(this);
        var release = this.RUST_API.cxs_connection_release;
        var handle = this.connectionHandle;
        weak.addCallback(weakRef, function () {
            release(handle);
        });
    };
    return Connection;
}());
exports.Connection = Connection;
//# sourceMappingURL=connection.js.map