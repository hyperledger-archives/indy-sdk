import { Callback } from 'ffi'

import { VCXInternalError } from '../errors'
import { rustAPI } from '../rustlib'
import { createFFICallbackPromise } from '../utils/ffi-helpers'
import { IndyTransactions } from './common'
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
    throw new VCXInternalError(err, VCXBase.errorMessage(err), 'vcx_update_agent_info')
  }
}

export function getVersion (): string {
  return rustAPI().vcx_version()
}

export type ILedgerFees = {
  [P in IndyTransactions]: number
}
export async function getLedgerFees (): Promise<ILedgerFees> {
  try {
    const ledgerFeesStr = await createFFICallbackPromise<string>(
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
    const ledgerFees: ILedgerFees = JSON.parse(ledgerFeesStr)
    return ledgerFees
  } catch (err) {
    throw new VCXInternalError(err, VCXBase.errorMessage(err), 'vcx_ledger_get_fees')
  }
}

export function shutdownVcx (deleteWallet: boolean): number {
  return rustAPI().vcx_shutdown(deleteWallet)
}

export function updateInstitutionConfigs (name: string, logoUrl: string): number {
  const rc = rustAPI().vcx_update_institution_info(name, logoUrl)
  if (rc) {
    throw new VCXInternalError(rc, VCXBase.errorMessage(rc), 'vcx_update_institution_info')
  }
  return rc
}
