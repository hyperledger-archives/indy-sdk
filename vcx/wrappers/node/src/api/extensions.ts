import * as ffi from 'ffi'
import * as ref from 'ref'
import { VCXInternalError } from '../errors'
import { rustAPI } from '../rustlib'
import { createFFICallbackPromise } from '../utils/ffi-helpers'

function voidPtrToUint8Array (origPtr: any, length: number): Buffer {
    /**
     * Read the contents of the pointer and copy it into a new Buffer
     */
  const ptrType = ref.refType('uint8 *')
  const pointerBuf = ref.alloc(ptrType, origPtr)
  const newPtr = ref.readPointer(pointerBuf, 0, length)
  const newBuffer = Buffer.from(newPtr)
  return newBuffer
}

export interface IPackMessageData {
  data: Buffer,
  keys: string,
  sender: string
}

export interface IUnpackMessageData {
  data: Buffer,
}

export class Extensions {
  public getWalletHandle (): number {
    return rustAPI().vcx_wallet_get_handle()
  }

  public getPoolHandle (): number {
    return rustAPI().vcx_pool_get_handle()
  }

  /**
   * Pack message.
   *
   * Example:
   * ```
   * ```
   * @returns {Promise<string}
   */
  public async packMessage ({ data, keys, sender }: IPackMessageData): Promise<Buffer> {
    try {
      return await createFFICallbackPromise<Buffer>(
          (resolve, reject, cb) => {
            const rc = rustAPI().vcx_pack_message(0, 0,
              ref.address(data), data.length, keys, sender, cb)
            if (rc) {
              reject(rc)
            }
          },
          (resolve, reject) => ffi.Callback(
            'void',
            ['uint32', 'uint32', 'pointer', 'uint32'],
            (xHandle: number, err: number, details: any, length: number) => {
              if (err) {
                reject(err)
                return
              }
              if (!details) {
                reject(`returned empty buffer`)
                return
              }
              const newBuffer = voidPtrToUint8Array(details, length)
              resolve(newBuffer)
            })
        )
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

 /**
  * Unpack message.
  *
  * Example:
  * ```
  * ```
  * @returns {Promise<string}
  */
  public async unpackMessage (unpackData: IUnpackMessageData): Promise<Buffer> {
    try {
      return await createFFICallbackPromise<Buffer>(
          (resolve, reject, cb) => {
            const rc = rustAPI().vcx_unpack_message(0, 0,
              ref.address(unpackData.data), unpackData.data.length, cb)
            if (rc) {
              reject(rc)
            }
          },
          (resolve, reject) => ffi.Callback(
            'void',
            ['uint32', 'uint32', 'pointer', 'uint32'],
            (xHandle: number, err: number, details: any, length: number) => {
              if (err) {
                reject(err)
                return
              }
              if (!details) {
                reject(`returned empty buffer`)
                return
              }
              const newBuffer = voidPtrToUint8Array(details, length)
              resolve(newBuffer)
            })
        )
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }
}