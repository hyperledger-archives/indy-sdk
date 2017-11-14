import { Callback, ForeignFunction } from 'ffi'
import { weak } from 'weak'
import { CXSRuntime, CXSRuntimeConfig } from '../index'
import { createFFICallbackPromise, IClaimData, StateType } from './api'
import { Connection } from './connection'
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
  // SourceId: String for SDK User's reference
  // schemaNumber: number representing the schema sequence number of the claim def
  // issuerDid: String, DID associated with the claim def
  // attributes: String(JSON formatted) representing the attributes of the claim def
  static async create (sourceId: string, schemaNumber: number,
                       issuerDid: string, attributes: string): Promise<IssuerClaim> {
    const claim = new IssuerClaim(sourceId)
    await claim.init(sourceId, schemaNumber, issuerDid, attributes)
    return claim
  }

  // Deserializes a JSON representing a issuer claim object
  static async deserialize (claimData: IClaimData): Promise<IssuerClaim> {
    const claim = new IssuerClaim(claimData.source_id)
    await claim._initFromClaimData(claimData)
    return claim
  }

  // Calls the cxs update state.  Used for polling the state of the issuer claim.
  // For example, when waiting for a request to send a claim offer.
  async updateState (): Promise<void> {
    const claimHandle = this._claimHandle
    const state = await createFFICallbackPromise<string>(
      (resolve, reject, callback) => {
        const commandHandle = 1
        const rc = this._RUST_API.cxs_issuer_claim_update_state(commandHandle, claimHandle, callback)
        if (rc) {
          reject(rc)
        }
      },
      (resolve, reject) => Callback('void', ['uint32', 'uint32', 'uint32', 'uint32'],
        (xcommandHandle, err, xstate) => {
          if (err > 0) {
            reject(err)
            return
          }
          resolve(JSON.stringify(xstate))
        })
      )
    this._setState(Number(state))
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
              reject(rc)
            }
          },
          (resolve, reject) => Callback('void', ['uint32', 'uint32', 'string'], (handle, err, serializedClaim) => {
            if (err) {
              reject(err)
              return
            } else if (serializedClaim == null) {
              reject('no claim to serialize')
            }
            resolve(serializedClaim)
          })
      )
      return JSON.parse(data)
    } catch (err) {
      throw new CXSInternalError(`cxs_issuer_claim_serialize -> ${rc}`)
    }
  }
  // send a claim offer to the connection
  async sendOffer (connection: Connection): Promise<void> {
    const claimHandle = this._claimHandle
    try {
      await createFFICallbackPromise<void>(
          (resolve, reject, cb) => {
            const rc = this._RUST_API.cxs_issuer_send_claim_offer(0, claimHandle, connection.getHandle(), cb)
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

  // Send a claim to the connection.
  async sendClaim (connection: Connection): Promise<void> {
    try {
      await createFFICallbackPromise<void>(
        (resolve, reject, cb) => {
          const rc = this._RUST_API.cxs_issuer_send_claim(0, this._claimHandle, connection.getHandle(), cb)
          if (rc) {
            reject(rc)
          }
        },
        (resolve, reject) => Callback('void', ['uint32', 'uint32'], (xcommandHandle, err) => {
          if (err) {
            reject(err)
            return
          }
          resolve(xcommandHandle)
        })
      )
      await this.updateState()
    } catch (err) {
      throw new CXSInternalError(`cxs_issuer_send_claim -> ${err}`)
    }
  }

  private _setState (state) {
    this._state = state
  }

  private async init (sourceId: string, schemaNumber: number, issuerDid: string, attr: string): Promise<void> {
    this._schemaNum = schemaNumber
    this._attr = attr
    this._sourceId = sourceId
    this._issuerDID = issuerDid
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
      await this.updateState()
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
      await this.updateState()
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
