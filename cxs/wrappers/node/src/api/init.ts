import * as ffi from 'ffi'
import { CXSRuntime } from '../index'
import { CXSRuntimeConfig } from '../rustlib'

export async function init_cxs (filename: string): Promise<number> {
  const config = new CXSRuntimeConfig(null)
  const rust = new CXSRuntime(config)
  return await new Promise<number>((resolve, reject) =>
      rust._ffi.cxs_init(
        0,
        filename,
        ffi.Callback('void', ['uint32', 'uint32'],
          (xhandle, err) => {
            resolve(err)
          }))
  )
}
