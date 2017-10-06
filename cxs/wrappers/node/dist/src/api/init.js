"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var posix = require("path");
var index_1 = require("../index");
var rustlib_1 = require("../rustlib");
function init_cxs(filename) {
    var currentDir = posix.dirname('.');
    var path = currentDir + '/../node/lib/libcxs.so';
    var config = new rustlib_1.CXSRuntimeConfig(path);
    var rust = new index_1.CXSRuntime(config);
    return rust.ffi.cxs_init(filename);
}
exports.init_cxs = init_cxs;
//# sourceMappingURL=init.js.map