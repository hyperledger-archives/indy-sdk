import { CXSRuntime } from '../index'
import { CXSRuntimeConfig } from '../rustlib'

export function init_cxs (filename: string) {
  const config = new CXSRuntimeConfig(null)
  const rust = new CXSRuntime(config)
  return rust._ffi.cxs_init(filename)
}
