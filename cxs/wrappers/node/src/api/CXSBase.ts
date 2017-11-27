import * as ffi from 'ffi'
import { createFFICallbackPromise } from '../utils/ffi-helpers'
import { GCWatcher } from '../utils/memory-management-helpers'
import { StateType } from './common'

export abstract class CXSBase extends GCWatcher {
  protected abstract _updateStFn: any
  protected abstract _serializeFn: any
  protected abstract _deserializeFn: any
  protected _handle: string
  protected _sourceId: string
  protected _state: StateType
  protected _attr: string

  constructor () {
    super()
    this._handle = null
    this._state = StateType.None
    this._sourceId = null
    this._attr = null
  }

  static async deserialize (objType, objData): Promise<any> {
    const obj = new objType()
    await obj._initFromData(objData)
    await obj._updateState()
    return obj
  }

  async abstract updateState ()

  async abstract serialize ()

  async _create (createFn): Promise<void> {
    await this._init(createFn)
  }

  get state (): number {
    return this._state
  }

  get handle () {
    return this._handle
  }

  get sourceId () {
    return this._sourceId
  }

  get attr () {
    return this._attr
  }

  protected async _updateState (): Promise<void> {
    const commandHandle = 0
    const state = await createFFICallbackPromise<number>(
      (resolve, reject, cb) => {
        const rc = this._updateStFn(commandHandle, this._handle, cb)
        if (rc) {
          resolve(StateType.None)
        }
      },
      (resolve, reject) => ffi.Callback('void', ['uint32', 'uint32', 'uint32'], (handle, err, _state) => {
        if (err) {
          reject(err)
        }
        resolve(_state)
      })
    )
    this._state = state
  }

  protected async _serialize (): Promise<string> {
    const serializeHandle = this._handle
    let rc = null
    const data = await createFFICallbackPromise<string>(
        (resolve, reject, cb) => {
          rc = this._serializeFn(0, serializeHandle, cb)
          if (rc) {
            // TODO: handle correct exception
            reject(rc)
          }
        },
        (resolve, reject) => ffi.Callback('void', ['uint32', 'uint32', 'string'], (handle, err, serializedData) => {
          if (err) {
            reject(err)
            return
          } else if (serializedData == null) {
            reject('no data to serialize')
          }
          resolve(serializedData)
        })
    )
    return data
  }

  private async _init (createFn): Promise<void> {
    const handle = await createFFICallbackPromise<string>(
        (resolve, reject, cb) => {
          const rc = createFn(cb)
          if (rc) {
            reject(rc)
          }
        },
        (resolve, reject) => ffi.Callback('void', ['uint32', 'uint32', 'uint32'], (xHandle, err, rtnHandle) => {
          if (err) {
            reject(err)
            return
          }
          resolve( rtnHandle )
        })
    )
    super._setHandle(handle)
    await this._updateState()
  }

  private async _initFromData (objData): Promise<void> {
    const commandHandle = 0
    const objHandle = await createFFICallbackPromise<string>(
        (resolve, reject, cb) => {
          const rc = this._deserializeFn(commandHandle, JSON.stringify(objData), cb)
          if (rc) {
            reject(rc)
          }
        },
        (resolve, reject) => ffi.Callback('void', ['uint32', 'uint32', 'uint32'], (xHandle, _rc, handle) => {
          if (_rc) {
            reject(_rc)
          }
          const value = JSON.stringify(handle)
          resolve(Number(value))
        })
    )
    super._setHandle(objHandle)
  }

}
