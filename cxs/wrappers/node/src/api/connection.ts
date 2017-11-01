import * as ffi from 'ffi'
import { alloc, deref, Type, types as refTypes } from 'ref'
import * as weak from 'weak'
import { CXSRuntime } from '../index'
import { CXSRuntimeConfig } from '../rustlib'
import {
    IConnectionData,
    IConnections,
    IConnectOptions,
    IRecipientInfo,
    StateType
} from './api'
import { ConnectionTimeoutError, CXSInternalError } from './errors'

export class Connection implements IConnections {
  public connectionHandle: string
  public state: StateType
  private RUST_API: { [ index: string ]: ffi.ForeignFunction }

  constructor ( path?: string ) {
    this._initRustApi(path)
  }

  async create ( recipientInfo: IRecipientInfo ): Promise<number> {
    // const myDid = recipientInfo.DIDself !== undefined ? recipientInfo.DIDself : null
    // const theirDid = recipientInfo.DIDremote !== undefined ? recipientInfo.DIDremote : null
    const id = recipientInfo.id // TODO verifiy that id is a string
    let result = null
    try{
      this.connectionHandle = await new Promise<string>((resolve, reject) =>
          result = this.RUST_API.cxs_connection_create(
              0,
              id,
              ffi.Callback('void', ['uint32', 'uint32', 'uint32'],
                  (command_handle, err, _connection_handle) => {
                      if (err) {
                          reject(err)
                          return
                      }
                      resolve(_connection_handle)
                  }))
      )
    } catch (error) {
        result = error
    }
    this._clearOnExit()

    return result
  }

  async connect ( options: IConnectOptions = {} ): Promise<void> {
    const timeout = options.timeout || 10000
    await this._waitFor(async () => await this._connect(options) === 0, timeout)
  }

  async getData (): Promise<IConnectionData> {
    const data = await new Promise<string>((resolve, reject) =>
      this.RUST_API.cxs_connection_serialize(
          this.connectionHandle,
            ffi.Callback('void', ['uint32', 'uint32', 'string'],
              (handle, err, _data) => {
                if (err) {
                  reject(err)
                  return
                }
                if (_data === '') {
                  resolve(null)
                } else {
                  resolve(_data)
                }
              }))
    )
    return JSON.parse(data)
  }

  getState (): StateType {
    const statusPtr = alloc(refTypes.uint32)
    const result = this.RUST_API.cxs_connection_get_state(this.connectionHandle, statusPtr)
    if (result) {
      throw new CXSInternalError(`cxs_connection_get_state -> ${result}`)
    }
    this.state = deref(statusPtr)
    return this.state
  }

  release (): number {
    return this.RUST_API.cxs_connection_release(this.connectionHandle)
  }

  private _initRustApi (path?) {
    this.RUST_API = new CXSRuntime(new CXSRuntimeConfig(path))._ffi
  }

  // _clearOnExit creates a callback that will release the Rust Object
  // when the node Connection object is Garbage collected
  private _clearOnExit () {
    const weakRef = weak(this)
    const release = this.RUST_API.cxs_connection_release
    const handle = this.connectionHandle
    weak.addCallback(weakRef, () => {
      release(handle)
    })
  }

  private async _connect (options: IConnectOptions): Promise<number> {
    const phone = options.phone
    const connectionType: string = phone ? 'SMS' : 'QR'
    let connectResult = null
    return await new Promise<number>((resolve, reject) => {
          connectResult = this.RUST_API.cxs_connection_connect(
              0,
              this.connectionHandle,
              JSON.stringify({connection_type: connectionType, phone}),
              ffi.Callback('void', ['uint32', 'uint32'],
                  (command_handle, err) => {
                      resolve(err)
                  }))
          if (connectResult != 0) {
              resolve(connectResult)
          }
        }
    )
  }

  private _sleep = (sleepTime: number): Promise<void> => new Promise((res) => setTimeout(res, sleepTime))

  private _waitFor = async (predicate: () => any, timeout: number, sleepTime: number = 1000) => {
    if (timeout < 0) {
      throw new ConnectionTimeoutError()
    }
    const res = predicate()
    if (!res) {
      await this._sleep(sleepTime)
      return this._waitFor(predicate, timeout - sleepTime)
    }
    return res
  }
}
