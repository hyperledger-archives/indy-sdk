import { Callback } from 'ffi'

import { VCXInternalError } from '../errors'
import { initRustAPI, rustAPI } from '../rustlib'
import { createFFICallbackPromise } from '../utils/ffi-helpers'
import { IInitVCXOptions } from './common'

export async function initVcx (configPath: string, options: IInitVCXOptions = {}): Promise<void> {
  initRustAPI(options.libVCXPath)
  let rc = null
  try {
    return await createFFICallbackPromise<void>(
      (resolve, reject, cb) => {
        rc = rustAPI().vcx_init(0, configPath, cb)
        if (rc) {
          reject(rc)
        }
      },
      (resolve, reject) => Callback(
        'void',
        ['uint32', 'uint32', 'string'],
        (xhandle: number, err: number) => {
          if (err) {
            reject(err)
            return
          }
          resolve()
        })
    )
  } catch (err) {
    throw new VCXInternalError(err)
  }
}

export async function initVcxWithConfig (config: string, options: IInitVCXOptions = {}): Promise<void> {
  initRustAPI(options.libVCXPath)
  let rc = null
  try {
    return await createFFICallbackPromise<void>(
      (resolve, reject, cb) => {
        rc = rustAPI().vcx_init_with_config(0, config, cb)
        if (rc) {
          reject(rc)
        }
      },
      (resolve, reject) => Callback(
        'void',
        ['uint32', 'uint32', 'string'],
        (xhandle: number, err: number) => {
          if (err) {
            reject(err)
            return
          }
          resolve()
        })
    )
  } catch (err) {
    throw new VCXInternalError(err)
  }
}
