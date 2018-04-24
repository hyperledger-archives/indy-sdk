import { Callback } from 'ffi'

import { VCXInternalError } from '../errors'
import { rustAPI } from '../rustlib'
import { createFFICallbackPromise } from '../utils/ffi-helpers'
import { VCXBase } from './VCXBase'

export async function provisionAgent (options: string): Promise<string> {
  try {
    return await createFFICallbackPromise<string>(
      (resolve, reject, cb) => {
        const rc = rustAPI().vcx_agent_provision_async(0, options, cb)
        if (rc) {
          reject(rc)
        }
      },
      (resolve, reject) => Callback('void', ['uint32','uint32','string'], (xhandle, err, config) => {
        if (err) {
          reject(err)
          return
        } else {
          resolve(config)
        }
      })
    )
  } catch (err) {
    throw new VCXInternalError(err, VCXBase.errorMessage(err), 'vcx_provision_agent_async')
  }
}

export async function updateAgentInfo (options: string): Promise<string> {
  try {
    return await createFFICallbackPromise<string>(
      (resolve, reject, cb) => {
        const rc = rustAPI().vcx_agent_update_info(0, options, cb)
        if (rc) {
          reject(rc)
        }
      },
      (resolve, reject) => Callback('void', ['uint32','uint32','string'], (xhandle, err, config) => {
        if (err) {
          reject(err)
          return
        } else {
          resolve(config)
        }
      })
    )
  } catch (err) {
    throw new VCXInternalError(err, VCXBase.errorMessage(err), 'vcx_update_agent_info')
  }
}

export function getVersion (): string {
  return rustAPI().vcx_version()
}
