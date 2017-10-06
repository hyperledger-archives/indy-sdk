"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var index = require("../index");
var posix = require("path");
function init_cxs(filename) {
    var currentDir = posix.dirname(".");
    var path = currentDir + '/../node/lib/libcxs.so';
    var config = new index.CXSRuntimeConfig(path);
    var rust = new index.CXSRuntime(config);
    return rust.ffi.cxs_init(filename);
}
exports.init_cxs = init_cxs;
//# sourceMappingURL=init.js.map