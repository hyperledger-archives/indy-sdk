import { Callback } from 'ffi'

import { CXSInternalError } from '../errors'
import { rustAPI } from '../rustlib'
import { createFFICallbackPromise } from '../utils/ffi-helpers'
import { StateType } from './common'
import { Connection } from './connection'
import { CXSBase } from './CXSBase'

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

export class IssuerClaim extends CXSBase {
  protected _releaseFn = rustAPI().cxs_connection_release // TODO: Fix me
  protected _updateStFn = rustAPI().cxs_issuer_claim_update_state
  protected _serializeFn = rustAPI().cxs_issuer_claim_serialize
  protected _deserializeFn = rustAPI().cxs_issuer_claim_deserialize
  private _schemaNum: number
  private _issuerDID: string
  private _claimName: string

  constructor (sourceId) {
    super(sourceId)
    this._schemaNum = null
    this._issuerDID = null
    this._claimName = 'Claim Name Here'
  }

  // SourceId: String for SDK User's reference
  // schemaNumber: number representing the schema sequence number of the claim def
  // issuerDid: String, DID associated with the claim def
  // attributes: String(JSON formatted) representing the attributes of the claim def
  static async create (config: IClaimConfig): Promise<IssuerClaim> {
    const claim = new IssuerClaim(config.sourceId)
    claim._schemaNum = config.schemaNum
    claim._issuerDID = config.issuerDid
    claim._attr = config.attr
    claim._claimName = config.claimName
    await claim._create((cb) => rustAPI().cxs_issuer_create_claim(
        0,
        config.sourceId,
        config.schemaNum,
        config.issuerDid,
        config.attr,
        config.claimName,
        cb
      )
    )
    return claim
  }

  // Deserializes a JSON representing a issuer claim object
  static async deserialize (claimData: IClaimData) {
    try {
      return await super._deserialize(IssuerClaim, claimData)
    } catch (err) {
      throw new CXSInternalError(`cxs_issuer_claim_deserialize -> ${err}`)
    }
  }

  // Calls the cxs update state.  Used for polling the state of the issuer claim.
  // For example, when waiting for a request to send a claim offer.
  async updateState (): Promise<void> {
    try {
      await this._updateState()
    } catch (error) {
      throw new CXSInternalError(`cxs_issuer_claim_updateState -> ${error}`)
    }
  }

  async serialize (): Promise<IClaimData> {
    try {
      return JSON.parse(await super._serialize())
    } catch (err) {
      throw new CXSInternalError(`cxs_issuer_claim_serialize -> ${err}`)
    }
  }

  // send a claim offer to the connection
  async sendOffer (connection: Connection): Promise<void> {
    const claimHandle = this.handle
    try {
      await createFFICallbackPromise<void>(
          (resolve, reject, cb) => {
            const rc = rustAPI().cxs_issuer_send_claim_offer(0, claimHandle, connection.handle, cb)
            if (rc) {
              reject(rc)
            }
            this._state = StateType.OfferSent
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
          const rc = rustAPI().cxs_issuer_send_claim(0, this.handle, connection.handle, cb)
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
      await this._updateState()
    } catch (err) {
      throw new CXSInternalError(`cxs_issuer_send_claim -> ${err}`)
    }
  }

  get issuerDid () {
    return this._issuerDID
  }

  get schemaNum () {
    return this._schemaNum
  }
}
