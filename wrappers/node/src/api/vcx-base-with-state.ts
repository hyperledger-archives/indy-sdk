import * as ffi from 'ffi'
import { VCXInternalError } from '../errors'
import { createFFICallbackPromise, ICbRef } from '../utils/ffi-helpers'
import { StateType } from './common'
import { VCXBase } from './vcx-base'

export abstract class VCXBaseWithState<SerializedData> extends VCXBase<SerializedData> {
  protected abstract _updateStFn: (commandHandle: number, handle: string, cb: ICbRef) => number
  protected abstract _getStFn: (commandHandle: number, handle: string, cb: ICbRef) => number

  /**
   * @memberof VCXBaseWithState
   * @description Communicates with the agent service for polling and setting the state of the entity.
   * @async
   * @function updateState
   * @returns {Promise<void>}
   */
  public async updateState (): Promise<void> {
    try {
      const commandHandle = 0
      await createFFICallbackPromise<number>(
        (resolve, reject, cb) => {
          const rc = this._updateStFn(commandHandle, this.handle, cb)
          if (rc) {
            resolve(StateType.None)
          }
        },
        (resolve, reject) => ffi.Callback(
          'void',
          ['uint32', 'uint32', 'uint32'],
          (handle: number, err: any, state: StateType) => {
            if (err) {
              reject(err)
            }
            resolve(state)
          })
      )
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

  /**
   * @memberof VCXBaseWithState
   * @description Gets the state of the entity.
   * @async
   * @function getState
   * @returns {Promise<StateType>}
   */
  public async getState (): Promise<StateType> {
    try {
      const commandHandle = 0
      const stateRes = await createFFICallbackPromise<StateType>(
        (resolve, reject, cb) => {
          const rc = this._getStFn(commandHandle, this.handle, cb)
          if (rc) {
            resolve(StateType.None)
          }
        },
        (resolve, reject) => ffi.Callback(
          'void',
          ['uint32', 'uint32', 'uint32'],
          (handle: number, err: number, state: StateType) => {
            if (err) {
              reject(err)
            }
            resolve(state)
          })
      )
      return stateRes
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }
}
