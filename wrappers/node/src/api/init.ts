import { Callback } from 'ffi'

import { VCXInternalError } from '../errors'
import { initRustAPI, rustAPI } from '../rustlib'
import { createFFICallbackPromise } from '../utils/ffi-helpers'

export interface IInitVCXOptions {
  libVCXPath?: string
}

export async function initVcx (configPath: string, options: IInitVCXOptions = {}): Promise<void> {
  initRustAPI(options.libVCXPath)
  let rc = null
  try {
    return await createFFICallbackPromise<void>(
      (resolve, reject, cb) => {
        rc = rustAPI().vcx_init(0, configPath, cb)
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
    throw new VCXInternalError(err, 'vcx_init')
  }
}
