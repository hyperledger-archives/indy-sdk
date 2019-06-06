import * as ffi from 'ffi'
import { VCXInternalError } from '../errors'
import { rustAPI } from '../rustlib'
import { createFFICallbackPromise } from '../utils/ffi-helpers'
import { ISerializedData, StateType } from './common'
import { Connection } from './connection'
import { VCXBaseWithState } from './vcx-base-with-state'

export interface IProofCreateData {
  sourceId: string,
  attrs: IProofAttr[],
  name: string,
  revocationInterval: IRevocationInterval
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

export interface IRevocationInterval {
  from?: number,
  to?: number
}

/**
 * Class representing a Proof
 */
export class Proof extends VCXBaseWithState<IProofData> {
  /**
   * Builds a generic proof object
   *
   * Example:
   * ```
   * data = {
   *   attrs: [
   *     { name: 'attr1' },
   *     { name: 'attr2' }],
   *   name: 'Proof',
   *   sourceId: 'testProofSourceId',
   *   revocationInterval: {from: 1, to: 2}
   * }
   * proof1 = await Proof.create(data)
   * ```
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
        JSON.stringify(createDataRest.revocationInterval),
        createDataRest.name,
        cb
      ))
      return proof
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

/**
 * Builds a Proof object with defined attributes.
 *
 * Attributes are provided by a previous call to the serialize function.
 * ```
 * data = {
 *   attrs: [
 *     { name: 'attr1' },
 *     { name: 'attr2' }],
 *   name: 'Proof',
 *   sourceId: 'testProofSourceId'
 * }
 * proof1 = await Proof.create(data)
 * data1 = await Proof.serialize()
 * await Proof.deserialize(data1)
 * ```
 */
  public static async deserialize (proofData: ISerializedData<IProofData>) {
    try {
      const { data: { requested_attrs, name } } = proofData
      const attrs = JSON.parse(requested_attrs)
      const constructorParams: IProofConstructorData = {
        attrs,
        name
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
   *
   * Updates the state of the proof from the given message.
   *
   * Example:
   * ```
   * await object.updateStateWithMessage(message)
   * ```
   * @returns {Promise<void>}
   */
  public async updateStateWithMessage (message: string): Promise<void> {
    try {
      const commandHandle = 0
      await createFFICallbackPromise<number>(
        (resolve, reject, cb) => {
          const rc = rustAPI().vcx_proof_update_state_with_message(commandHandle, this.handle, message, cb)
          if (rc) {
            resolve(StateType.None)
          }
        },
        (resolve, reject) => ffi.Callback(
          'void',
          ['uint32', 'uint32', 'uint32'],
          (handle: number, err: any, state: StateType) => {
            if (err) {
              reject(err)
            }
            resolve(state)
          })
      )
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

  /**
   * Sends a proof request to pairwise connection.
   *
   * Example
   * ```
   * data = {
   *   attrs: [
   *     { name: 'attr1' },
   *     { name: 'attr2' }],
   *   name: 'Proof',
   *   sourceId: 'testProofSourceId'
   * }
   * proof = await Proof.create(data)
   * await proof.requestProof(connection)
   * ```
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
          (resolve, reject) => ffi.Callback(
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
   * Returns the requested proof if available
   *
   * Example
   * ```
   * data = {
   *   attrs: [
   *     { name: 'attr1' },
   *     { name: 'attr2' }],
   *   name: 'Proof',
   *   sourceId: 'testProofSourceId'
   * }
   * proof = await Proof.create(data)
   * await proof.requestProof(connection)
   * proofData = await proof.getProof(connection)
   * assert.equal(proofData.proofState, ProofState.Verified)
   * ```
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
          (resolve, reject) => ffi.Callback(
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
  /**
   * Get the state of the proof
   *
   * Example
   * ```
   * data = {
   *   attrs: [
   *     { name: 'attr1' },
   *     { name: 'attr2' }],
   *   name: 'Proof',
   *   sourceId: 'testProofSourceId'
   * }
   * proof = await Proof.create(data)
   * await proof.requestProof(connection)
   * assert.equal(await proof.getState(), StateType.OfferSent)
   * ```
   */
  get proofState (): ProofState | null {
    return this._proofState
  }
  /**
   * Get the attributes of the proof
   *
   * Example
   * ```
   * data = {
   *   attrs: [
   *     { name: 'attr1' },
   *     { name: 'attr2' }],
   *   name: 'Proof',
   *   sourceId: 'testProofSourceId'
   * }
   * proof = await Proof.create(data)
   * await proof.requestProof(connection)
   * assert.equal(await proof.getState(), StateType.OfferSent)
   * proofData = await proof.getProof(connection)
   * await proof.updateState()
   * assert.equal(await proof.requestedAttributes(), data.attrs)
   * ```
   */
  get requestedAttributes () {
    return this._requestedAttributes
  }

  /**
   * Get the name of the proof
   *
   * Example
   * ```
   * data = {
   *   attrs: [
   *     { name: 'attr1' },
   *     { name: 'attr2' }],
   *   name: 'Proof',
   *   sourceId: 'testProofSourceId'
   * }
   * proof = await Proof.create(data)
   * await proof.requestProof(connection)
   * assert.equal(await proof.getState(), StateType.OfferSent)
   * proofData = await proof.getProof(connection)
   * await proof.updateState()
   * assert.equal(await proof.name(), data.name)
   * ```
   */
  get name () {
    return this._name
  }
}
