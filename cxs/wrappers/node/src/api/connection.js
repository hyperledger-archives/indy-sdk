"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var index = require("../index");
var ref = require("ref");
var Connection = (function () {
    function Connection(path) {
        this.RustAPI = new index.CXSRuntime(new index.CXSRuntimeConfig(path)).ffi;
    }
    Connection.prototype.connection_create = function (recipient_info, connection_handle) {
        var connection_handle_ptr = connection_handle;
        if (typeof connection_handle == "number") {
            // console.log(connection_handle)
            connection_handle_ptr = ref.alloc(ref.types.int, connection_handle);
            var old_handle = ref.alloc(ref.types.int, connection_handle);
            // console.log(connection_handle_ptr)
        }
        var result = this.RustAPI.cxs_connection_create(recipient_info, connection_handle_ptr);
        // console.log(old_handle)
        // console.log(connection_handle_ptr)
        // console.log(connection_handle_ptr == old_handle)
        this.connection_handle = ref.deref(connection_handle_ptr);
        return result;
    };
    Connection.prototype.connection_connect = function (connection_handle) {
        return this.RustAPI.cxs_connection_connect(connection_handle);
    };
    Connection.prototype.connection_get_data = function (connection_handle) {
        return this.RustAPI.cxs_connection_get_data(connection_handle);
    };
    return Connection;
}());
exports.Connection = Connection;
//
//
// export function connection_create(): Errorcode{
//
//     return Errorcode.Failure
// }
//
// export function connection_connect(): Errorcode{
//
//     return Errorcode.Failure
// }
//
// export function connection_get_data(): Errorcode{
//
//     return Errorcode.Failure
// }
//
// export function connection_get_state(): Errorcode{
//
//     return Errorcode.Failure
// }
//
// export function connection_list_state(): Errorcode{
//
//     return Errorcode.Failure
// } 
