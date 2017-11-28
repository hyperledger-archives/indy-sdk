import { CXSInternalError } from '../errors'
import { rustAPI } from '../rustlib'
import { StateType } from './common'
import { CXSBase } from './CXSBase'

export interface IProofData {
  source_id: string
  handle: number
  proof_attributes: string
  proof_requester_did: string
  proover_did: string
  state: StateType
}

export class Proof extends CXSBase {
  protected _releaseFn = rustAPI().cxs_proof_release
  // protected _updateStFn = rustAPI().cxs_proof_update_state
  protected _updateStFn = null
  protected _serializeFn = rustAPI().cxs_proof_serialize
  protected _deserializeFn = rustAPI().cxs_proof_deserialize
  private _proofRequesterDid: string

  constructor (sourceId) {
    super()
    this._sourceId = sourceId
    this._proofRequesterDid = null
  }

  static async create (sourceId: string, did: string, attributes: string): Promise<Proof> {
    const proof = new Proof(sourceId)
    const commandHandle = 0
    try {
      await proof._create((cb) => rustAPI().cxs_proof_create(commandHandle, sourceId, did, attributes, cb))
      return proof
    } catch (err) {
      throw new CXSInternalError(`cxs_proof_create -> ${err}`)
    }
  }

  static async deserialize (proofData: IProofData): Promise<Proof> {
    try {
      return await super.deserialize(Proof, JSON.stringify(proofData))
    } catch (err) {
      throw new CXSInternalError(`cxs_proof_deserialize -> ${err}`)
    }
  }

  async serialize (): Promise<IProofData> {
    try {
      const data: IProofData = JSON.parse(await super._serialize())
      return data
    } catch (err) {
      throw new CXSInternalError(`cxs_proof_serialize -> ${err}`)
    }
  }

  updateState () {
    this._state = null
  }

  get proofRequesterDid () {
    return this._proofRequesterDid
  }
}
