import { Callback, ForeignFunction } from 'ffi'
import * as weak from 'weak'
import { CXSRuntime, CXSRuntimeConfig } from '../index'
import { createFFICallbackPromise, IProofData, StateType } from './api'
// import { Connection } from './connection'
import { CXSInternalError } from './errors'

export class Proof {
  private _attr: string
  private _sourceId: string
  private _proofHandle: number
  private _state: number
  private _RUST_API: { [ index: string ]: ForeignFunction }
  private _proofRequesterDid: string

  constructor (sourceId) {
    this._sourceId = sourceId
    this._initRustApi(null)
    this._proofHandle = null
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
    const proofHandle = this._proofHandle
    let rc = null
    try {
      const data = await createFFICallbackPromise<string>(
          (resolve, reject, cb) => {
            rc = this._RUST_API.cxs_proof_serialize(0, proofHandle, cb)
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

  async release (): Promise<number> {
    return this._RUST_API.cxs_proof_release(this._proofHandle)
  }

  getProofRequesterDid () {
    return this._proofRequesterDid
  }
  getSourceId () {
    return this._sourceId
  }

  getProofHandle () {
    return this._proofHandle
  }

  getAttr () {
    return this._attr
  }

  setProofHandle (handle) {
    this._proofHandle = handle
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
      const data = await createFFICallbackPromise<number>(
          (resolve, reject, cb) => {
            this._RUST_API.cxs_proof_create(0, this._sourceId, this._proofRequesterDid, this._attr, cb)
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
      this.setProofHandle(data)
      // Todo: when updateState is working, call that instead of explicitly hardcoding the state
      this._setState(StateType.Initialized)
    //   await this.updateState()
    } catch (err) {
      throw new CXSInternalError(`cxs_proof_create -> ${err}`)
    }
  }

  private async _initFromProofData (proofData: IProofData): Promise<void> {
    try {
      const xproofHandle = await createFFICallbackPromise<number>(
          (resolve, reject, cb) => {
            this._RUST_API.cxs_proof_deserialize(0, JSON.stringify(proofData), cb)
          },
          (resolve, reject) => Callback('void', ['uint32', 'uint32', 'uint32'], (commandHandle, err, proofHandle) => {
            if (err) {
              reject(err)
              return
            }
            resolve(proofHandle)
          })
      )
      this.setProofHandle(xproofHandle)
    //   await this.updateState()
    } catch (err) {
      throw new CXSInternalError(`cxs_proof_deserialize -> ${err}`)
    }
  }

  private _initRustApi (path?) {
    this._RUST_API = new CXSRuntime(new CXSRuntimeConfig(path))._ffi
  }

  private _clearOnExit () {
    // Todo: need to add proof release and issuer_claim release
    const weakRef = weak(this)
    const release = this._RUST_API.cxs_connection_release
    const handle = this._proofHandle
    weak.addCallback(weakRef, () => {
      release(handle)
    })
  }
}
