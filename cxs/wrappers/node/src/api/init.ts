import * as posix from 'path'
import { CXSRuntime } from '../index'
import { CXSRuntimeConfig } from '../rustlib'

export function init_cxs (filename) {
  const currentDir = posix.dirname('.')
  const path = currentDir + '/../node/lib/libcxs.so'
  const config = new CXSRuntimeConfig(path)
  const rust = new CXSRuntime(config)
  return rust.ffi.cxs_init(filename)
}
