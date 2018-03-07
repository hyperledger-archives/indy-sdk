import { VCXInternalError } from '../errors'
import { rustAPI } from '../rustlib'
import { VCXBase } from './VCXBase'

/**
 * @interface
 * @description
 * SourceId: String for SDK User's reference.
 * name: name of claimdef.
 * schemaNo: Schema Number wanted to create claimdef off of
 * revocation:
 */
export interface IClaimDefinition {
  sourceId: string,
  name: string,
  schemaNo: number,
  revocation: boolean
}

export interface IClaimDefObj {
  source_id: string,
  handle: number
  name: string
  claim_def: IClaimDefData
}

export interface IClaimDefData {
  ref: number,
  origin: string,
  signature_type: string,
  data: any,
}

export interface IClaimDefParams {
  schemaNo: number,
  name: string,
}

/**
 * @class Class representing a Claim Definition
 */
export class ClaimDef extends VCXBase {
  protected _releaseFn = rustAPI().vcx_claimdef_release
  protected _serializeFn = rustAPI().vcx_claimdef_serialize
  protected _deserializeFn = rustAPI().vcx_claimdef_deserialize
  private _name: string
  private _schemaNo: number

  constructor (sourceId, { name, schemaNo }: IClaimDefParams) {
    super(sourceId)
    this._name = name
    this._schemaNo = schemaNo
  }

  /**
   * @memberof ClaimDef
   * @description Builds a generic ClaimDef object
   * @static
   * @async
   * @function create
   * @param {IClaimConfig} config
   * @example <caption>Example of IClaimDefinition</caption>
   * { sourceId: "12", schemaNum: 1, name: "name of claim", revocation: false}
   * @returns {Promise<ClaimDef>} A ClaimDef Object
   */
  static async create (data: IClaimDefinition): Promise<ClaimDef> {
    const claimDef = new ClaimDef(data.sourceId, { name: data.name, schemaNo: data.schemaNo })
    const commandHandle = 0
    const issuerDid = null
    try {
      await claimDef._create((cb) => rustAPI().vcx_claimdef_create(
      commandHandle,
      claimDef.sourceId,
      claimDef._name,
      data.schemaNo,
      issuerDid,
      data.revocation,
      cb
      ))
      return claimDef
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), 'vcx_claimdef_create')
    }
  }

  /**
   * @memberof ClaimDef
   * @description Builds a ClaimDef object with defined attributes.
   * Attributes are often provided by a previous call to the serialize function.
   * @static
   * @async
   * @function deserialize
   * @param {IClaimDefObj} data - contains the information that will be used to build a claimdef object
   * @example <caption>Example of claimData.</caption>
   * { source_id: string, handle: number, name: string }
   * claim_def: { ref: number, origin: string, signature_type: string, data: any}}
   * @returns {Promise<ClaimDef>} A ClaimDef Obj
   */
  static async deserialize (data: IClaimDefObj) {
    try {
      const claimDefParams = {
        name: data.name,
        schemaNo: data.claim_def.ref
      }
      return await super._deserialize(ClaimDef, data, claimDefParams)
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), 'vcx_claimdef_deserialize')
    }
  }

  /**
   * @memberof ClaimDef
   * @description Serializes a ClaimDef object.
   * Data returned can be used to recreate a ClaimDef object by passing it to the deserialize function.
   * @async
   * @function serialize
   * @returns {Promise<IClaimDefObj>} - Jason object with all of the underlying Rust attributes.
   * Same json object structure that is passed to the deserialize function.
   */
  async serialize (): Promise<IClaimDefObj> {
    try {
      const data: IClaimDefObj = JSON.parse(await super._serialize())
      return data
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), 'vcx_claimdef_serialize')
    }
  }
}
