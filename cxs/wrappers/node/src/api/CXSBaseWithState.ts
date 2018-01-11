import * as ffi from 'ffi'
import { createFFICallbackPromise } from '../utils/ffi-helpers'
import { StateType } from './common'
import { CXSBase } from './CXSBase'

export abstract class CXSBaseWithState extends CXSBase {
  protected abstract _updateStFn: any
  protected _state: StateType

  constructor (sourceId) {
    super(sourceId)
    this._state = StateType.None
  }

  async abstract updateState ()

  get state (): number {
    return this._state
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
}
