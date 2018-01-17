import { CXSInternalError } from '../errors'
import { rustAPI } from '../rustlib'
import { CXSBase } from './CXSBase'

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

export class ClaimDef extends CXSBase {
  protected _releaseFn = rustAPI().cxs_claimdef_release
  protected _serializeFn = rustAPI().cxs_claimdef_serialize
  protected _deserializeFn = rustAPI().cxs_claimdef_deserialize
  private _name: string
  private _schemaNo: number

  constructor (sourceId, { name, schemaNo }: IClaimDefParams) {
    super(sourceId)
    this._name = name
    this._schemaNo = schemaNo
  }

  static async create (data: IClaimDefinition): Promise<ClaimDef> {
    const claimDef = new ClaimDef(data.sourceId, { name: data.name, schemaNo: data.schemaNo })
    const commandHandle = 0
    try {
      await claimDef._create((cb) => rustAPI().cxs_claimdef_create(
      commandHandle,
      claimDef.sourceId,
      claimDef._name,
      data.schemaNo,
      data.revocation,
      cb
      ))
      return claimDef
    } catch (err) {
      throw new CXSInternalError(`cxs_claimdef_create -> ${err}`)
    }
  }

  static async deserialize (data: IClaimDefObj) {
    try {
      const claimDefParams = {
        name: data.name,
        schemaNo: data.claim_def.ref
      }
      return await super._deserialize(ClaimDef, data, claimDefParams)
    } catch (err) {
      throw new CXSInternalError(`cxs_claimdef_deserialize -> ${err}`)
    }
  }

  async serialize (): Promise<IClaimDefObj> {
    try {
      const data: IClaimDefObj = JSON.parse(await super._serialize())
      return data
    } catch (err) {
      throw new CXSInternalError(`cxs_claimdef_serialize -> ${err}`)
    }
  }
}
