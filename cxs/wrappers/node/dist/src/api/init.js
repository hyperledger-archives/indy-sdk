"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var index = require("../index");
var posix = require("path");
function init_cxs(poolName, configName, walletName, walletType) {
    var currentDir = posix.dirname(this.filename);
    var path = currentDir + '/../node/lib/libcxs.so';
    var config = new index.CXSRuntimeConfig(path);
    var rust = new index.CXSRuntime(config);
    return rust.ffi.cxs_init(poolName, configName, walletName, walletType);
}
exports.init_cxs = init_cxs;
//# sourceMappingURL=init.js.map