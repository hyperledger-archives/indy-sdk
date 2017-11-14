import { Callback, ForeignFunction } from 'ffi'

import { CXSInternalError } from '../errors'
import { rustAPI } from '../rustlib'
import { createFFICallbackPromise } from '../utils/ffi-helpers'
import { GCWatcher } from '../utils/memory-management-helpers'
import { StateType } from './common'

export interface IProofData {
  source_id: string
  handle: number
  proof_attributes: string
  proof_requester_did: string
  proover_did: string
  state: StateType
}

export class Proof extends GCWatcher {
  protected _releaseFn = rustAPI().cxs_proof_release
  protected _handle: string
  private _attr: string
  private _sourceId: string
  private _state: number
  private _RUST_API: { [ index: string ]: ForeignFunction }
  private _proofRequesterDid: string

  constructor (sourceId) {
    super()
    this._sourceId = sourceId
    this._handle = null
    this._state = StateType.None
    this._proofRequesterDid = null
    this._attr = null
  }

  static async create (sourceId: string, did: string, attributes: string): Promise<Proof> {
    const proof = new Proof(sourceId)
    await proof.init(sourceId, did, attributes)
    return proof
  }

  static async deserialize (proofData: IProofData): Promise<Proof> {
    const proof = new Proof(proofData.source_id)
    await proof._initFromProofData(proofData)
    return proof
  }

  async serialize (): Promise<Proof> {
    const proofHandle = this._handle
    let rc = null
    try {
      const data = await createFFICallbackPromise<string>(
          (resolve, reject, cb) => {
            rc = rustAPI().cxs_proof_serialize(0, proofHandle, cb)
            if (rc) {
              reject(rc)
            }
          },
          (resolve, reject) => Callback('void', ['uint32', 'uint32', 'string'], (handle, err, serializedProof) => {
            if (err) {
              reject(err)
              return
            } else if (serializedProof == null) {
              reject('no claim to serialize')
            }
            resolve(serializedProof)
          })
      )
      return JSON.parse(data)
    } catch (err) {
      throw new CXSInternalError(`cxs_proof_serialize -> ${rc}`)
    }
  }

  getProofRequesterDid () {
    return this._proofRequesterDid
  }
  getSourceId () {
    return this._sourceId
  }

  getProofHandle () {
    return this._handle
  }

  getAttr () {
    return this._attr
  }

  getState () {
    return this._state
  }

  private _setState (state) {
    this._state = state
  }

  private async init (sourceId: string, did: string, attr: string): Promise<void> {
    this._attr = attr
    this._sourceId = sourceId
    this._proofRequesterDid = did
    try {
      const data = await createFFICallbackPromise<string>(
          (resolve, reject, cb) => {
            rustAPI().cxs_proof_create(0, this._sourceId, this._proofRequesterDid, this._attr, cb)
          },
          (resolve, reject) => Callback('void', ['uint32', 'uint32', 'uint32'], (commandHandle, err, proofHandle) => {
            if (err) {
              reject(err)
              return
            }
            const value = JSON.stringify(proofHandle)
            resolve(Number(value))
          })
        )
      this._setHandle(data)
      // Todo: when updateState is working, call that instead of explicitly hardcoding the state
      this._setState(StateType.Initialized)
    //   await this.updateState()
    } catch (err) {
      throw new CXSInternalError(`cxs_proof_create -> ${err}`)
    }
  }

  private async _initFromProofData (proofData: IProofData): Promise<void> {
    try {
      const xproofHandle = await createFFICallbackPromise<string>(
          (resolve, reject, cb) => {
            rustAPI().cxs_proof_deserialize(0, JSON.stringify(proofData), cb)
          },
          (resolve, reject) => Callback('void', ['uint32', 'uint32', 'uint32'], (commandHandle, err, proofHandle) => {
            if (err) {
              reject(err)
              return
            }
            resolve(proofHandle)
          })
      )
      this._setHandle(xproofHandle)
    //   await this.updateState()
    } catch (err) {
      throw new CXSInternalError(`cxs_proof_deserialize -> ${err}`)
    }
  }
}
