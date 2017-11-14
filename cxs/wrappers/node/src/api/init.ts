import { Callback } from 'ffi'

import { CXSInternalError } from '../errors'
import { initRustAPI, rustAPI } from '../rustlib'
import { createFFICallbackPromise } from '../utils/ffi-helpers'

export interface IInitCXSOptions {
  libCXSPath?: string
}

export async function initCxs (configPath: string, options: IInitCXSOptions = {}): Promise<void> {
  initRustAPI(options.libCXSPath)
  let rc = null
  try {
    return await createFFICallbackPromise<void>(
      (resolve, reject, cb) => {
        rc = rustAPI().cxs_init(0, configPath, cb)
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
