import { Callback } from 'ffi'

import { CXSInternalError } from '../errors'
import { rustAPI } from '../rustlib'
import { createFFICallbackPromise } from '../utils/ffi-helpers'
import { StateType } from './common'
import { Connection } from './connection'
import { CXSBaseWithState } from './CXSBaseWithState'

/**
 * @interface
 * @description
 * SourceId: String for SDK User's reference.
 * issuerDid: DID associated with the claim def.
 * attributes: String(JSON formatted) representing the attributes of the claim def.
 */
export interface IClaimConfig {
  sourceId: string,
  schemaNum: number,
  issuerDid: string,
  attr: {
    [ index: string ]: string
  },
  claimName: string,
}

export interface IClaimCXSAttributes {
  [ index: string ]: [ string ]
}

export interface IClaimParams {
  schemaNum: number,
  issuerDid: string,
  claimName: string,
  attr: IClaimCXSAttributes
}

/**
 * @description Interface that represents the attributes of an Issuer Claim object.
 * This interface is expected as the type for deserialize's parameter and serialize's return value
 * @interface
 */
export interface IClaimData {
  source_id: string
  handle: number
  schema_seq_no: number
  claim_attributes: string
  claim_name: string,
  issuer_did: string
  state: StateType
}

/**
 * @class Class representing an Issuer Claim
 */
export class IssuerClaim extends CXSBaseWithState {
  protected _releaseFn = rustAPI().cxs_connection_release // TODO: Fix me
  protected _updateStFn = rustAPI().cxs_issuer_claim_update_state
  protected _serializeFn = rustAPI().cxs_issuer_claim_serialize
  protected _deserializeFn = rustAPI().cxs_issuer_claim_deserialize
  private _schemaNum: number
  private _issuerDID: string
  private _claimName: string
  private _attr: IClaimCXSAttributes

  constructor (sourceId, { schemaNum, issuerDid, claimName, attr }: IClaimParams) {
    super(sourceId)
    this._schemaNum = schemaNum
    this._issuerDID = issuerDid
    this._claimName = claimName
    this._attr = attr
  }

  /**
   * @memberof IssuerClaim
   * @description Builds a generic Issuer Claim object
   * @static
   * @async
   * @function create
   * @param {IClaimConfig} config
   * @example <caption>Example of IClaimConfig</caption>
   * { sourceId: "12", schemaNum: 1, issuerDid: "did", attr: {key: "value"}, claimName: "name of claim"}
   * @returns {Promise<IssuerClaim>} An Issuer Claim Object
   */
  static async create ({ attr, sourceId, schemaNum, issuerDid, claimName }: IClaimConfig): Promise<IssuerClaim> {
    const attrsCXS: IClaimCXSAttributes = Object.keys(attr)
      .reduce((accum, attrKey) => ({ ...accum, [attrKey]: [attr[attrKey]] }), {})
    const claim = new IssuerClaim(sourceId, { schemaNum, issuerDid, claimName, attr: attrsCXS })
    const attrsStringified = JSON.stringify(attrsCXS)
    try {
      await claim._create((cb) => rustAPI().cxs_issuer_create_claim(
        0,
        sourceId,
        schemaNum,
        issuerDid,
        attrsStringified,
        claimName,
        cb
        )
      )
      await claim._updateState()
      return claim
    } catch (err) {
      throw new CXSInternalError(`cxs_issuer_create_claim -> ${err}`)
    }
  }

/**
 * @memberof IssuerClaim
 * @description Builds an Issuer Claim object with defined attributes.
 * Attributes are often provided by a previous call to the serialize function.
 * @static
 * @async
 * @function deserialize
 * @param {IClaimData} claimData - contains the information that will be used to build a proof object
 * @example <caption>Example of claimData.</caption>
 * { source_id: "12", handle: 22, schema_seq_no: 1, claim_attributes: "{key: [\"value\"]}",
 * issuer_did: "did", state: 1 }
 * @returns {Promise<Connection>} An Issuer Claim Object
 */
  static async deserialize (claimData: IClaimData) {
    try {
      const attr = JSON.parse(claimData.claim_attributes)
      const params: IClaimParams = {
        attr,
        claimName: claimData.claim_name,
        issuerDid: claimData.issuer_did,
        schemaNum: claimData.schema_seq_no
      }
      const claim = await super._deserialize<IssuerClaim, IClaimParams>(IssuerClaim, claimData, params)
      await claim._updateState()
      return claim
    } catch (err) {
      throw new CXSInternalError(`cxs_issuer_claim_deserialize -> ${err}`)
    }
  }

  /**
   * @memberof IssuerClaim
   * @description Communicates with the agent service for polling and setting the state of the issuer claim.
   * @async
   * @function updateState
   * @returns {Promise<void>}
   */
  async updateState (): Promise<void> {
    try {
      await this._updateState()
    } catch (error) {
      throw new CXSInternalError(`cxs_issuer_claim_updateState -> ${error}`)
    }
  }

  /**
   * @memberof IssuerClaim
   * @description Serializes an Issuer Claim object.
   * Data returned can be used to recreate an Issuer Claim object by passing it to the deserialize function.
   * @async
   * @function serialize
   * @returns {Promise<IProofData>} - Jason object with all of the underlying Rust attributes.
   * Same json object structure that is passed to the deserialize function.
   */
  async serialize (): Promise<IClaimData> {
    try {
      return JSON.parse(await super._serialize())
    } catch (err) {
      throw new CXSInternalError(`cxs_issuer_claim_serialize -> ${err}`)
    }
  }

  /**
   * @memberof IssuerClaim
   * @description Sends a Claim Offer to the end user.
   * Claim Offer is made up of the data provided in the creation of this object
   * @async
   * @function sendOffer
   * @param {Connection} connection
   * Connection is the object that was created to set up the pairwise relationship.
   * @returns {Promise<void>}
   */
  async sendOffer (connection: Connection): Promise<void> {
    try {
      await createFFICallbackPromise<void>(
          (resolve, reject, cb) => {
            const rc = rustAPI().cxs_issuer_send_claim_offer(0, this.handle, connection.handle, cb)
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

/**
 * @memberof IssuerClaim
 * @description Sends the Claim to the end user.
 * Claim is made up of the data sent during Claim Offer
 * @async
 * @function sendClaim
 * @param {Connection} connection
 * Connection is the object that was created to set up the pairwise relationship.
 * @returns {Promise<void>}
 */
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

  get attr () {
    return this._attr
  }
}
