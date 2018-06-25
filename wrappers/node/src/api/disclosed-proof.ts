import { Callback } from 'ffi'

import { VCXInternalError } from '../errors'
import { rustAPI } from '../rustlib'
import { createFFICallbackPromise } from '../utils/ffi-helpers'
import { Connection } from './connection'
import { VCXBaseWithState } from './vcx-base-with-state'

export interface IDisclosedProofData {
  source_id: string,
}

export type IDisclosedProofRequest = object

export interface IDisclosedProofCreateData {
  // We're going to need it in the future
  connection: Connection,
  sourceId: string,
  request: string
}

export interface IDisclosedProofCreateWithMsgIdData {
  connection: Connection,
  msgId: string,
  sourceId: string
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

export interface IGenerateProofData {
  selectedCreds: {
    [index: string]: ICredData
  },
  selfAttestedAttrs: {
    [index: string]: string
  }
}

export class DisclosedProof extends VCXBaseWithState<IDisclosedProofData> {
  public static async create ({ sourceId, request }: IDisclosedProofCreateData): Promise<DisclosedProof> {
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
      throw new VCXInternalError(err)
    }
  }

  public static async createWithMsgId ({ connection, sourceId, msgId }: IDisclosedProofCreateWithMsgIdData):
  Promise<DisclosedProof> {
    try {
      return await createFFICallbackPromise<DisclosedProof>(
          (resolve, reject, cb) => {
            const rc = rustAPI().vcx_disclosed_proof_create_with_msgid(0, sourceId, connection.handle, msgId, cb)
            if (rc) {
              reject(rc)
            }
          },
          (resolve, reject) => Callback(
            'void',
            ['uint32', 'uint32', 'uint32', 'string'],
            (xHandle: number, err: number, handle: number, proofReq: string) => {

              if (err) {
                reject(err)
                return
              }
              const newObj = new DisclosedProof(sourceId)
              const handleStr = handle.toString()
              newObj._setHandle(handleStr)
              newObj._proofReq = proofReq
              resolve(newObj)
            })
      )
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

  public static async deserialize (data: IDisclosedProofData) {
    try {
      const newObj = await super._deserialize<DisclosedProof, {}>(DisclosedProof, data)
      return newObj
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

  public static async getRequests (connection: Connection): Promise<IDisclosedProofRequest[]> {
    const requestsStr = await createFFICallbackPromise<string>(
      (resolve, reject, cb) => {
        const rc = rustAPI().vcx_disclosed_proof_get_requests(0, connection.handle, cb)
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
    const requests = JSON.parse(requestsStr)
    return requests
  }

  protected _releaseFn = rustAPI().vcx_disclosed_proof_release
  protected _updateStFn = rustAPI().vcx_disclosed_proof_update_state
  protected _getStFn = rustAPI().vcx_disclosed_proof_get_state
  protected _serializeFn = rustAPI().vcx_disclosed_proof_serialize
  protected _deserializeFn = rustAPI().vcx_disclosed_proof_deserialize
  private _proofReq: string = ''

  public async getCredentials (): Promise<IRetrievedCreds> {
    try {
      const credsStr = await createFFICallbackPromise<string>(
          (resolve, reject, cb) => {
            const rc = rustAPI().vcx_disclosed_proof_retrieve_credentials(0, this.handle, cb)
            if (rc) {
              reject(rc)
            }
          },
          (resolve, reject) => Callback(
            'void',
            ['uint32', 'uint32', 'string'],
            (xcommandHandle: number, err: number, credsRet: string) => {
              if (err) {
                reject(err)
                return
              }
              resolve(credsRet)
            })
        )
      const creds: IRetrievedCreds = JSON.parse(credsStr)
      return creds
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

  public async sendProof (connection: Connection): Promise<void> {
    try {
      await createFFICallbackPromise<void>(
          (resolve, reject, cb) => {
            const rc = rustAPI().vcx_disclosed_proof_send_proof(0, this.handle, connection.handle, cb)
            if (rc) {
              reject(rc)
            }
          },
          (resolve, reject) => Callback(
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

  public async generateProof ({ selectedCreds, selfAttestedAttrs }: IGenerateProofData): Promise<void> {
    try {
      await createFFICallbackPromise<void>(
          (resolve, reject, cb) => {
            const rc = rustAPI().vcx_disclosed_proof_generate_proof(
              0,
              this.handle,
              JSON.stringify(selectedCreds),
              JSON.stringify(selfAttestedAttrs),
              cb
            )
            if (rc) {
              reject(rc)
            }
          },
          (resolve, reject) => Callback(
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

  get proofRequest (): string {
    return this._proofReq
  }
}
