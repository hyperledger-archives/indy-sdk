import { snakeCase } from 'change-case'
import { Callback } from 'ffi'
import { _ } from 'lodash'
import { VCXInternalError } from '../errors'
import { rustAPI } from '../rustlib'
import { createFFICallbackPromise } from '../utils/ffi-helpers'
import { StateType } from './common'
import { Connection } from './connection'
import { VCXBase } from './VCXBase'
import { VCXBaseWithState } from './VCXBaseWithState'

export interface IProofConfig {
  sourceId: string,
  attrs: IProofAttr[],
  name: string,
}

/**
 * @description Interface that represents the attributes of a Proof object.
 * This interface is expected as the type for deserialize's parameter and serialize's return value
 * @interface
 */
export interface IProofData {
  source_id: string
  handle: number
  requested_attrs: string
  prover_did: string
  state: StateType
  name: string
  proof_state: ProofState
  proof: any
}

export interface IProofResponses {
  proofAttrs: IProofResponseAttr[],
  proofState: ProofState,
}

export interface IProofResponseAttr {
  schema_seq_no?: number,
  issuer_did?: string,
  claim_uuid?: string,
  attr_info: IClaimInfo
}

export interface IClaimInfo {
  name: string,
  value: string,
  type: ProofFieldType,
  predicate_type?: PredicateTypes
}

export enum ProofFieldType {
  Revealed = 'revealed',
  Unrevealed = 'unrevealed',
  SelfAttested = 'self_attested',
  Predicate = 'predicate'
}

export enum PredicateTypes {
  GE = 'GE',
  LE = 'LE',
  EQ = 'EQ'
}

/**
 * @description This represents one attribute expected for user to prove.
 * A list of these attributes will be composed of all requirements for user to prove.
 * @interface
 */
export interface IProofAttr {
  issuerDid?: string,
  schemaSeqNo?: number,
  name: string,
}

export enum ProofState {
  Undefined = 0,
  Verified = 1,
  Invalid = 2
}

// export interface IProofPredicate {
//   attr_name: string,
//   p_type: string,
//   value: number,
//   schema_seq_no: number,
//   issuer_did: string,
// }

/**
 * @class Class representing a Connection
 */
export class Proof extends VCXBaseWithState {
  protected _releaseFn = rustAPI().vcx_proof_release
  protected _updateStFn = rustAPI().vcx_proof_update_state
  protected _getStFn = rustAPI().vcx_proof_get_state
  protected _serializeFn = rustAPI().vcx_proof_serialize
  protected _deserializeFn = rustAPI().vcx_proof_deserialize
  private _requestedAttributes: IProofAttr[]
  private _name: string
  private _proofState: number

  /**
   * @memberof Proof
   * @description Builds a generic Proof object
   * @static
   * @async
   * @function create
   * @param {IProofConfig} data
   * @example <caption>Example of IProofConfig</caption>
   * {sourceId: string,attrs: [{issuerDid: "123",schemaSeqNo: 1, name: "name of attr expected"}], name: "name of proof"}
   * @returns {Promise<Proof>} A Proof Object
   */
  static async create (data: IProofConfig): Promise<Proof> {
    const proof = new Proof(data.sourceId)
    proof._requestedAttributes = data.attrs
    proof._name = data.name
    const commandHandle = 0

    try {
      await proof._create((cb) => rustAPI().vcx_proof_create(
        commandHandle,
        proof.sourceId,
        JSON.stringify(proof._convertAttrToSnakeCase(data.attrs)),
        JSON.stringify([]),
        proof._name,
        cb
      ))
      return proof
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), 'vcx_proof_create')
    }
  }

  /**
   * @description Builds a Proof object with defined attributes.
   * Attributes are often provided by a previous call to the serialize function.
   * @static
   * @async
   * @memberof Proof
   * @function deserialize
   * @param {IProofData} proofData - contains the information that will be used to build a proof object
   * @example <caption>Example of proofData.</caption>
   * proofData = {source_id:"12",handle:1,"requested_attrs":[{issuerDid:"did",schemaSeqNo:1,name:"test"}],
   * requested_predicates:[],msg_uid:"",requester_did:"",prover_did:"",state:1,tid:0,mid:0,name:"Proof of Address"}
   * @returns {Promise<Connection>} A Connection Object
   */
  static async deserialize (proofData: IProofData) {
    try {
      const proof = await super._deserialize(Proof, proofData)
      return proof
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), 'vcx_proof_deserialize')
    }
  }

  /**
   * @description Serializes a proof object.
   * Data returned can be used to recreate a Proof object by passing it to the deserialize function.
   * @async
   * @memberof Proof
   * @function serialize
   * @returns {Promise<IProofData>} - Jason object with all of the underlying Rust attributes.
   * Same json object structure that is passed to the deserialize function.
   */
  async serialize (): Promise<IProofData> {
    try {
      const data: IProofData = JSON.parse(await super._serialize())
      return data
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), 'vcx_proof_serialize')
    }
  }

  /**
   * @description Gets the state of the proof.
   * @async
   * @memberof Proof
   * @function getState
   * @returns {Promise<number>}
   */
  async getState (): Promise<number> {
    try {
      return await this._getState()
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), 'vcx_proof_get_state')
    }
  }

  /**
   * @description Communicates with the agent service for polling and setting the state of the Proof.
   * @async
   * @memberof Proof
   * @function updateState
   * @returns {Promise<void>}
   */
  async updateState (): Promise<void> {
    try {
      await this._updateState()
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), 'vcx_proof_updateState')
    }
  }

  /**
   * @memberof Proof
   * @description Sends a proof request to the end user.
   * Proof request is made up of the data provided in the creation of this object
   * @async
   * @param {Connection} connection
   * Connection is the object that was created to set up the pairwise relationship.
   * @function requestProof
   * @returns {Promise<void>}
   */
  async requestProof (connection: Connection): Promise<void> {
    try {
      await createFFICallbackPromise<void>(
          (resolve, reject, cb) => {
            const rc = rustAPI().vcx_proof_send_request(0, this.handle, connection.handle, cb)
            if (rc) {
              reject(rc)
            }
          },
          (resolve, reject) => Callback('void', ['uint32', 'uint32'], (xcommandHandle, err) => {
            if (err) {
              reject(err)
              return
            }
            resolve(xcommandHandle)
          })
        )
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), 'vcx_proof_send_request')
    }
  }

  async getProof (connection: Connection): Promise<IProofResponses> {
    try {
      const proof = await createFFICallbackPromise<string>(
          (resolve, reject, cb) => {
            const rc = rustAPI().vcx_get_proof(0, this.handle, connection.handle, cb)
            if (rc) {
              reject(rc)
            }
          },
          (resolve, reject) => Callback('void', ['uint32', 'uint32', 'uint32', 'string'],
          (xcommandHandle, err, proofState, proofData) => {
            if (err) {
              reject(err)
              return
            }
            this._setProofState(proofState)
            resolve(proofData)
          })
        )
      return {proofAttrs: JSON.parse(proof), proofState: this.getProofState()}
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), 'vcx_get_proof')
    }
  }

  getProofState (): number {
    return this._proofState
  }

  _setProofState (state: number) {
    this._proofState = state
  }

  _convertAttrToSnakeCase (data: IProofAttr[]): any[] {
    const requestedAttrs = []
    data.forEach((x) => {
      requestedAttrs.push(_.mapKeys(x, (value, key) => {
        return snakeCase(key)
      }))
    })
    return requestedAttrs
  }
}
