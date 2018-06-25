import { Callback } from 'ffi'
import { VCXInternalError } from '../errors'
import { rustAPI } from '../rustlib'
import { createFFICallbackPromise } from '../utils/ffi-helpers'
import { StateType } from './common'
import { Connection } from './connection'
import { VCXBaseWithState } from './vcx-base-with-state'

export interface IProofCreateData {
  sourceId: string,
  attrs: IProofAttr[],
  name: string,
}

export interface IProofConstructorData {
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
  proof?: string,
  proofState: ProofState,
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
  restrictions?: IFilter[],
  name: string,
}

export interface IFilter {
  schemaId?: string,
  schemaIssuerDid?: string,
  schemaName: string,
  schemaVersion: string,
  issuerDid?: string,
  credDefId?: string,
}

export enum ProofState {
  Undefined = 0,
  Verified = 1,
  Invalid = 2
}

export interface IProofPredicate {
  attr_name: string,
  p_type: string,
  value: number,
  restrictions?: IFilter[],
}

/**
 * @class Class representing a Connection
 */
export class Proof extends VCXBaseWithState<IProofData> {
  /**
   * @memberof Proof
   * @description Builds a generic Proof object
   * @static
   * @async
   * @function create
   * @param {IProofCreateData} data
   * @example <caption>Example of IProofConfig</caption>
   * {sourceId: string,attrs: [{restrictions: [IFilter ...], name: "attrName"}], name: "name of proof"}
   * @returns {Promise<Proof>} A Proof Object
   */
  public static async create ({ sourceId, ...createDataRest }: IProofCreateData): Promise<Proof> {
    try {
      const proof = new Proof(sourceId, createDataRest)
      const commandHandle = 0
      await proof._create((cb) => rustAPI().vcx_proof_create(
        commandHandle,
        proof.sourceId,
        JSON.stringify(createDataRest.attrs),
        JSON.stringify([]),
        createDataRest.name,
        cb
      ))
      return proof
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

  /**
   * @description Builds a Proof object with defined attributes.
   * Attributes are provided by a previous call to the serialize function.
   * @static
   * @async
   * @memberof Proof
   * @function deserialize
   * @param {IProofData} proofData - Data obtained by serialize api. Used to create proof object.
   * @returns {Promise<Proof>} A Proof Object
   */
  public static async deserialize (proofData: IProofData) {
    try {
      const attrs = JSON.parse(proofData.requested_attrs)
      const constructorParams: IProofConstructorData = {
        attrs,
        name: proofData.name
      }
      const proof = await super._deserialize(Proof, proofData, constructorParams)
      return proof
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

  protected _releaseFn = rustAPI().vcx_proof_release
  protected _updateStFn = rustAPI().vcx_proof_update_state
  protected _getStFn = rustAPI().vcx_proof_get_state
  protected _serializeFn = rustAPI().vcx_proof_serialize
  protected _deserializeFn = rustAPI().vcx_proof_deserialize
  private _requestedAttributes: IProofAttr[]
  private _name: string
  private _proofState: ProofState | null = null

  constructor (sourceId: string, { attrs, name }: IProofConstructorData) {
    super(sourceId)
    this._requestedAttributes = attrs
    this._name = name
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
  public async requestProof (connection: Connection): Promise<void> {
    try {
      await createFFICallbackPromise<void>(
          (resolve, reject, cb) => {
            const rc = rustAPI().vcx_proof_send_request(0, this.handle, connection.handle, cb)
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

  /**
   * @memberof Proof
   * @description Returns the requested proof if available
   * @async
   * @function getProof
   * @param {Connection} connection
   * @returns {Promise<IProofResponses>} The proof and the state of the proof (valid | invalid | undefined)
   */
  public async getProof (connection: Connection): Promise<IProofResponses> {
    try {
      const proofRes = await createFFICallbackPromise<{ proofState: ProofState, proofData: string}>(
          (resolve, reject, cb) => {
            const rc = rustAPI().vcx_get_proof(0, this.handle, connection.handle, cb)
            if (rc) {
              reject(rc)
            }
          },
          (resolve, reject) => Callback(
            'void',
            ['uint32', 'uint32', 'uint32', 'string'],
            (xcommandHandle: number, err: number, proofState: ProofState, proofData: string) => {
              if (err) {
                reject(err)
                return
              }
              resolve({ proofState, proofData })
            })
        )
      this._proofState = proofRes.proofState
      return { proof: proofRes.proofData, proofState: proofRes.proofState }
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

  get proofState (): ProofState | null {
    return this._proofState
  }

  get requestedAttributes () {
    return this._requestedAttributes
  }

  get name () {
    return this._name
  }
}
