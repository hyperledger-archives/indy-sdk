import { Callback } from 'ffi'

import { VCXInternalError } from '../errors'
import { rustAPI } from '../rustlib'
import { createFFICallbackPromise } from '../utils/ffi-helpers'
import { Connection } from './connection'
import { VCXBaseWithState } from './vcx-base-with-state'
import { VCXPaymentTxn } from './vcx-payment-txn'

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

class CredentialBase extends VCXBaseWithState<ICredentialStructData> {
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

  protected _releaseFn = rustAPI().vcx_credential_release
  protected _updateStFn = rustAPI().vcx_credential_update_state
  protected _getStFn = rustAPI().vcx_credential_get_state
  protected _serializeFn = rustAPI().vcx_credential_serialize
  protected _deserializeFn = rustAPI().vcx_credential_deserialize
  protected _getPaymentTxnFn = rustAPI().vcx_credential_get_payment_txn
  protected _credOffer: string = ''

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
}

// tslint:disable max-classes-per-file
export class Credential extends VCXPaymentTxn(CredentialBase) {
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
              const handleStr = handleNum.toString()
              newObj._setHandle(handleStr)
              newObj._credOffer = credOffer
              resolve(newObj)
            })
      )
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

  public static async deserialize (credentialData: ICredentialStructData) {
    const credential = await super._deserialize<Credential, {}>(Credential, credentialData)
    return credential
  }
}
