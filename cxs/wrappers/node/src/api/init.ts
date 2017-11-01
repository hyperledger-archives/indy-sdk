import { CXSRuntime } from '../index'
import { CXSRuntimeConfig } from '../rustlib'
import * as ffi from 'ffi'

//command_handle: u32, config_path: *char, cb: (xcommand_handle: u32, err: u32)
export async function init_cxs (filename: string): Promise<number> {
  const config = new CXSRuntimeConfig(null)
  const rust = new CXSRuntime(config)
  return await new Promise<number>((resolve, reject) =>
      rust._ffi.cxs_init(
        0,
        filename,
        ffi.Callback('void', ['uint32', 'uint32'],
            (command_handle, err) => {
                resolve(err)
            }))
  )
}
