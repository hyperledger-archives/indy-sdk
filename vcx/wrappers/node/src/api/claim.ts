import { Callback } from 'ffi'

import { VCXInternalError } from '../errors'
import { rustAPI } from '../rustlib'
import { createFFICallbackPromise } from '../utils/ffi-helpers'
import { StateType } from './common'
import { Connection } from './connection'
import { VCXBase } from './VCXBase'
import { VCXBaseWithState } from './VCXBaseWithState'

export interface IClaimStructData {
  source_id: string,
}

export type IClaimOffer = string

export interface IClaimCreateData {
  sourceId: string,
  offer: IClaimOffer
}

export class Claim extends VCXBaseWithState {
  protected _releaseFn = rustAPI().vcx_claim_release
  protected _updateStFn = rustAPI().vcx_claim_update_state
  protected _getStFn = rustAPI().vcx_claim_get_state
  protected _serializeFn = rustAPI().vcx_claim_serialize
  protected _deserializeFn = rustAPI().vcx_claim_deserialize

  static async create ({ sourceId, offer }: IClaimCreateData): Promise<Claim> {
    const claim = new Claim(sourceId)
    try {
      await claim._create((cb) => rustAPI().vcx_claim_create_with_offer(
        0,
        sourceId,
        offer,
        cb
        )
      )
      return claim
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), `vcx_claim_create_with_offer`)
    }
  }

  static async deserialize (claimData: IClaimStructData) {
    try {
      const claim = await super._deserialize<Claim, {}>(Claim, claimData)
      return claim
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), `vcx_issuer_claim_deserialize`)
    }
  }

  static async get_offers (connection: Connection): Promise<IClaimOffer[]> {
    const offersStr = await createFFICallbackPromise<string>(
      (resolve, reject, cb) => {
        const rc = rustAPI().vcx_claim_get_offers(0, connection.handle, cb)
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
      throw new VCXInternalError(err, VCXBase.errorMessage(err), `vcx_claim_get_state`)
    }
  }

  async updateState (): Promise<void> {
    try {
      await this._updateState()
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), `vcx_claim_update_state`)
    }
  }

  async serialize (): Promise<IClaimStructData> {
    try {
      return JSON.parse(await super._serialize())
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), `vcx_claim_serialize`)
    }
  }

  async sendRequest (connection: Connection): Promise<void> {
    try {
      await createFFICallbackPromise<void>(
          (resolve, reject, cb) => {
            const rc = rustAPI().vcx_claim_send_request(0, this.handle, connection.handle, cb)
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
      throw new VCXInternalError(err, VCXBase.errorMessage(err), `vcx_claim_send_request`)
    }
  }
}
