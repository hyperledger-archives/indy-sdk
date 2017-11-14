import { Callback } from 'ffi'
import { CXSRuntime } from '../index'
import { CXSRuntimeConfig } from '../rustlib'
import { createFFICallbackPromise } from './api'
import { CXSInternalError } from './errors'

export async function init_cxs (filename: string): Promise<void> {
  const config = new CXSRuntimeConfig(null)
  const rust = new CXSRuntime(config)
  let rc = null
  try {
    return await createFFICallbackPromise<void>(
      (resolve, reject, cb) => {
        rc = rust._ffi.cxs_init(0, filename, cb)
        if (rc) {
          reject(rc)
        } else {
          resolve(rc)
        }
      },
      (resolve, reject) => Callback('void', ['uint32', 'uint32', 'string'], (xhandle, err) => {
        if (err) {
          reject(err)
          return
        }
        resolve(err)
      })
    )
  } catch (err) {
    throw new CXSInternalError(`cxs_init -> ${err}`)
  }
}
