import * as ffi from 'ffi'
import { VCXInternalError } from '../errors'
import { rustAPI } from '../rustlib'
import { createFFICallbackPromise } from '../utils/ffi-helpers'
import { ISerializedData, StateType } from './common'
import { Connection } from './connection'
import { VCXBaseWithState } from './vcx-base-with-state'
import { PaymentManager } from './vcx-payment-txn'

export interface IIssuerCredentialCreateData {
  sourceId: string,
  credDefHandle: number,
  attr: {
    [ index: string ]: string
  },
  credentialName: string,
  price: string,
}

export interface IIssuerCredentialVCXAttributes {
  [ index: string ]: string
}

export interface IIssuerCredentialParams {
  credDefHandle: number,
  credentialName: string,
  attr: IIssuerCredentialVCXAttributes,
  price: string
}

/**
 * Interface that represents the attributes of an Issuer credential object.
 * This interface is expected as the type for deserialize's parameter and serialize's return value
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
  cred_def_handle: number
  price: string
  tails_file?: string
  cred_rev_id?: string
  rev_reg_id?: string
}

// tslint:disable max-classes-per-file
export class IssuerCredentialPaymentManager extends PaymentManager {
  protected _getPaymentTxnFn = rustAPI().vcx_issuer_credential_get_payment_txn
}

/**
 * A Credential created by the issuing party (institution)
 */
