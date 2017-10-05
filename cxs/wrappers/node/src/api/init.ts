import * as index from '../index'
import * as posix from 'path'

export function init_cxs (poolName, configName, walletName, walletType) {
    var currentDir = posix.dirname(this.filename)
    var path = currentDir + '/../node/lib/libcxs.so'
    var config = new index.CXSRuntimeConfig(path)
    var rust = new index.CXSRuntime(config)
    return rust.ffi.cxs_init(poolName, configName,walletName,walletType)
}
