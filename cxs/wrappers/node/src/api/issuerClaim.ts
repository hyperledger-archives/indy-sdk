import { Callback } from 'ffi'

import { CXSInternalError } from '../errors'
import { rustAPI } from '../rustlib'
import { createFFICallbackPromise } from '../utils/ffi-helpers'
import { GCWatcher } from '../utils/memory-management-helpers'
import { StateType } from './common'
import { Connection } from './connection'

export interface IClaimConfig {
  sourceId: string,
  schemaNum: number,
  issuerDid: string,
  attr: string,
  claimName: string,
}
export interface IClaimData {
  source_id: string
  handle: number
  schema_seq_no: number
  claim_attributes: string
  issuer_did: string
  state: StateType
}

export class IssuerClaim extends GCWatcher {
  protected _releaseFn = rustAPI().cxs_connection_release // TODO: Fix me
  protected _handle: string
  private _attr: string
  private _schemaNum: number
  private _sourceId: string
  private _state: number
  private _issuerDID: string
  private _claimName: string

  constructor (sourceId) {
    super()
    this._sourceId = sourceId
    this._handle = null
    this._state = StateType.None
    this._schemaNum = null
    this._attr = null
    this._issuerDID = null
    this._claimName = 'Claim Name Here'
  }

  // SourceId: String for SDK User's reference
  // schemaNumber: number representing the schema sequence number of the claim def
  // issuerDid: String, DID associated with the claim def
  // attributes: String(JSON formatted) representing the attributes of the claim def
  static async create (config: IClaimConfig): Promise<IssuerClaim> {
    const claim = new IssuerClaim(config.sourceId)
    await claim.init(config.sourceId, config.schemaNum, config.issuerDid, config.attr, config.claimName)
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
    const claimHandle = this._handle
    const state = await createFFICallbackPromise<string>(
      (resolve, reject, callback) => {
        const commandHandle = 1
        const rc = rustAPI().cxs_issuer_claim_update_state(commandHandle, claimHandle, callback)
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
    return this._handle
  }

  getSchemaNum () {
    return this._schemaNum
  }

  getAttr () {
    return this._attr
  }

  getState (): StateType {
    return this._state
  }

  async serialize (): Promise<IClaimData> {
    const claimHandle = this._handle
    let rc = null
    try {
      const data = await createFFICallbackPromise<string>(
          (resolve, reject, cb) => {
            rc = rustAPI().cxs_issuer_claim_serialize(0, claimHandle, cb)
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
    const claimHandle = this._handle
    try {
      await createFFICallbackPromise<void>(
          (resolve, reject, cb) => {
            const rc = rustAPI().cxs_issuer_send_claim_offer(0, claimHandle, connection.getHandle(), cb)
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
          const rc = rustAPI().cxs_issuer_send_claim(0, this._handle, connection.getHandle(), cb)
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

  private _setState (state: StateType) {
    this._state = state
  }

  private async init (sourceId: string, schemaNumber: number, issuerDid: string, attr: string, claimName: string):
   Promise<void> {
    this._schemaNum = schemaNumber
    this._attr = attr
    this._sourceId = sourceId
    this._issuerDID = issuerDid
    this._claimName = claimName
    try {
      const data = await createFFICallbackPromise<string>(
          (resolve, reject, cb) => {
            // TODO: check if cxs_issuer_create_claim has a return value
            rustAPI().cxs_issuer_create_claim(0,
               this._sourceId, this._schemaNum, this._issuerDID, this._attr, this._claimName, cb)
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
      this._setHandle(data)
      await this.updateState()
    } catch (err) {
      throw new CXSInternalError(`cxs_issuer_create_claim -> ${err}`)
    }
  }

  private async _initFromClaimData (claimData: IClaimData): Promise<void> {
    try {
      const xclaimHandle = await createFFICallbackPromise<string>(
          (resolve, reject, cb) => {
            rustAPI().cxs_issuer_claim_deserialize(0, JSON.stringify(claimData), cb)
          },
          (resolve, reject) => Callback('void', ['uint32', 'uint32', 'uint32'], (commandHandle, err, claimHandle) => {
            if (err) {
              reject(err)
              return
            }
            resolve(claimHandle)
          })
      )
      this._setHandle(xclaimHandle)
      await this.updateState()
    } catch (err) {
      throw new CXSInternalError(`cxs_issuer_claim_deserialize -> ${err}`)
    }
  }
}
