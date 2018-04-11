import * as ffi from 'ffi'
import { createFFICallbackPromise } from '../utils/ffi-helpers'
import { StateType } from './common'
import { VCXBase } from './VCXBase'

export abstract class VCXBaseWithState extends VCXBase {
  protected abstract _updateStFn: any
  protected abstract _getStFn: any

  constructor (sourceId) {
    super(sourceId)
  }

  async abstract updateState ()

  async abstract getState ()

  protected async _updateState (): Promise<void> {
    const commandHandle = 0
    await createFFICallbackPromise<number>(
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
  }

  protected async _getState (): Promise<StateType> {
    const commandHandle = 0
    return await createFFICallbackPromise<number>(
      (resolve, reject, cb) => {
        const rc = this._getStFn(commandHandle, this._handle, cb)
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
  }
}
