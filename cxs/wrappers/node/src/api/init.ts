import * as ffi from 'ffi'
import { CXSRuntime } from '../index'
import { CXSRuntimeConfig } from '../rustlib'
import { CXSInternalError } from './errors'

export async function init_cxs (filename: string): Promise<number> {
  const config = new CXSRuntimeConfig(null)
  const rust = new CXSRuntime(config)
  try {
    return await new Promise<number>((resolve, reject) => {
      const rc = rust._ffi.cxs_init(
            0,
            filename,
            ffi.Callback('void', ['uint32', 'uint32'],
                (xhandle, err) => {
                  if (err) {
                    reject(err)
                  }
                }))
      if (rc) {
        reject(rc)
      } else {
        resolve(rc)
      }
    })
  } catch (err) {
    throw new CXSInternalError(`cxs_init -> ${err}`)
  }
}
