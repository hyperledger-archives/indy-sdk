import { Callback } from 'ffi'

import { VCXInternalError } from '../errors'
import { initRustAPI, rustAPI } from '../rustlib'
import { createFFICallbackPromise } from '../utils/ffi-helpers'
import { IInitVCXOptions } from './common'

export async function provisionAgent (configAgent: string, options: IInitVCXOptions = {}): Promise<string> {
  try {
    initRustAPI(options.libVCXPath)
    return await createFFICallbackPromise<string>(
      (resolve, reject, cb) => {
        const rc = rustAPI().vcx_agent_provision_async(0, configAgent, cb)
        if (rc) {
          reject(rc)
        }
      },
      (resolve, reject) => Callback(
        'void',
        ['uint32','uint32','string'],
        (xhandle: number, err: number, config: string) => {
          if (err) {
            reject(err)
            return
          }
          resolve(config)
        })
    )
  } catch (err) {
    throw new VCXInternalError(err)
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
      (resolve, reject) => Callback(
        'void',
        ['uint32','uint32','string'],
        (xhandle: number, err: number, config: string) => {
          if (err) {
            reject(err)
            return
          }
          resolve(config)
        })
    )
  } catch (err) {
    throw new VCXInternalError(err)
  }
}

export function getVersion (): string {
  return rustAPI().vcx_version()
}

export async function getLedgerFees (): Promise<string> {
  try {
    const ledgerFees = await createFFICallbackPromise<string>(
      (resolve, reject, cb) => {
        const rc = rustAPI().vcx_ledger_get_fees(0, cb)
        if (rc) {
          reject(rc)
        }
      },
      (resolve, reject) => Callback(
        'void',
        ['uint32','uint32','string'],
        (xhandle: number, err: number, fees: string) => {
          if (err) {
            reject(err)
            return
          }
          resolve(fees)
        })
    )
    return ledgerFees
  } catch (err) {
    throw new VCXInternalError(err)
  }
}

export function shutdownVcx (deleteWallet: boolean): number {
  return rustAPI().vcx_shutdown(deleteWallet)
}

export interface IUpdateInstitutionConfigs {
  name: string,
  logoUrl: string
}
export function updateInstitutionConfigs ({ name, logoUrl }: IUpdateInstitutionConfigs): number {
  const rc = rustAPI().vcx_update_institution_info(name, logoUrl)
  if (rc) {
    throw new VCXInternalError(rc)
  }
  return rc
}
