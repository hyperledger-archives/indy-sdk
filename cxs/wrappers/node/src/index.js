"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var rustlib_1 = require("./rustlib");
var ffi = require("ffi");
var CXSRuntimeConfig = (function () {
    function CXSRuntimeConfig(_basepath) {
        this.basepath = _basepath;
    }
    return CXSRuntimeConfig;
}());
exports.CXSRuntimeConfig = CXSRuntimeConfig;
var CXSRuntime = (function () {
    function CXSRuntime(config) {
        config = config || {};
        function _initialize_basepath() {
            // this needs additional logic
            var basepath = config.basepath;
            if (basepath === undefined) {
                basepath = "/usr/local/lib/node_modules/cxs/lib/libcxs.so";
            }
            return basepath;
        }
        // initialize FFI
        var libraryPath = _initialize_basepath();
        this.ffi = ffi.Library(libraryPath, rustlib_1.FFIConfiguration);
    }
    return CXSRuntime;
}());
exports.CXSRuntime = CXSRuntime;
