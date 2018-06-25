import * as ffi from 'ffi'
import { VCXInternalError } from '../errors'
import { createFFICallbackPromise, ICbRef } from '../utils/ffi-helpers'
import { GCWatcher } from '../utils/memory-management-helpers'

export type IVCXBaseCreateFn = (cb: ICbRef) => number

export abstract class VCXBase<SerializedData> extends GCWatcher {
  public static async _deserialize<T extends VCXBase<any> = any, P = object> (
    VCXClass: new(sourceId: string, ...args: any[]) => T,
    objData: { source_id: string },
    constructorParams?: P
  ): Promise<T> {
    try {
      const obj = new VCXClass(objData.source_id, constructorParams)
      await obj._initFromData(objData)
      return obj
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

  protected abstract _serializeFn: (commandHandle: number, handle: string, cb: ICbRef) => number
  protected abstract _deserializeFn: (commandHandle: number, handle: string, cb: ICbRef) => number
  protected _sourceId: string

  constructor (sourceId: string) {
    super()
    this._sourceId = sourceId
  }

  /**
   * @memberof VCXBase
   * @description Serializes an enitity.
   * Data returned can be used to recreate an entity by passing it to the deserialize function.
   * @async
   * @function serialize
   * @returns {Promise<SerializedData>} - Json object with all of the underlying Rust attributes.
   * Same json object structure that is passed to the deserialize function.
   */
  public async serialize (): Promise<SerializedData> {
    try {
      const dataStr = await createFFICallbackPromise<string>(
        (resolve, reject, cb) => {
          const rc = this._serializeFn(0, this.handle, cb)
          if (rc) {
            reject(rc)
            return
          }
        },
        (resolve, reject) => ffi.Callback(
          'void',
          ['uint32', 'uint32', 'string'],
          (handle: string, err: any, serializedData?: string) => {
            if (err) {
              reject(err)
              return
            }
            if (!serializedData) {
              reject('no data to serialize')
              return
            }
            resolve(serializedData)
          }
        )
      )
      const data: SerializedData = JSON.parse(dataStr)
      return data
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

  get sourceId () {
    return this._sourceId
  }

  protected async _create (createFn: IVCXBaseCreateFn): Promise<void> {
    const handleRes = await createFFICallbackPromise<string>(
        (resolve, reject, cb) => {
          const rc = createFn(cb)
          if (rc) {
            reject(rc)
          }
        },
        (resolve, reject) => ffi.Callback(
          'void',
          ['uint32', 'uint32', 'uint32'],
          (xHandle: number, err: number, handle: number) => {
            if (err) {
              reject(err)
              return
            }
            const handleStr = handle.toString()
            resolve(handleStr)
          })
    )
    super._setHandle(handleRes)
  }

  private async _initFromData (objData: object): Promise<void> {
    const commandHandle = 0
    const objHandle = await createFFICallbackPromise<string>(
        (resolve, reject, cb) => {
          const rc = this._deserializeFn(commandHandle, JSON.stringify(objData), cb)
          if (rc) {
            reject(rc)
          }
        },
        (resolve, reject) => ffi.Callback(
          'void',
          ['uint32', 'uint32', 'uint32'],
          (xHandle: number, err: number, handle: number) => {
            if (err) {
              reject(err)
            }
            const handleStr = handle.toString()
            resolve(handleStr)
          })
    )
    super._setHandle(objHandle)
  }

}
