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
 * issuerDid: DID associated with the credential def.
 * attributes: key: [value] list of items offered in credential
 */
export interface IIssuerCredentialCreateData {
  sourceId: string,
  credDefId: string,
  attr: {
    [ index: string ]: string
  },
  credentialName: string,
  price: number,
}

export interface IIssuerCredentialVCXAttributes {
  [ index: string ]: [ string ]
}

export interface IIssuerCredentialParams {
  credDefId: string,
  credentialName: string,
  attr: IIssuerCredentialVCXAttributes,
  price: number
}

/**
 * @description Interface that represents the attributes of an Issuer credential object.
 * This interface is expected as the type for deserialize's parameter and serialize's return value
 * @interface
 */
export interface IIssuerCredentialData {
  source_id: string
  handle: number
  schema_seq_no: number
  credential_attributes: string
  credential_name: string
  issuer_did: string
  state: StateType
  msg_uid: string
  cred_def_id: string
  price: number
}

/**
 * @class Class representing an Issuer credential
 */
export class IssuerCredential extends VCXBaseWithState {
  protected _releaseFn = rustAPI().vcx_issuer_credential_release
  protected _updateStFn = rustAPI().vcx_issuer_credential_update_state
  protected _getStFn = rustAPI().vcx_issuer_credential_get_state
  protected _serializeFn = rustAPI().vcx_issuer_credential_serialize
  protected _deserializeFn = rustAPI().vcx_issuer_credential_deserialize
  private _credDefId: string
  private _issuerDID: string
  private _credentialName: string
  private _attr: IIssuerCredentialVCXAttributes
  private _price: number

  constructor (sourceId: string, { credDefId, credentialName, attr, price }: IIssuerCredentialParams) {
    super(sourceId)
    this._credDefId = credDefId
    this._credentialName = credentialName
    this._attr = attr
    this._price = price
  }

  /**
   * @memberof IssuerCredential
   * @description Builds a generic Issuer credential object
   * @static
   * @async
   * @function create
   * @param {IIssuerCredentialCreateData} config
   * @example <caption>Example of ICredentialConfig</caption>
   * { sourceId: "12", credDefId: "credDefId", attr: {key: "value"}, credentialName: "name", price: 0}
   * @returns {Promise<IssuerCredential>} An Issuer credential Object
   */
  static async create ({ attr, sourceId, credDefId,
                         credentialName, price }: IIssuerCredentialCreateData): Promise<IssuerCredential> {
    const attrsVCX: IIssuerCredentialVCXAttributes = Object.keys(attr)
      .reduce((accum, attrKey) => ({ ...accum, [attrKey]: [attr[attrKey]] }), {})
    const credential = new IssuerCredential(sourceId, { credDefId, credentialName, attr: attrsVCX, price })
    const attrsStringified = JSON.stringify(attrsVCX)
    const commandHandle = 0
    const issuerDid = null
    try {
      await credential._create((cb) => rustAPI().vcx_issuer_create_credential(
        commandHandle,
        sourceId,
        credDefId,
        issuerDid,
        attrsStringified,
        credentialName,
        price,
        cb
        )
      )
      return credential
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), 'vcx_issuer_create_credential')
    }
  }

/**
 * @memberof IssuerCredential
 * @description Builds an Issuer credential object with defined attributes.
 * Attributes are provided by a previous call to the serialize function.
 * @static
 * @async
 * @function deserialize
 * @param {IIssuerCredentialData} credentialData - Data from the serialize api. Used to create IssuerCredential Object
 * @returns {Promise<IssuerCredential>} An Issuer credential Object
 */
  static async deserialize (credentialData: IIssuerCredentialData) {
    try {
      const attr = JSON.parse(credentialData.credential_attributes)
      const params: IIssuerCredentialParams = {
        attr,
        credDefId: credentialData.cred_def_id,
        credentialName: credentialData.credential_name,
        price: credentialData.price
      }
      const credential = await super._deserialize<IssuerCredential, IIssuerCredentialParams>(IssuerCredential,
         credentialData,
         params)
      return credential
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), 'vcx_issuer_credential_deserialize')
    }
  }

  /**
   * @memberof IssuerCredential
   * @description Gets the state of the issuer credential object.
   * @async
   * @function getState
   * @returns {Promise<number>}
   */
  async getState (): Promise<number> {
    try {
      return await this._getState()
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), 'vcx_issuer_credential_get_state')
    }
  }

  /**
   * @memberof IssuerCredential
   * @description Communicates with the agent service for polling and setting the state of the issuer credential.
   * @async
   * @function updateState
   * @returns {Promise<void>}
   */
  async updateState (): Promise<void> {
    try {
      await this._updateState()
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), 'vcx_issuer_credential_updateState')
    }
  }

  /**
   * @memberof IssuerCredential
   * @description Serializes an Issuer credential object.
   * Data returned can be used to recreate an Issuer credential object by passing it to the deserialize function.
   * @async
   * @function serialize
   * @returns {Promise<IProofData>} - Jason object with all of the underlying Rust attributes.
   * Same json object structure that is passed to the deserialize function.
   */
  async serialize (): Promise<IIssuerCredentialData> {
    try {
      return JSON.parse(await super._serialize())
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), 'vcx_issuer_credential_serialize')
    }
  }

  /**
   * @memberof IssuerCredential
   * @description Sends a credential Offer to the end user.
   * credential Offer is made up of the data provided in the creation of this object
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
            const rc = rustAPI().vcx_issuer_send_credential_offer(0, this.handle, connection.handle, cb)
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
      throw new VCXInternalError(err, VCXBase.errorMessage(err), 'vcx_issuer_send_credential_offer')
    }
  }

/**
 * @memberof IssuerCredential
 * @description Sends the credential to the end user.
 * credential is made up of the data sent during credential Offer
 * @async
 * @function sendcredential
 * @param {Connection} connection
 * Connection is the object that was created to set up the pairwise relationship.
 * @returns {Promise<void>}
 */
  async sendCredential (connection: Connection): Promise<void> {
    try {
      await createFFICallbackPromise<void>(
        (resolve, reject, cb) => {
          const rc = rustAPI().vcx_issuer_send_credential(0, this.handle, connection.handle, cb)
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
      throw new VCXInternalError(err, VCXBase.errorMessage(err), 'vcx_issuer_send_credential')
    }
  }

  get issuerDid () {
    return this._issuerDID
  }

  get credDefId () {
    return this._credDefId
  }

  get attr () {
    return this._attr
  }
}
