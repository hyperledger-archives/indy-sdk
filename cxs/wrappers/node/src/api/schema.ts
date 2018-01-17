
import { snakeCase } from 'change-case'
import * as ffi from 'ffi'
import { _ } from 'lodash'
import { CXSInternalError } from '../errors'
import { rustAPI } from '../rustlib'
import { createFFICallbackPromise } from '../utils/ffi-helpers'
import { CXSBase } from './CXSBase'

export interface ISchema {
  sourceId: string,
  name: string,
  data: ISchemaAttrs
}

export interface ISchemaAttrs {
  name: string,
  version: string,
  attrNames: [string]
}

export interface ISchemaObj {
  source_id: string,
  handle: number,
  name: string,
  data: any,
  sequence_num: number,
}

export interface ISchemaParams {
  schemaNo: number,
  name: string,
}

export class Schema extends CXSBase {
  protected _releaseFn = rustAPI().cxs_schema_release
  protected _serializeFn = rustAPI().cxs_schema_serialize
  protected _deserializeFn = rustAPI().cxs_schema_deserialize
  private _name: string
  private _schemaNo: number

  constructor (sourceId, { name, schemaNo }: ISchemaParams) {
    super(sourceId)
    this._name = name
    this._schemaNo = schemaNo
  }

  static async create (data: ISchema): Promise<Schema> {
    const schema = new Schema(data.sourceId, { name: data.name, schemaNo: 0 })
    const commandHandle = 0
    try {
      await schema._create((cb) => rustAPI().cxs_schema_create(
      commandHandle,
      schema.sourceId,
      schema._name,
      JSON.stringify(schema._convertAttrToSnakeCase(data.data)),
      cb
      ))
      await schema.getSchemaNo()
      return schema
    } catch (err) {
      throw new CXSInternalError(`cxs_schema_create -> ${err}`)
    }
  }

  static async deserialize (data: ISchemaObj) {
    try {
      const schemaParams = {
        name: data.name,
        schemaNo: data.sequence_num
      }
      return await super._deserialize(Schema, data, schemaParams)
    } catch (err) {
      throw new CXSInternalError(`cxs_schema_deserialize -> ${err}`)
    }
  }

  async serialize (): Promise<ISchemaObj> {
    try {
      const data: ISchemaObj = JSON.parse(await super._serialize())
      return data
    } catch (err) {
      throw new CXSInternalError(`cxs_schema_serialize -> ${err}`)
    }
  }

  async getSchemaNo (): Promise<number> {
    try {
      const schemaNo = await createFFICallbackPromise<number>(
          (resolve, reject, cb) => {
            const rc = rustAPI().cxs_schema_get_sequence_no(0, this.handle, cb)
            if (rc) {
              reject(rc)
            }
          },
          (resolve, reject) => ffi.Callback('void', ['uint32', 'uint32', 'uint32'],
          (xcommandHandle, err, _schemaNo) => {
            if (err) {
              reject(err)
              return
            }
            this._setSchemaNo(_schemaNo)
            resolve(_schemaNo)
          })
        )
      return schemaNo
    } catch (err) {
      throw new CXSInternalError(`cxs_schema_get_sequence_no -> ${err}`)
    }
  }

  _setSchemaNo (schemaNo: number) {
    this._schemaNo = schemaNo
  }

  _convertAttrToSnakeCase (data: ISchemaAttrs): any {
    return _.mapKeys(data, (value, key) => {
      return snakeCase(key)
    })
  }
}
