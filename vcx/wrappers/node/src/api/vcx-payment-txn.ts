import * as ffi from 'ffi'
import { VCXInternalError } from '../errors'
import { createFFICallbackPromise } from '../utils/ffi-helpers'
import { IUTXO } from './common'

export interface IPaymentTxn {
  amount: number,
  inputs: string[],
  outputs: IUTXO[]
}

export type Constructor<T> = new(...args: any[]) => T

export interface IVCXPaymentTxnRes {
  getPaymentTxn: () => Promise<IPaymentTxn>
}

export const VCXPaymentTxn = <T extends Constructor<{ handle: string }>>(Base: T):
  T & Constructor<IVCXPaymentTxnRes> => {
  class BasePaymentTxn extends Base implements IVCXPaymentTxnRes {
    public async getPaymentTxn (): Promise<IPaymentTxn> {
      try {
        const paymentTxnStr = await createFFICallbackPromise<string>(
            (resolve, reject, cb) => {
              // Can not really enforce presence of _getPaymentTxnFn
              const rc = (this as any)._getPaymentTxnFn(0, this.handle, cb)
              if (rc) {
                reject(rc)
              }
            },
            (resolve, reject) => ffi.Callback('void', ['uint32', 'uint32', 'string'],
            (xcommandHandle: number, err: number, info: any) => {
              if (err) {
                reject(err)
                return
              }
              resolve(info)
            })
          )
        const paymentTxn = JSON.parse(paymentTxnStr)
        return paymentTxn
      } catch (err) {
        throw new VCXInternalError(err)
      }
    }
  }
  return BasePaymentTxn
}
