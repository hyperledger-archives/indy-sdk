import * as ffi from 'ffi'
import { VCXInternalError } from '../errors'
import { rustAPI } from '../rustlib'
import { createFFICallbackPromise } from '../utils/ffi-helpers'
import { ISerializedData, StateType } from './common'
import { Connection } from './connection'
import { VCXBaseWithState } from './vcx-base-with-state'

/**
 *    The object of the VCX API representing a Verifier side in the credential presentation process.
 *    Assumes that pairwise connection between Verifier and Prover is already established.
 *
 *    # State
 *
 *    The set of object states and transitions depends on communication method is used.
 *    The communication method can be specified as config option on one of *_init function. The default communication method us `proprietary`.
 *
 *    proprietary:
 *        VcxStateType::VcxStateInitialized - once `vcx_proof_create` (create Proof object) is called.
 *
 *        VcxStateType::VcxStateOfferSent - once `vcx_credential_send_request` (send `PROOF_REQ` message) is called.
 *
 *        VcxStateType::VcxStateAccepted - once `PROOF` messages is received.
 *                                         use `vcx_proof_update_state` or `vcx_proof_update_state_with_message` functions for state updates.
 *
 *    aries:
 *        VcxStateType::VcxStateInitialized - once `vcx_proof_create` (create Proof object) is called.
 *
 *        VcxStateType::VcxStateOfferSent - once `vcx_credential_send_request` (send `PresentationRequest` message) is called.
 *
 *        VcxStateType::VcxStateAccepted - once `Presentation` messages is received.
 *        VcxStateType::None - once `ProblemReport` messages is received.
 *        VcxStateType::None - once `PresentationProposal` messages is received.
 *        VcxStateType::None - on `Presentation` validation failed.
 *                                                use `vcx_proof_update_state` or `vcx_proof_update_state_with_message` functions for state updates.
 *
 *    # Transitions
 *
 *    proprietary:
 *        VcxStateType::None - `vcx_proof_create` - VcxStateType::VcxStateInitialized
 *
 *        VcxStateType::VcxStateInitialized - `vcx_credential_send_request` - VcxStateType::VcxStateOfferSent
 *
 *        VcxStateType::VcxStateOfferSent - received `PROOF` - VcxStateType::VcxStateAccepted
 *
 *    aries: RFC - https://github.com/hyperledger/aries-rfcs/tree/7b6b93acbaf9611d3c892c4bada142fe2613de6e/features/0037-present-proof#propose-presentation
 *        VcxStateType::None - `vcx_proof_create` - VcxStateType::VcxStateInitialized
 *
 *        VcxStateType::VcxStateInitialized - `vcx_credential_send_request` - VcxStateType::VcxStateOfferSent
 *
 *        VcxStateType::VcxStateOfferSent - received `Presentation` - VcxStateType::VcxStateAccepted
 *        VcxStateType::VcxStateOfferSent - received `PresentationProposal` - VcxStateType::None
 *        VcxStateType::VcxStateOfferSent - received `ProblemReport` - VcxStateType::None
 *
 *    # Messages
 *
 *    proprietary:
 *        ProofRequest (`PROOF_REQ`)
 *        Proof (`PROOF`)
 *
 *    aries:
 *        PresentationRequest - https://github.com/hyperledger/aries-rfcs/tree/7b6b93acbaf9611d3c892c4bada142fe2613de6e/features/0037-present-proof#request-presentation
 *        Presentation - https://github.com/hyperledger/aries-rfcs/tree/7b6b93acbaf9611d3c892c4bada142fe2613de6e/features/0037-present-proof#presentation
 *        PresentationProposal - https://github.com/hyperledger/aries-rfcs/tree/7b6b93acbaf9611d3c892c4bada142fe2613de6e/features/0037-present-proof#propose-presentation
 *        Ack - https://github.com/hyperledger/aries-rfcs/tree/master/features/0015-acks#explicit-acks
 */

/**
 * @description Interface that represents the parameters for `Proof.create` function.
 * @interface
 */
export interface IProofCreateData {
  // Enterprise's personal identification for the user.
  sourceId: string,
  //  Describes requested attribute
  attrs: IProofAttr[],
  // Describres requested predicates
  preds: IProofPredicate[],
  // Name of the proof request
  name: string,
  // Revocation interval
  revocationInterval: IRevocationInterval
}

export interface IProofConstructorData {
  //  Describes requested attribute
  attrs: IProofAttr[],
  // Name of the proof request
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
  // Proof json
  proof?: string,
  // Proof status
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
  // Requested attribute restrictions
  restrictions?: IFilter[],
  // Requested attribute name
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
   *     { name: 'attr2' },
   *     { names: ['attr3', 'attr4'] }],
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
        JSON.stringify(createDataRest.preds),
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
  protected _updateStWithMessageFn = rustAPI(). vcx_proof_update_state_with_message
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
   * Generates the proof request message for sending.
   *
   * Example:
   * ```
   * data = {
   *   attrs: [
   *     { name: 'attr1' },
   *     { name: 'attr2' }],
   *   name: 'Proof',
   *   sourceId: 'testProofSourceId'
   * }
   * proof = await Proof.create(data)
   * await proof.getProofRequestMessage()
   * ```
   */
  public async getProofRequestMessage (): Promise<string> {
    try {
      return await createFFICallbackPromise<string>(
          (resolve, reject, cb) => {
            const rc = rustAPI().vcx_proof_get_request_msg(0, this.handle, cb)
            if (rc) {
              reject(rc)
            }
          },
          (resolve, reject) => ffi.Callback(
            'void',
            ['uint32', 'uint32', 'string'],
            (xHandle: number, err: number, message: string) => {
              if (err) {
                reject(err)
                return
              }
              if (!message) {
                reject(`proof ${this.sourceId} returned empty string`)
                return
              }
              resolve(message)
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
