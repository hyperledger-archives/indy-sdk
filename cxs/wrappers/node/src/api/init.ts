import * as posix from 'path'
import { CXSRuntime } from '../index'
import { CXSRuntimeConfig } from '../rustlib'

export function init_cxs (filename) {
  const config = new CXSRuntimeConfig(null)
  const rust = new CXSRuntime(config)
  return rust.ffi.cxs_init(filename)
}
