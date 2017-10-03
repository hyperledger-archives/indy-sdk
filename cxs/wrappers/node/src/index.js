"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var rustlib_1 = require("./rustlib");
var ffi = require("ffi");
var path = require("path");
var CXSRuntimeConfig = (function () {
    function CXSRuntimeConfig(_basepath) {
        this.basepath = _basepath;
    }
    return CXSRuntimeConfig;
}());
exports.CXSRuntimeConfig = CXSRuntimeConfig;
/** CXSRuntime is the object that interfaces with the cxs sdk functions **/
/** FFIConfiguration will contain all the sdk api functions **/
/** CXSRuntimeConfg is a class that currently only contains a chosen basepath for the .so file
 * I made it a class just in case we think of more needed configs **/
var CXSRuntime = (function () {
    function CXSRuntime(config) {
        config = config || {};
        function _initialize_basepath() {
            // this needs additional logic
            var basepath = config.basepath;
            if (basepath === undefined) {
                //This basepath is in the local/appSpecific node_modules
                basepath = path.resolve("../node_modules/cxs/lib/libcxs.so");
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
