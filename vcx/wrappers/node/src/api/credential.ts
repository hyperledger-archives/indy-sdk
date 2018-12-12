import { Callback } from 'ffi'

import { VCXInternalError } from '../errors'
import { rustAPI } from '../rustlib'
import { createFFICallbackPromise } from '../utils/ffi-helpers'
import { ISerializedData } from './common'
import { Connection } from './connection'
import { VCXBaseWithState } from './vcx-base-with-state'
import { PaymentManager } from './vcx-payment-txn'

export interface ICredentialStructData {
  source_id: string,
}

export type ICredentialOffer = [ object, object ]

export interface ICredentialCreateWithOffer {
  sourceId: string,
  offer: string,
  // We're going to need it in the future
  connection: Connection
}

export interface ICredentialCreateWithMsgId {
  sourceId: string,
  msgId: string,
  connection: Connection
}

export interface ICredentialSendData {
  connection: Connection,
  payment: number
}

// tslint:disable max-classes-per-file
export class CredentialPaymentManager extends PaymentManager {
  protected _getPaymentTxnFn = rustAPI().vcx_credential_get_payment_txn
}

/**
 * A Credential Object, which is issued by the issuing party to the prover and stored in the prover's wallet.
 */
export class Credential extends VCXBaseWithState<ICredentialStructData> {
  /**
   * Creates a credential with an offer.
   *
   * * Requires a credential offer to be submitted to prover.
   *
   * ```
   * credentialOffer = [
   *   {
   *     claim_id: 'defaultCredentialId',
   *     claim_name: 'Credential',
   *     cred_def_id: 'id',
   *     credential_attrs: {
   *     address1: ['101 Tela Lane'],
   *     address2: ['101 Wilson Lane'],
   *     city: ['SLC'],
   *     state: ['UT'],
   *     zip: ['87121']
   *   },
   *   from_did: '8XFh8yBzrpJQmNyZzgoTqB',
   *   libindy_offer: '{}',
   *   msg_ref_id: '123',
   *   msg_type: 'CLAIM_OFFER',
   *   schema_seq_no: 1487,
   *   to_did: '8XFh8yBzrpJQmNyZzgoTqB',
   *   version: '0.1'
   * },
   * {
   *   payment_addr: 'pov:null:OsdjtGKavZDBuG2xFw2QunVwwGs5IB3j',
   *   payment_required: 'one-time',
   *   price: 5
   * }]
   *
   * {
   *   JSON.stringify(credentialOffer),
   *   'testCredentialSourceId'
   * }
   * credential = Credential.create(data)
   * ```
   *
   */
  public static async create ({ sourceId, offer }: ICredentialCreateWithOffer): Promise<Credential> {
    const credential = new Credential(sourceId)
    try {
      await credential._create((cb) => rustAPI().vcx_credential_create_with_offer(
        0,
        sourceId,
        offer,
        cb
        )
      )
      return credential
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

  /**
   * Create a Credential object that requests and receives a credential for an institution
   *
   * ```
   * credential = Credential.createWithMsgId({
   *   connection,
   *   msgId: 'testCredentialMsgId',
   *   sourceId: 'testCredentialSourceId'
   * })
   * ```
   */
  public static async createWithMsgId (
    { connection, sourceId, msgId }: ICredentialCreateWithMsgId
  ): Promise<Credential> {
    try {
      return await createFFICallbackPromise<Credential>(
          (resolve, reject, cb) => {
            const rc = rustAPI().vcx_credential_create_with_msgid(0, sourceId, connection.handle, msgId, cb)
            if (rc) {
              reject(rc)
            }
          },
          (resolve, reject) => Callback(
            'void',
            ['uint32', 'uint32', 'uint32', 'string'],
            (xHandle: number, err: number, handleNum: number, credOffer: string) => {
              if (err) {
                reject(err)
                return
              }
              const newObj = new Credential(sourceId)
              newObj._setHandle(handleNum)
              newObj._credOffer = credOffer
              resolve(newObj)
            })
      )
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }
  /**
   * Create an object from a JSON Structured data produced from the objects serialize method
   *
   * ```
   * data = credential.deserialize()
   * ```
   */
  public static async deserialize (credentialData: ISerializedData<ICredentialStructData>) {
    const credential = await super._deserialize<Credential, {}>(Credential, credentialData)
    return credential
  }
  /**
   * Retrieves all pending credential offers.
   *
   * ```
   * connection = await Connection.create({id: 'foobar'})
   * inviteDetails = await connection.connect()
   * offers = await Credential.getOffers(connection)
   * ```
   */
  public static async getOffers (connection: Connection): Promise<ICredentialOffer[]> {
    try {
      const offersStr = await createFFICallbackPromise<string>(
        (resolve, reject, cb) => {
          const rc = rustAPI().vcx_credential_get_offers(0, connection.handle, cb)
          if (rc) {
            reject(rc)
          }
        },
        (resolve, reject) => Callback(
          'void',
          ['uint32', 'uint32', 'string'],
          (handle: number, err: number, messages: string) => {
            if (err) {
              reject(err)
              return
            }
            resolve(messages)
          })
      )
      const offers: ICredentialOffer[] = JSON.parse(offersStr)
      return offers
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

  public paymentManager!: CredentialPaymentManager
  protected _releaseFn = rustAPI().vcx_credential_release
  protected _updateStFn = rustAPI().vcx_credential_update_state
  protected _getStFn = rustAPI().vcx_credential_get_state
  protected _serializeFn = rustAPI().vcx_credential_serialize
  protected _deserializeFn = rustAPI().vcx_credential_deserialize
  protected _credOffer: string = ''

  /**
   * Approves the credential offer and submits a credential request.
   * The result will be a credential stored in the prover's wallet.
   *
   * ```
   * connection = await Connection.create({id: 'foobar'})
   * inviteDetails = await connection.connect()
   * credential = Credential.create(data)
   * await credential.sendRequest({ connection, 1000 })
   * ```
   *
   */
  public async sendRequest ({ connection, payment }: ICredentialSendData): Promise<void> {
    try {
      await createFFICallbackPromise<void>(
          (resolve, reject, cb) => {
            const rc = rustAPI().vcx_credential_send_request(0, this.handle, connection.handle, payment, cb)
            if (rc) {
              reject(rc)
            }
          },
          (resolve, reject) => Callback('void', ['uint32', 'uint32'], (xcommandHandle: number, err: number) => {
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

  get credOffer (): string {
    return this._credOffer
  }
  /**
   * Retrieve Payment Transaction Information for this Credential. Typically this will include
   * how much payment is requried by the issuer, which needs to be provided by the prover, before
   * the issuer will issue the credential to the prover. Ideally a prover would want to know
   * how much payment is being asked before submitting the credential request (which triggers
   * the payment to be made).
   * ```
   * EXAMPLE HERE
   * ```
   */
  public async getPaymentInfo (): Promise<string> {
    try {
      return await createFFICallbackPromise<string>(
          (resolve, reject, cb) => {
            const rc = rustAPI().vcx_credential_get_payment_info(0, this.handle, cb)
            if (rc) {
              reject(rc)
            }
          },
          (resolve, reject) => Callback('void', ['uint32', 'uint32', 'string'],
          (xcommandHandle: number, err: number, info: any) => {
            if (err) {
              reject(err)
            } else {
              resolve(info)
            }
          })
        )
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

  protected _setHandle (handle: number) {
    super._setHandle(handle)
    this.paymentManager = new CredentialPaymentManager({ handle })
  }
}
