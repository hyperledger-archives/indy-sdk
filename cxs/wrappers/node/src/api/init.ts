import { CXSRuntime } from '../index'
import { CXSRuntimeConfig } from '../rustlib'

//command_handle: u32, config_path: *char, cb: (xcommand_handle: u32, err: u32)
export function init_cxs (filename: string) {
  const config = new CXSRuntimeConfig(null)
  const rust = new CXSRuntime(config)
  return rust._ffi.cxs_init(filename)
}
