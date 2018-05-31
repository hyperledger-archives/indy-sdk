import * as ffi from 'ffi'
import { VCXInternalError } from '../errors'
import { rustAPI } from '../rustlib'
import { createFFICallbackPromise } from '../utils/ffi-helpers'
import { VCXBase } from './VCXBase'

/**
 * @interface
 * @description
 * SourceId: String for SDK User's reference.
 * name: name of credentialdef.
 * schemaNo: Schema Number wanted to create credentialdef off of
 * revocation:
 */
export interface ICredentialDefinition {
  sourceId: string,
  name: string,
  schemaId: string,
  revocation: boolean,
  paymentHandle: number
}

export interface ICredentialDefObj {
  source_id: string,
  handle: number
  name: string
  credential_def: ICredentialDefData
}

export interface ICredentialDefData {
  ref: number,
  origin: string,
  signature_type: string,
  data: any,
}

export interface ICredentialDefParams {
  schemaId: string,
  name: string,
}

/**
 * @class Class representing a credential Definition
 */
export class CredentialDef extends VCXBase {
  protected _releaseFn = rustAPI().vcx_credentialdef_release
  protected _serializeFn = rustAPI().vcx_credentialdef_serialize
  protected _deserializeFn = rustAPI().vcx_credentialdef_deserialize
  private _name: string
  private _schemaId: string
  private _credDefId: string

  constructor (sourceId: string, { name, schemaId }: ICredentialDefParams) {
    super(sourceId)
    this._name = name
    this._schemaId = schemaId
    this._credDefId = null
  }

  /**
   * @memberof CredentialDef
   * @description creates a credential definition on the ledger and returns an associated object.
   * @static
   * @async
   * @function create
   * @param {ICredentialDefinition} data
   * @example <caption>Example of ICredentialDefinition</caption>
   * {
   *    sourceId: "12",
   *    schemaId: "2hoqvcwupRTUNkXn6ArYzs:2:test-licence:4.4.4",
   *    name: "test-licence",
   *    revocation: false,
   *    paymentHandle: 0
   * }
   * @returns {Promise<credentialDef>} A credentialDef Object
   */
  static async create ({
    name,
    paymentHandle,
    schemaId,
    sourceId
  }: ICredentialDefinition): Promise<CredentialDef> {
    // Todo: need to add params for tag and config
    const credentialDef = new CredentialDef(sourceId, { name, schemaId })
    const commandHandle = 0
    const issuerDid = null
    try {
      await credentialDef._create((cb) => rustAPI().vcx_credentialdef_create(
      commandHandle,
      sourceId,
      name,
      schemaId,
      issuerDid,
      'tag1',
      '{}',
      paymentHandle,
      cb
      ))
      return credentialDef
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), 'vcx_credentialdef_create')
    }
  }

  /**
   * @memberof CredentialDef
   * @description Builds a credentialDef object with defined attributes.
   * Attributes are provided by a previous call to the serialize function.
   * @static
   * @async
   * @function deserialize
   * @param {ICredentialDefObj} data - data obtained by serialize api. Used to build a credentialdef object.
   * @returns {Promise<credentialDef>} A credentialDef Object
   */
  static async deserialize (data: ICredentialDefObj) {
    // Todo: update the ICredentialDefObj
    try {
      const credentialDefParams = {
        name: data.name,
        schemaId: null
      }
      return await super._deserialize(CredentialDef, data, credentialDefParams)
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), 'vcx_credentialdef_deserialize')
    }
  }

  /**
   * @memberof CredentialDef
   * @description Serializes a credentialDef object.
   * Data returned can be used to recreate a credentialDef object by passing it to the deserialize function.
   * @async
   * @function serialize
   * @returns {Promise<ICredentialDefObj>} - Jason object with all of the underlying Rust attributes.
   * Same json object structure that is passed to the deserialize function.
   */
  async serialize (): Promise<ICredentialDefObj> {
    try {
      const data: ICredentialDefObj = JSON.parse(await super._serialize())
      return data
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), 'vcx_credentialdef_serialize')
    }
  }

  /**
   * @memberof CredentialDef
   * @description Retrieves the credential definition id associated with the created cred def.
   * @async
   * @function getCredDefId
   * @returns {Promise<string>} - CredDef's Identifier
   */
  async getCredDefId (): Promise<string> {
    try {
      const credDefId = await createFFICallbackPromise<string>(
          (resolve, reject, cb) => {
            const rc = rustAPI().vcx_credentialdef_get_cred_def_id(0, this.handle, cb)
            if (rc) {
              reject(rc)
            }
          },
          (resolve, reject) => ffi.Callback('void', ['uint32', 'uint32', 'string'],
          (xcommandHandle, err, credDefIdVal) => {
            if (err) {
              reject(err)
              return
            }
            this._credDefId = credDefIdVal
            resolve(credDefIdVal)
          })
        )
      return credDefId
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), 'vcx_credentialdef_get_cred_def_id')
    }
  }
}
