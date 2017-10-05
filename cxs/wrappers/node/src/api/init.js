
var index = require('../index')
var parentDir = require('path');


function init_cxs (poolName, configName, walletName, walletType) {
    var currentDir = parentDir.dirname(module.filename)
    var path = currentDir + '/../../lib/libcxs.so'
    var config = new index.CXSRuntimeConfig(path)
    var rust = new index.CXSRuntime(config)
    return rust.ffi.cxs_init(poolName, configName,walletName,walletType)
}
module.exports = init_cxs;