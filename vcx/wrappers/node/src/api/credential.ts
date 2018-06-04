import { Callback } from 'ffi'

import { VCXInternalError } from '../errors'
import { rustAPI } from '../rustlib'
import { createFFICallbackPromise } from '../utils/ffi-helpers'
import { Connection } from './connection'
import { VCXBase } from './VCXBase'
import { VCXBaseWithState } from './VCXBaseWithState'

export interface ICredentialStructData {
  source_id: string,
}

export type ICredentialOffer = string

export interface ICredentialCreateWithOffer {
  sourceId: string,
  offer: ICredentialOffer,
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

export class Credential extends VCXBaseWithState<ICredentialStructData> {
  protected _releaseFn = rustAPI().vcx_credential_release
  protected _updateStFn = rustAPI().vcx_credential_update_state
  protected _getStFn = rustAPI().vcx_credential_get_state
  protected _serializeFn = rustAPI().vcx_credential_serialize
  protected _deserializeFn = rustAPI().vcx_credential_deserialize
  private _credOffer: string = ''

  static async create ({ sourceId, offer }: ICredentialCreateWithOffer): Promise<Credential> {
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
      throw new VCXInternalError(err, VCXBase.errorMessage(err), `vcx_credential_create_with_offer`)
    }
  }

  static async createWithMsgId ({ connection, sourceId, msgId }: ICredentialCreateWithMsgId): Promise<Credential> {
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
              resolve( newObj )
            })
      )
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), `vcx_credential_create_with_msgid`)
    }
  }

  static async deserialize (credentialData: ICredentialStructData) {
    const credential = await super._deserialize<Credential, {}>(Credential, credentialData)
    return credential
  }

  static async getOffers (connection: Connection): Promise<ICredentialOffer[]> {
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
  }

  async sendRequest ({ connection, payment }: ICredentialSendData): Promise<void> {
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
      throw new VCXInternalError(err, VCXBase.errorMessage(err), `vcx_credential_send_request`)
    }
  }

  get credOffer (): string {
    return this._credOffer
  }
}
