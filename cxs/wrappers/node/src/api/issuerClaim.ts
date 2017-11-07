import { format } from 'path'
import { Callback, ForeignFunction } from 'ffi'
import { weak } from 'weak'
import { CXSRuntime, CXSRuntimeConfig } from '../index'
import { IClaimData, StateType } from './api'
import { CXSInternalError } from './errors'

export class IssuerClaim {
  private _attr: string
  private _schemaNum: number
  private _sourceId: string
  private _claimHandle: number
  private _state: number
  private _RUST_API: { [ index: string ]: ForeignFunction }
  constructor (sourceId) {
    this._sourceId = sourceId
    this._initRustApi(null)
    this._claimHandle = null
    this._state = StateType.None
    this._schemaNum = null
    this._attr = null
  }
  static async create (sourceId: string, schemaNumber: number, attributes: string): Promise<IssuerClaim> {
    const claim = new IssuerClaim(sourceId)
    await claim.init(sourceId, schemaNumber, attributes)
    return claim
  }

  static async deserialize (claimData: IClaimData): Promise<IssuerClaim> {
    const sourceId = claimData.source_id
    const attr = claimData.claim_attributes
    const schemaNumber = claimData.claim_def
    const claim = await IssuerClaim.create(sourceId, schemaNumber , attr)
    await claim._initFromClaimData(claimData)
    return claim
  }

  async _callCxsAndGetCurrentState () {
    const buff = await this.serialize()
    const json = buff
    const state = json ? json.state : null
    return state
  }

  getSourceId () {
    return this._sourceId
  }

  getClaimHandle () {
    return this._claimHandle
  }

  getSchemaNum () {
    return this._schemaNum
  }

  getAttr () {
    return this._attr
  }

  setClaimHandle (handle) {
    this._claimHandle = handle
  }

  getState () {
    return this._state
  }

  async serialize (): Promise<IClaimData> {
    let callback = null
    const claimHandle = this._claimHandle
    try {
      const ptr = await new Promise<string> ((resolve, reject) => {
        callback = Callback('void', ['uint32', 'uint32', 'string'], (xclaimHandle, err, serializedClaim) => {
          if (err > 0 ) {
            reject(err)
            return
          }
          resolve(serializedClaim)
        })
        const rc = this._RUST_API.cxs_issuer_claim_serialize(0, claimHandle, callback)
        if (rc > 0) {
          // TODO: handle correct exception
          resolve(null)
        }
      })
      const data: IClaimData = JSON.parse(ptr)
      return data
    } catch (error) {
      throw new CXSInternalError(`cxs_issuer_send_claim_offer -> ${error}`)
    }
  }

  async send (connectionHandle): Promise<void> {
    let callback = null
    const claimHandle = this._claimHandle
    try {
      await new Promise<void> ((resolve, reject) => {
        callback = Callback('void', ['uint32', 'uint32'], (xcommandHandle, err) => {
          if (err > 0 ) {
            reject(err)
            return
          }
          resolve(xcommandHandle)

        })
        this._RUST_API.cxs_issuer_send_claim_offer(0, claimHandle, connectionHandle, callback)
        this._setState(StateType.OfferSent)
      })
    } catch (err) {
      // TODO handle error
      throw new CXSInternalError(`cxs_issuer_send_claim_offer -> ${err}`)
    }
  }

  private _setState (state) {
    this._state = state
  }
  private async init (sourceId: string, schemaNumber: number, attr: string): Promise<void> {
    let callback = null
    this._schemaNum = schemaNumber
    this._attr = attr
    this._sourceId = sourceId
    const data = await new Promise<number>((resolve,reject) => {
      callback = Callback('void', ['uint32', 'uint32', 'uint32'], (commandHandle, err, claimHandle) => {
        if (err > 0) {
          reject (err)
          return
        }
        const value = JSON.stringify(claimHandle)
        resolve(Number(value))
      })
      this._RUST_API.cxs_issuer_create_claim(0, this._sourceId,
        this._schemaNum, '8XFh8yBzrpJQmNyZzgoTqB',this._attr, callback)
    })
    this.setClaimHandle(data)
    this._setState(await this._callCxsAndGetCurrentState())
  }

  private async _initFromClaimData (claimData: IClaimData): Promise<void> {
    let callback = null
    try {
      const xclaimHandle = await new Promise<number> ((resolve, reject) => {
        callback = Callback('void', ['uint32', 'uint32', 'uint32'],
        (xcommandHandle, err, claimHandle) => {
          if (err > 0 ) {
            reject(err)
            return
          }
          resolve(claimHandle)
        })
        this._RUST_API.cxs_issuer_claim_deserialize(0, JSON.stringify(claimData), callback)
      })
      this.setClaimHandle(xclaimHandle)
      this._setState(await this._callCxsAndGetCurrentState())
    } catch (error) {
      throw new CXSInternalError(`cxs_issuer_claim_deserialize -> ${error}`)
    }
  }

  private _clearOnExit () {
    const weakRef = weak(this)
    const release = this._RUST_API.cxs_connection_release
    const handle = this._claimHandle
    weak.addCallback(weakRef, () => {
      release(handle)
    })
  }
  private _initRustApi (path?) {
    this._RUST_API = new CXSRuntime(new CXSRuntimeConfig(path))._ffi
  }
}
