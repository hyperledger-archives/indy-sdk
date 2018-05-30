import { Callback } from 'ffi'

import { VCXInternalError } from '../errors'
import { rustAPI } from '../rustlib'
import { createFFICallbackPromise } from '../utils/ffi-helpers'
import { StateType } from './common'
import { Connection } from './connection'
import { VCXBase } from './VCXBase'
import { VCXBaseWithState } from './VCXBaseWithState'

export interface IDisclosedProofData {
  source_id: string,
}

export type IDisclosedProofRequest = string

export interface IDisclosedProofCreateData {
  sourceId: string,
  request: IDisclosedProofRequest
}

export interface IRetrievedCreds {
  attrs: {
    [ index: string ]: ICredData[]
  },
  predicates: any
}

export interface ICredData {
  cred_info: {
    [ index: string ]: any
  },
  interval: any
}

export class DisclosedProof extends VCXBaseWithState {
  protected _releaseFn = rustAPI().vcx_disclosed_proof_release
  protected _updateStFn = rustAPI().vcx_disclosed_proof_update_state
  protected _getStFn = rustAPI().vcx_disclosed_proof_get_state
  protected _serializeFn = rustAPI().vcx_disclosed_proof_serialize
  protected _deserializeFn = rustAPI().vcx_disclosed_proof_deserialize
  private _proofReq: string

  static async create ({ sourceId, request }: IDisclosedProofCreateData): Promise<DisclosedProof> {
    const newObj = new DisclosedProof(sourceId)
    try {
      await newObj._create((cb) => rustAPI().vcx_disclosed_proof_create_with_request(
        0,
        sourceId,
        request,
        cb
        )
      )
      return newObj
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), `vcx_disclosed_proof_create_with_request`)
    }
  }

  static async createWithMsgId (connection: Connection, sourceId, msgId): Promise<DisclosedProof> {
    try {
      return await createFFICallbackPromise<DisclosedProof>(
          (resolve, reject, cb) => {
            const rc = rustAPI().vcx_disclosed_proof_create_with_msgid(0, sourceId, connection.handle, msgId, cb)
            if (rc) {
              reject(rc)
            }
          },
          (resolve, reject) => Callback('void', ['uint32', 'uint32', 'uint32', 'string'],
          (xHandle, err, handle, proofReq) => {

            if (err) {
              reject(err)
              return
            }
            const newObj = new DisclosedProof(sourceId)
            newObj._setHandle(handle)
            newObj._setProofRequest(proofReq)
            resolve( newObj )
          })
      )
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), `vcx_disclosed_proof_create_with_msgid`)
    }
  }

  static async deserialize (data: IDisclosedProofData) {
    try {
      const newObj = await super._deserialize<DisclosedProof, {}>(DisclosedProof, data)
      return newObj
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), `vcx_disclosed_proof_deserialize`)
    }
  }

  static async getRequests (connection: Connection): Promise<IDisclosedProofRequest[]> {
    const requestsStr = await createFFICallbackPromise<string>(
      (resolve, reject, cb) => {
        const rc = rustAPI().vcx_disclosed_proof_get_requests(0, connection.handle, cb)
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
    const requests = JSON.parse(requestsStr)
    return requests
  }

  async getState (): Promise<StateType> {
    try {
      return await this._getState()
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), `vcx_disclosed_proof_get_state`)
    }
  }

  async updateState (): Promise<void> {
    try {
      await this._updateState()
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), `vcx_disclosed_proof_update_state`)
    }
  }

  async serialize (): Promise<IDisclosedProofData> {
    try {
      return JSON.parse(await super._serialize())
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), `vcx_disclosed_proof_serialize`)
    }
  }

  async getCredentials (): Promise<IRetrievedCreds> {
    try {
      return await createFFICallbackPromise<IRetrievedCreds>(
          (resolve, reject, cb) => {
            const rc = rustAPI().vcx_disclosed_proof_retrieve_credentials(0, this.handle, cb)
            if (rc) {
              reject(rc)
            }
          },
          (resolve, reject) => Callback('void', ['uint32', 'uint32', 'string'], (xcommandHandle, err, creds) => {
            if (err) {
              reject(err)
            } else {
              resolve(JSON.parse(creds))
            }
          })
        )
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), `vcx_disclosed_proof_retrieve_credentials`)
    }
  }

  async sendProof (connection: Connection): Promise<void> {
    try {
      await createFFICallbackPromise<void>(
          (resolve, reject, cb) => {
            const rc = rustAPI().vcx_disclosed_proof_send_proof(0, this.handle, connection.handle, cb)
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
      throw new VCXInternalError(err, VCXBase.errorMessage(err), `vcx_disclosed_proof_send_proof`)
    }
  }

  async generateProof (selectedCreds: {[index: string]: ICredData},
                       selfAttestedAttrs: {[index: string]: string}): Promise<void> {
    try {
      await createFFICallbackPromise<void>(
          (resolve, reject, cb) => {
            const rc = rustAPI().vcx_disclosed_proof_generate_proof(0,
                                                                    this.handle,
                                                                    JSON.stringify(selectedCreds),
                                                                    JSON.stringify(selfAttestedAttrs), cb)
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
      throw new VCXInternalError(err, VCXBase.errorMessage(err), `vcx_disclosed_proof_generate_proof`)
    }
  }

  getProofRequest (): string {
    return this._proofReq
  }

  _setProofRequest (proofReq: string) {
    this._proofReq = proofReq
  }
}