export class IssuerCredential extends VCXBaseWithState<IIssuerCredentialData> {
  /**
   * Builds a generic Issuer Credential object
   * ```
   * issuerCredential = await IssuerCredential.create({sourceId: "12",
   * credDefId: "credDefId", attr: {key: "value"}, credentialName: "name", price: "0"})
   * ```
   * @returns {Promise<IssuerCredential>} An Issuer credential Object
   */
  public static async create ({ attr, sourceId, credDefHandle,
                         credentialName, price }: IIssuerCredentialCreateData): Promise<IssuerCredential> {
    try {
      const attrsVCX: IIssuerCredentialVCXAttributes = attr
      const credential = new IssuerCredential(sourceId, { credDefHandle, credentialName, attr: attrsVCX, price })
      const attrsStringified = attrsVCX ? JSON.stringify(attrsVCX) : attrsVCX
      const commandHandle = 0
      const issuerDid = null
      await credential._create((cb) => rustAPI().vcx_issuer_create_credential(
        commandHandle,
        sourceId,
        credDefHandle,
        issuerDid,
        attrsStringified,
        credentialName,
        price,
        cb
        )
      )
      return credential
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

  /**
   * Builds an Issuer credential object with defined attributes.
   *
   * Attributes are provided by a previous call to the serialize function.
   * ```
   * issuerCredential = await IssuerCredential.create({sourceId: "12",
   * credDefId: "credDefId", attr: {key: "value"}, credentialName: "name", price: 0})
   * data1 = await issuerCredential.serialize()
   * issuerCredential2 = await IssuerCredential.deserialize(data1)
   * ```
   */
  public static async deserialize (credentialData: ISerializedData<IIssuerCredentialData>) {
    try {
      const { data: { credential_name, price, credential_attributes, cred_def_handle } } = credentialData
      const attr: IIssuerCredentialVCXAttributes = JSON.parse(credential_attributes)
      const params: IIssuerCredentialParams = {
        attr,
        credDefHandle: cred_def_handle,
        credentialName: credential_name,
        price
      }
      const credential = await super._deserialize<IssuerCredential, IIssuerCredentialParams>(
        IssuerCredential,
        credentialData,
        params
      )
      return credential
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

  public paymentManager!: IssuerCredentialPaymentManager
  protected _releaseFn = rustAPI().vcx_issuer_credential_release
  protected _updateStFn = rustAPI().vcx_issuer_credential_update_state
  protected _getStFn = rustAPI().vcx_issuer_credential_get_state
  protected _serializeFn = rustAPI().vcx_issuer_credential_serialize
  protected _deserializeFn = rustAPI().vcx_issuer_credential_deserialize
  private _credDefHandle: number
  private _credentialName: string
  private _attr: IIssuerCredentialVCXAttributes
  private _price: string

  constructor (sourceId: string, { credDefHandle, credentialName, attr, price }: IIssuerCredentialParams) {
    super(sourceId)
    this._credDefHandle = credDefHandle
    this._credentialName = credentialName
    this._attr = attr
    this._price = price
  }

  /**
   * Sends a credential Offer to the end user.
   *
   * A Credential Offer is made up of the data provided in the creation of this object
   *
   * Example:
   * ```
   * connection = await Connection.create({id: 'foobar'})
   * inviteDetails = await connection.connect()
   * issuerCredential = await IssuerCredential.create({sourceId: "12",
   *   credDefId: "credDefId", attr: {k    ey: "value"}, credentialName: "name", price: 0})
   * await issuerCredential.sendOffer(connection)
   * ```
   */
  public async sendOffer (connection: Connection): Promise<void> {
    try {
      await createFFICallbackPromise<void>(
          (resolve, reject, cb) => {
            const rc = rustAPI().vcx_issuer_send_credential_offer(0, this.handle, connection.handle, cb)
            if (rc) {
              reject(rc)
            }
          },
          (resolve, reject) => ffi.Callback(
            'void',
            ['uint32', 'uint32'],
            (xcommandHandle: number, err: number) => {
              if (err) {
                reject(err)
                return
              }
              resolve()
            })
        )
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

  /**
   *
   * Updates the state of the credential from the given message.
   *
   * Example:
   * ```
   * await object.updateStateWithMessage(message)
   * ```
   * @returns {Promise<void>}
   */
  public async updateStateWithMessage (message: string): Promise<void> {
    try {
      const commandHandle = 0
      await createFFICallbackPromise<number>(
        (resolve, reject, cb) => {
          const rc = rustAPI().vcx_issuer_credential_update_state_with_message(commandHandle, this.handle, message, cb)
          if (rc) {
            resolve(StateType.None)
          }
        },
        (resolve, reject) => ffi.Callback(
          'void',
          ['uint32', 'uint32', 'uint32'],
          (handle: number, err: any, state: StateType) => {
            if (err) {
              reject(err)
            }
            resolve(state)
          })
      )
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

  /**
   * Sends the credential to the end user.
   *
   * Credential is made up of the data sent during Credential Offer
   * ```
   * connection = await connectionCreateConnect()
   * issuerCredential = await issuerCredentialCreate()
   * await issuerCredential.sendOffer(connection)
   * await issuerCredential.updateState()
   * assert.equal(await issuerCredential.getState(), StateType.RequestReceived)
   * await issuerCredential.sendCredential(connection)
   * ```
   *
   */
  public async sendCredential (connection: Connection): Promise<void> {
    try {
      await createFFICallbackPromise<void>(
        (resolve, reject, cb) => {
          const rc = rustAPI().vcx_issuer_send_credential(0, this.handle, connection.handle, cb)
          if (rc) {
            reject(rc)
          }
        },
        (resolve, reject) => ffi.Callback(
          'void',
          ['uint32', 'uint32'],
          (xcommandHandle: number, err: number) => {
            if (err) {
              reject(err)
              return
            }
            resolve()
          })
      )
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

  /**
   * Revokes credential.
   *
   * Credential is made up of the data sent during Credential Offer
   * ```
   * connection = await connectionCreateConnect()
   * issuerCredential = await issuerCredentialCreate()
   * await issuerCredential.sendOffer(connection)
   * await issuerCredential.updateState()
   * assert.equal(await issuerCredential.getState(), StateType.RequestReceived)
   * await issuerCredential.sendCredential(connection)
   * await issuerCredential.revokeCredential()
   * ```
   *
   */
  public async revokeCredential (): Promise<void> {
    try {
      await createFFICallbackPromise<void>(
        (resolve, reject, cb) => {
          const rc = rustAPI().vcx_issuer_revoke_credential(0, this.handle, cb)
          if (rc) {
            reject(rc)
          }
        },
        (resolve, reject) => ffi.Callback(
          'void',
          ['uint32', 'uint32'],
          (xcommandHandle: number, err: number) => {
            if (err) {
              reject(err)
              return
            }
            resolve()
          })
      )
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

  get credDefHandle () {
    return this._credDefHandle
  }

  get attr () {
    return this._attr
  }

  get credentialName () {
    return this._credentialName
  }

  get price () {
    return this._price
  }

  protected _setHandle (handle: number) {
    super._setHandle(handle)
    this.paymentManager = new IssuerCredentialPaymentManager({ handle })
  }
}
