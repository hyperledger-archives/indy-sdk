import * as ffi from 'ffi'
import { VCXInternalError } from '../errors'
import { rustAPI } from '../rustlib'
import { createFFICallbackPromise } from '../utils/ffi-helpers'
import { ISerializedData } from './common'
import { VCXBase } from './vcx-base'
import { PaymentManager } from './vcx-payment-txn'

/**
 * @interface
 * @description
 * SourceId: String for SDK User's reference.
 * name: name of credentialdef.
 * schemaNo: Schema Number wanted to create credentialdef off of
 * revocation:
 */
export interface ICredentialDefCreateData {
  sourceId: string,
  name: string,
  schemaId: string,
  revocationDetails: IRevocationDetails,
  paymentHandle: number
}

export interface ICredentialDefData {
  source_id: string,
  handle: number
  name: string
  credential_def: ICredentialDefDataObj
}

export interface ICredentialDefDataObj {
  ref: number,
  origin: string,
  signature_type: string,
  data: any,
}

export interface ICredentialDefParams {
  schemaId: string,
  name: string,
  tailsFile?: string
}

export interface IRevocationDetails {
  maxCreds?: number,
  supportRevocation?: boolean,
  tailsFile?: string,
}

// tslint:disable max-classes-per-file
export class CredentialDefPaymentManager extends PaymentManager {
  protected _getPaymentTxnFn = rustAPI().vcx_credentialdef_get_payment_txn
}

/**
 * @class Class representing a credential Definition
 */
export class CredentialDef extends VCXBase<ICredentialDefData> {
  /**
   * Creates a new CredentialDef object that is written to the ledger
   *
   * Example:
   * ```
   * data = {
   *   name: 'testCredentialDefName',
   *   paymentHandle: 0,
   *   revocation: false,
   *   schemaId: 'testCredentialDefSchemaId',
   *   sourceId: 'testCredentialDefSourceId'
   * }
   * credentialDef = await CredentialDef.create(data)
   * ```
   */
  public static async create ({
    name,
    paymentHandle,
    revocationDetails,
    schemaId,
    sourceId
  }: ICredentialDefCreateData): Promise<CredentialDef> {
    // Todo: need to add params for tag and config
    const tailsFile = revocationDetails.tailsFile
    const credentialDef = new CredentialDef(sourceId, { name, schemaId, tailsFile })
    const commandHandle = 0
    const issuerDid = null
    const revocation = {
      max_creds: revocationDetails.maxCreds,
      support_revocation: revocationDetails.supportRevocation,
      tails_file: revocationDetails.tailsFile
    }

    try {
      await credentialDef._create((cb) => rustAPI().vcx_credentialdef_create(
        commandHandle,
        sourceId,
        name,
        schemaId,
        issuerDid,
        'tag1',
        JSON.stringify(revocation),
        paymentHandle,
      cb
      ))
      return credentialDef
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

  /**
   * Builds a credentialDef object with defined attributes.
   * Attributes are provided by a previous call to the serialize function.
   * Example:
   * ```
   * data = {
   *   name: 'testCredentialDefName',
   *   paymentHandle: 0,
   *   revocation: false,
   *   schemaId: 'testCredentialDefSchemaId',
   *   sourceId: 'testCredentialDefSourceId'
   * }
   * credentialDef = await CredentialDef.create(data)
   * data1 = await credentialDef.serialize()
   * credentialDef2 = await CredentialDef.deserialzie(data1)
   * ```
   */
  public static async deserialize (credentialDef: ISerializedData<ICredentialDefData>) {
    // Todo: update the ICredentialDefObj
    const { data: { name } } = credentialDef
    const credentialDefParams = {
      name,
      schemaId: null
    }
    return super._deserialize(CredentialDef, credentialDef, credentialDefParams)
  }

  public paymentManager!: CredentialDefPaymentManager
  protected _releaseFn = rustAPI().vcx_credentialdef_release
  protected _serializeFn = rustAPI().vcx_credentialdef_serialize
  protected _deserializeFn = rustAPI().vcx_credentialdef_deserialize
  private _name: string
  private _schemaId: string
  private _credDefId: string | null
  private _tailsFile: string | undefined

  constructor (sourceId: string, { name, schemaId, tailsFile }: ICredentialDefParams) {
    super(sourceId)
    this._name = name
    this._schemaId = schemaId
    this._credDefId = null
    this._tailsFile = tailsFile
  }

  /**
   * Retrieves the credential definition id associated with the created cred def.
   * Example:
   * ```
   * data = {
   *   name: 'testCredentialDefName',
   *   paymentHandle: 0,
   *   revocation: false,
   *   schemaId: 'testCredentialDefSchemaId',
   *   sourceId: 'testCredentialDefSourceId'
   * }
   * credentialDef = await CredentialDef.create(data)
   * id = await credentialDef.getCredDefId()
   * ```
   */
  public async getCredDefId (): Promise<string> {
    try {
      const credDefId = await createFFICallbackPromise<string>(
          (resolve, reject, cb) => {
            const rc = rustAPI().vcx_credentialdef_get_cred_def_id(0, this.handle, cb)
            if (rc) {
              reject(rc)
            }
          },
          (resolve, reject) => ffi.Callback(
            'void',
            ['uint32', 'uint32', 'string'],
            (xcommandHandle: number, err: number, credDefIdVal: string) => {
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
      throw new VCXInternalError(err)
    }
  }

  get name () {
    return this._name
  }

  get schemaId () {
    return this._schemaId
  }

  get credDefId () {
    return this._credDefId
  }

  get tailsFile () {
    return this._tailsFile
  }

  protected _setHandle (handle: number) {
    super._setHandle(handle)
    this.paymentManager = new CredentialDefPaymentManager({ handle })
  }
}
