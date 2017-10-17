"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const ffi = require("ffi");
const path = require("path");
const rustlib_1 = require("./rustlib");
// CXSRuntime is the object that interfaces with the cxs sdk functions
// FFIConfiguration will contain all the sdk api functions
// CXSRuntimeConfg is a class that currently only contains a chosen basepath for the .so file
// I made it a class just in case we think of more needed configs
class CXSRuntime {
    constructor(config) {
        config = config || {};
        function _initialize_basepath() {
            let basepath = config.basepath;
            if (basepath === undefined || basepath === null) {
                // This basepath is in the local/appSpecific node_modules
                basepath = path.resolve(__dirname, '../lib/libcxs.so');
            }
            return basepath;
        }
        // initialize FFI
        const libraryPath = _initialize_basepath();
        this.ffi = ffi.Library(libraryPath, rustlib_1.FFIConfiguration);
    }
}
exports.CXSRuntime = CXSRuntime;
var init_1 = require("./api/init");
exports.init_cxs = init_1.init_cxs;
var connection_1 = require("./api/connection");
exports.Connection = connection_1.Connection;
var rustlib_2 = require("./rustlib");
exports.CXSRuntimeConfig = rustlib_2.CXSRuntimeConfig;
//# sourceMappingURL=index.js.map