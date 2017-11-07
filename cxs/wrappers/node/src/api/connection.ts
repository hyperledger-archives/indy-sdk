import * as ffi from 'ffi'
import * as weak from 'weak'
import { CXSRuntime } from '../index'
import { CXSRuntimeConfig } from '../rustlib'
import {
    createFFICallbackPromise,
    IConnectionData,
    IConnections,
    IConnectOptions,
    IRecipientInfo,
    StateType
} from './api'
import { ConnectionTimeoutError, CXSInternalError } from './errors'

export class Connection implements IConnections {
  public connectionHandle: string
  private _state: StateType
  private RUST_API: { [ index: string ]: ffi.ForeignFunction }

  constructor ( path?: string ) {
    this._initRustApi(path)
    this._state = StateType.None
  }

  static async create ( recipientInfo: IRecipientInfo, path?: string ): Promise<Connection> {
    const connection = new Connection(path)
    await connection.init(recipientInfo)
    return connection
  }

  static async deserialize (connectionData: IConnectionData, path?: string): Promise<Connection> {
    const sourceId = connectionData.source_id
    const didSelf = connectionData.pw_did ? connectionData.pw_did : ''
    const didRemote = connectionData.did_endpoint ? connectionData.did_endpoint : ''
    const connection = await Connection.create({id: sourceId, DIDself: didSelf, DIDremote: didRemote }, path)
    await connection._initFromConnectionData(connectionData)
    return connection
  }

  async connect ( options: IConnectOptions = {} ): Promise<void> {
    const timeout = options.timeout || 10000
    await this._waitFor(async () => await this._connect(options) === 0, timeout)
  }

  async serialize (): Promise<IConnectionData> {
    let rc = null
    try {
      const data = await createFFICallbackPromise<string>(
            (resolve, reject, cb) => {
              rc = this.RUST_API.cxs_connection_serialize(0, this.connectionHandle, cb)
              if (rc) {
                reject(rc)
              }
            },
            (resolve, reject) => ffi.Callback('void', ['uint32', 'uint32', 'string'], (handle, err, _data) => {
              if (err) {
                reject(err)
                return
              } else if (_data == null) {
                reject('no connection to serialize')
              }
              resolve(_data)
            })
        )
      return JSON.parse(data)
    } catch (err) {
      throw new CXSInternalError(`cxs_connection_serialize -> ${rc}`)
    }
  }

  async updateState (): Promise<void> {
    try {
      const state = await createFFICallbackPromise<number>(
          (resolve, reject, cb) => {
            const rc = this.RUST_API.cxs_connection_update_state(0, this.connectionHandle, cb)
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
      this._setState(state)
    } catch (error) {
      throw new CXSInternalError(`cxs_connection_updateState -> ${error}`)
    }
  }

  getState (): number {
    return this._state
  }

  async release (): Promise<number> {
    return this.RUST_API.cxs_connection_release(this.connectionHandle)
  }

  getHandle () {
    return this.connectionHandle
  }

  _setConnectionHandle (handle) {
    this.connectionHandle = handle
  }

  private _initRustApi (path?) {
    this.RUST_API = new CXSRuntime(new CXSRuntimeConfig(path))._ffi
  }

  private _setState (state) {
    this._state = state
  }

  private async init ( recipientInfo: IRecipientInfo ): Promise<void> {
    const id = recipientInfo.id // TODO verifiy that id is a string
    try {
      this.connectionHandle = await createFFICallbackPromise<string>(
          (resolve, reject, cb) => {
            const rc = this.RUST_API.cxs_connection_create(0, id, cb)
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
    } catch (error) {
      throw new CXSInternalError(`cxs_connection_create -> ${error}`)
    }
    this._clearOnExit()
  }

  private async _initFromConnectionData (connectionData: IConnectionData): Promise<void> {
    const commandHandle = 0
    try {
      const connectionHandle = await createFFICallbackPromise<string>(
          (resolve, reject, cb) => {
            const rc = this.RUST_API.cxs_connection_deserialize(commandHandle, JSON.stringify(connectionData), cb)
            if (rc) {
              reject(rc)
            }
          },
          (resolve, reject) => ffi.Callback('void', ['uint32', 'uint32', 'uint32'], (xHandle, _rc, handle) => {
            if (_rc) {
              reject(_rc)
            }
            resolve(JSON.stringify(handle))
          })
      )
      this._setConnectionHandle(connectionHandle)
      await this.updateState()
    } catch (error) {
      throw new CXSInternalError(`cxs_connection_deserialize -> ${error}`)
    }
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
    const connectionData: string = JSON.stringify({connection_type: connectionType, phone})
    try {
      return await createFFICallbackPromise<number>(
          (resolve, reject, cb) => {
            const rc = this.RUST_API.cxs_connection_connect(0, this.connectionHandle, connectionData, cb)
            if (rc) {
              resolve(rc)
            }
          },
          (resolve, reject) => ffi.Callback('void', ['uint32', 'uint32'], (xHandle, err) => {
            resolve(err)
          })
        )
    } catch (error) {
      throw new CXSInternalError(`cxs_connection_connect -> ${error}`)
    }
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
