import { Callback } from 'ffi'

import { VCXInternalError } from '../errors'
import { rustAPI } from '../rustlib'
import { createFFICallbackPromise } from '../utils/ffi-helpers'
import { StateType } from './common'
import { Connection } from './connection'
import { VCXBase } from './VCXBase'
import { VCXBaseWithState } from './VCXBaseWithState'

/**
 * @interface
 * @description
 * SourceId: String for SDK User's reference.
 * issuerDid: DID associated with the claim def.
 * attributes: key: [value] list of items offered in claim
 */
export interface IClaimConfig {
  sourceId: string,
  schemaNum: number,
  attr: {
    [ index: string ]: string
  },
  claimName: string,
}

export interface IClaimVCXAttributes {
  [ index: string ]: [ string ]
}

export interface IClaimParams {
  schemaNum: number,
  claimName: string,
  attr: IClaimVCXAttributes
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
  claim_name: string
  issuer_did: string
  state: StateType
  msg_uid: string
}

/**
 * @class Class representing an Issuer Claim
 */
export class IssuerClaim extends VCXBaseWithState {
  protected _releaseFn = rustAPI().vcx_issuer_claim_release
  protected _updateStFn = rustAPI().vcx_issuer_claim_update_state
  protected _getStFn = rustAPI().vcx_issuer_claim_get_state
  protected _serializeFn = rustAPI().vcx_issuer_claim_serialize
  protected _deserializeFn = rustAPI().vcx_issuer_claim_deserialize
  private _schemaNum: number
  private _issuerDID: string
  private _claimName: string
  private _attr: IClaimVCXAttributes

  constructor (sourceId, { schemaNum, claimName, attr }: IClaimParams) {
    super(sourceId)
    this._schemaNum = schemaNum
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
  static async create ({ attr, sourceId, schemaNum, claimName }: IClaimConfig): Promise<IssuerClaim> {
    const attrsVCX: IClaimVCXAttributes = Object.keys(attr)
      .reduce((accum, attrKey) => ({ ...accum, [attrKey]: [attr[attrKey]] }), {})
    const claim = new IssuerClaim(sourceId, { schemaNum, claimName, attr: attrsVCX })
    const attrsStringified = JSON.stringify(attrsVCX)
    const commandHandle = 0
    const issuerDid = null
    try {
      await claim._create((cb) => rustAPI().vcx_issuer_create_claim(
        commandHandle,
        sourceId,
        schemaNum,
        issuerDid,
        attrsStringified,
        claimName,
        cb
        )
      )
      return claim
    } catch (err) {
      throw new VCXInternalError(err, await VCXBase.errorMessage(err), 'vcx_issuer_create_claim')
    }
  }

/**
 * @memberof IssuerClaim
 * @description Builds an Issuer Claim object with defined attributes.
 * Attributes are often provided by a previous call to the serialize function.
 * @static
 * @async
 * @function deserialize
 * @param {IClaimData} claimData - contains the information that will be used to build an issuerClaim object
 * @example <caption>Example of claimData.</caption>
 * { source_id: "12", handle: 22, schema_seq_no: 1, claim_attributes: "{key: [\"value\"]}",
 * issuer_did: "did", state: 1 }
 * @returns {Promise<IssuerClaim>} An Issuer Claim Object
 */
  static async deserialize (claimData: IClaimData) {
    try {
      const attr = JSON.parse(claimData.claim_attributes)
      const params: IClaimParams = {
        attr,
        claimName: claimData.claim_name,
        schemaNum: claimData.schema_seq_no
      }
      const claim = await super._deserialize<IssuerClaim, IClaimParams>(IssuerClaim, claimData, params)
      return claim
    } catch (err) {
      throw new VCXInternalError(err, await VCXBase.errorMessage(err), 'vcx_issuer_claim_deserialize')
    }
  }

  /**
   * @memberof IssuerClaim
   * @description Gets the state of the issuer claim object.
   * @async
   * @function getState
   * @returns {Promise<number>}
   */
  async getState (): Promise<number> {
    try {
      return await this._getState()
    } catch (err) {
      throw new VCXInternalError(err, await VCXBase.errorMessage(err), 'vcx_issuer_claim_get_state')
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
    } catch (err) {
      throw new VCXInternalError(err, await VCXBase.errorMessage(err), 'vcx_issuer_claim_updateState')
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
      throw new VCXInternalError(err, await VCXBase.errorMessage(err), 'vcx_issuer_claim_serialize')
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
            const rc = rustAPI().vcx_issuer_send_claim_offer(0, this.handle, connection.handle, cb)
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
    } catch (err) {
      // TODO handle error
      throw new VCXInternalError(err, await VCXBase.errorMessage(err), 'vcx_issuer_send_claim_offer')
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
          const rc = rustAPI().vcx_issuer_send_claim(0, this.handle, connection.handle, cb)
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
    } catch (err) {
      throw new VCXInternalError(err, await VCXBase.errorMessage(err), 'vcx_issuer_send_claim')
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
