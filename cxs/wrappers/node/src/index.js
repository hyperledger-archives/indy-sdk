"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var rustlib_1 = require("./rustlib");
var ffi = require("ffi");
var CXSRuntimeConfig = /** @class */ (function () {
    function CXSRuntimeConfig(_basepath) {
        this.basepath = _basepath;
    }
    return CXSRuntimeConfig;
}());
var CXSRuntime = /** @class */ (function () {
    function CXSRuntime(config) {
        config = config || {};
        function _initialize_basepath() {
            // this needs additional logic
            var basepath = config.basepath;
            if (basepath === undefined) {
                basepath = "/node_modules/cxs/lib/libcxs.so";
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
// var basepath = "/node_modules/cxs/lib/libcxs.so"
// var basepath = "/home/rmarsh/dev/cxs/cxs/wrappers/node/lib/libcxs.so";
var run = new CXSRuntime(new CXSRuntimeConfig());
// var run = new CXSRuntime(new CXSRuntimeConfig(basepath))
run.ffi.cxs_init();
