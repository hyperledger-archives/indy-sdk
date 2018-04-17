import { Callback } from 'ffi'

import { VCXInternalError } from '../errors'
import { rustAPI } from '../rustlib'
import { createFFICallbackPromise } from '../utils/ffi-helpers'
import { StateType } from './common'
import { Connection } from './connection'
import { VCXBase } from './VCXBase'
import { VCXBaseWithState } from './VCXBaseWithState'

export interface ICredentialStructData {
  source_id: string,
}

export type ICredentialOffer = string

export interface ICredentialCreateData {
  sourceId: string,
  offer: ICredentialOffer
}

export class Credential extends VCXBaseWithState {
  protected _releaseFn = rustAPI().vcx_credential_release
  protected _updateStFn = rustAPI().vcx_credential_update_state
  protected _getStFn = rustAPI().vcx_credential_get_state
  protected _serializeFn = rustAPI().vcx_credential_serialize
  protected _deserializeFn = rustAPI().vcx_credential_deserialize

  static async create ({ sourceId, offer }: ICredentialCreateData): Promise<Credential> {
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

  static async createWithMsgId (connection: Connection, sourceId, msgId): Promise<Credential> {
    const credential = new Credential(sourceId)
    try {
      await credential._create((cb) => rustAPI().vcx_credential_create_with_msgid(
        0,
        sourceId,
        connection.handle,
        msgId,
        cb
        )
      )
      return credential
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), `vcx_credential_create_with_msgid`)
    }
  }

  static async deserialize (credentialData: ICredentialStructData) {
    try {
      const credential = await super._deserialize<Credential, {}>(Credential, credentialData)
      return credential
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), `vcx_issuer_credential_deserialize`)
    }
  }

  static async getOffers (connection: Connection): Promise<ICredentialOffer[]> {
    const offersStr = await createFFICallbackPromise<string>(
      (resolve, reject, cb) => {
        const rc = rustAPI().vcx_credential_get_offers(0, connection.handle, cb)
        if (rc) {
          reject(rc)
        }
      },
      (resolve, reject) => Callback('void', ['uint32', 'uint32', 'string'], (handle, err, messages) => {
        if (err) {
          reject(err)
        } else {
          resolve(messages)
        }
      })
    )
    const offers = JSON.parse(offersStr)
    return offers
  }

  async getState (): Promise<StateType> {
    try {
      return await this._getState()
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), `vcx_credential_get_state`)
    }
  }

  async updateState (): Promise<void> {
    try {
      await this._updateState()
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), `vcx_credential_update_state`)
    }
  }

  async serialize (): Promise<ICredentialStructData> {
    try {
      return JSON.parse(await super._serialize())
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), `vcx_credential_serialize`)
    }
  }

  async sendRequest (connection: Connection): Promise<void> {
    try {
      await createFFICallbackPromise<void>(
          (resolve, reject, cb) => {
            const rc = rustAPI().vcx_credential_send_request(0, this.handle, connection.handle, cb)
            if (rc) {
              reject(rc)
            }
          },
          (resolve, reject) => Callback('void', ['uint32', 'uint32'], (xcommandHandle, err) => {
            if (err) {
              reject(err)
            } else {
              resolve()
            }
          })
        )
    } catch (err) {
      // TODO handle error
      throw new VCXInternalError(err, VCXBase.errorMessage(err), `vcx_credential_send_request`)
    }
  }
}
