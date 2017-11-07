import { Callback, ForeignFunction } from 'ffi'
import { weak } from 'weak'
import { CXSRuntime, CXSRuntimeConfig } from '../index'
import { createFFICallbackPromise, IClaimData, StateType } from './api'
import { CXSInternalError } from './errors'

export class IssuerClaim {
  private _attr: string
  private _schemaNum: number
  private _sourceId: string
  private _claimHandle: number
  private _state: number
  private _RUST_API: { [ index: string ]: ForeignFunction }
  private _issuerDID: string
  constructor (sourceId) {
    this._sourceId = sourceId
    this._initRustApi(null)
    this._claimHandle = null
    this._state = StateType.None
    this._schemaNum = null
    this._attr = null
    this._issuerDID = null
  }
  static async create (sourceId: string, schemaNumber: number, did: string, attributes: string): Promise<IssuerClaim> {
    const claim = new IssuerClaim(sourceId)
    await claim.init(sourceId, schemaNumber, did, attributes)
    return claim
  }

  static async deserialize (claimData: IClaimData): Promise<IssuerClaim> {
    const sourceId = claimData.source_id
    const attr = claimData.claim_attributes
    const schemaNumber = claimData.schema_seq_no
    const did = claimData.issuer_did
    const claim = await IssuerClaim.create(sourceId, schemaNumber, did, attr)
    await claim._initFromClaimData(claimData)
    return claim
  }

  async _callCxsAndGetCurrentState () {
    const buff = await this.serialize()
    const json = buff
    const state = json ? json.state : null
    return state
  }

  getIssuedDid () {
    return this._issuerDID
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
    const claimHandle = this._claimHandle
    let rc = null
    try {
      const data = await createFFICallbackPromise<string>(
          (resolve, reject, cb) => {
            rc = this._RUST_API.cxs_issuer_claim_serialize(0, claimHandle, cb)
            if (rc) {
              // TODO: handle correct exception
              resolve(null)
            }
          },
          (resolve, reject) => Callback('void', ['uint32', 'uint32', 'string'], (handle, err, serializedClaim) => {
            if (err) {
              reject(err)
              return
            }
            resolve(serializedClaim)
          })
      )
      return JSON.parse(data)
    } catch (err) {
      throw new CXSInternalError(`cxs_issuer_claim_serialize -> ${rc}`)
    }
  }

  async send (connectionHandle): Promise<void> {
    const claimHandle = this._claimHandle
    try {
      await createFFICallbackPromise<void>(
          (resolve, reject, cb) => {
            const rc = this._RUST_API.cxs_issuer_send_claim_offer(0, claimHandle, connectionHandle, cb)
            if (rc) {
              reject(rc)
            }
            this._setState(StateType.OfferSent)

          },
          (resolve, reject) => Callback('void', ['uint32', 'uint32'], (xcommandHandle, err) => {
            if (err) {
              reject(err)
              return
            }
            resolve(xcommandHandle)
          })
        )
    } catch (err) {
      // TODO handle error
      throw new CXSInternalError(`cxs_issuer_send_claim_offer -> ${err}`)
    }
  }

  private _setState (state) {
    this._state = state
  }

  private async init (sourceId: string, schemaNumber: number, did: string, attr: string): Promise<void> {
    this._schemaNum = schemaNumber
    this._attr = attr
    this._sourceId = sourceId
    this._issuerDID = did
    try {
      const data = await createFFICallbackPromise<number>(
          (resolve, reject, cb) => {
            // TODO: check if cxs_issuer_create_claim has a return value
            this._RUST_API.cxs_issuer_create_claim(0, this._sourceId, this._schemaNum, this._issuerDID, this._attr, cb)
          },
          (resolve, reject) => Callback('void', ['uint32', 'uint32', 'uint32'], (commandHandle, err, claimHandle) => {
            if (err) {
              reject(err)
              return
            }
            const value = JSON.stringify(claimHandle)
            resolve(Number(value))
          })
        )
      this.setClaimHandle(data)
      this._setState(await this._callCxsAndGetCurrentState())
    } catch (err) {
      throw new CXSInternalError(`cxs_issuer_create_claim -> ${err}`)
    }
  }

  private async _initFromClaimData (claimData: IClaimData): Promise<void> {
    try {
      const xclaimHandle = await createFFICallbackPromise<number>(
          (resolve, reject, cb) => {
            this._RUST_API.cxs_issuer_claim_deserialize(0, JSON.stringify(claimData), cb)
          },
          (resolve, reject) => Callback('void', ['uint32', 'uint32', 'uint32'], (commandHandle, err, claimHandle) => {
            if (err) {
              reject(err)
              return
            }
            resolve(claimHandle)
          })
      )
      this.setClaimHandle(xclaimHandle)
      this._setState(await this._callCxsAndGetCurrentState())
    } catch (err) {
      throw new CXSInternalError(`cxs_issuer_claim_deserialize -> ${err}`)
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
