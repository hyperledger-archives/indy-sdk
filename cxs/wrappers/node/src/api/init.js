
var index = require('../index')
var parentDir = require('path');


function init_cxs () {
    var currentDir = parentDir.dirname(module.filename)
    var path = '/home/mark/dev/sdk/cxs/wrappers/node/lib/libcxs.so'
    
    var config = new index.CXSRuntimeConfig(path)
    var rust = new index.CXSRuntime(config)
    return rust.ffi.cxs_init('pool1', 'config1', 'wallet1', 'default')
}
module.exports = init_cxs;