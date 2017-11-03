import * as ffi from 'ffi'
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

  async create ( recipientInfo: IRecipientInfo ): Promise<void> {
    const id = recipientInfo.id // TODO verifiy that id is a string
    let result = null
    try {
      this.connectionHandle = await new Promise<string>((resolve, reject) => {
        result = this.RUST_API.cxs_connection_create(
              0,
              id,
            ffi.Callback('void', ['uint32', 'uint32', 'uint32'],
                  (xHandle, err, _connectionHandle) => {
                    if (err) {
                      reject(err)
                      return
                    }
                    resolve(_connectionHandle)
                  }))
        if (result) {
          reject(result)
        }
      })

    } catch (error) {
      throw new CXSInternalError(`cxs_connection_connect -> ${error}`)
    }
    this._clearOnExit()

    return result
  }

  async connect ( options: IConnectOptions = {} ): Promise<void> {
    const timeout = options.timeout || 10000
    await this._waitFor(async () => await this._connect(options) === 0, timeout)
  }

  async serialize (): Promise<IConnectionData> {
    const commandHandle = 0
    try {
      const data = await new Promise<string>((resolve, reject) => {
        const rc = this.RUST_API.cxs_connection_serialize(
              commandHandle,
              this.connectionHandle,
              ffi.Callback('void', ['uint32', 'uint32', 'string'], (handle, err, _data) => {
                if (err) {
                  reject(err)
                  return
                } else if (_data === '') {
                  _data = null
                }
                resolve(_data)
              }))

        if (rc) {
          resolve(null)
        }
      })
      return JSON.parse(data)
    } catch (error) {
      throw new CXSInternalError(`cxs_connection_serialize -> ${error}`)
    }
  }

  async deserialize (connectionData): Promise<void> {
    const commandHandle = 0
    let result = 0
    try {
      this.connectionHandle = await new Promise<string>((resolve, reject) => {
        result = this.RUST_API.cxs_connection_deserialize(
                commandHandle,
                connectionData,
                ffi.Callback('void', ['uint32', 'uint32', 'uint32'], (xHandle, _rc, handle) => {
                  if (_rc) {
                    reject(_rc)
                  }
                  resolve(JSON.stringify(handle))
                }))
        if (result) {
          reject(result)
        }
      })
    } catch (error) {
      throw new CXSInternalError(`cxs_connection_deserialize -> ${error}`)
    }
  }

  async updateState (): Promise<void> {
    try {
      this.state = await new Promise<number>((resolve, reject) => {
        const rc = this.RUST_API.cxs_connection_update_state(
              0,
              this.connectionHandle,
              ffi.Callback('void', ['uint32', 'uint32', 'uint32'], (handle, err, state) => {
                if (err) {
                  reject(err)
                  return
                }
                resolve(state)
              }))
        if (rc) {
          resolve(StateType.None)
        }
      })

    } catch (error) {
      throw new CXSInternalError(`cxs_connection_get_state -> ${error}`)
    }
  }

  async release (): Promise<number> {
    return this.RUST_API.cxs_connection_release(this.connectionHandle)
  }
  async getHandle () {
    return this.connectionHandle
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
              ffi.Callback('void', ['uint32', 'uint32'], (xhandle, err) => {
                resolve(err)
              }))
      if (connectResult) {
        resolve(connectResult)
      }
    })
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
