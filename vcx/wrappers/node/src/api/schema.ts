
import { snakeCase } from 'change-case'
import * as ffi from 'ffi'
import { _ } from 'lodash'
import { VCXInternalError } from '../errors'
import { rustAPI } from '../rustlib'
import { createFFICallbackPromise } from '../utils/ffi-helpers'
import { VCXBase } from './VCXBase'

export interface ISchema {
  sourceId: string,
  data: ISchemaAttrs
}

/**
 * @interface
 * @description
 * name: name of schema
 * version:
 * attrNames: a list of named attribtes inteded to be added to the schema
 */
export interface ISchemaAttrs {
  name: string,
  version: string,
  attrNames: string[]
}

export interface ISchemaObj {
  source_id: string,
  handle: string,
  name: string,
  data: ISchemaTxn,
  sequence_num: number,
}

export interface ISchemaTxn {
  sequence_num?: number,
  sponsor?: string,
  txn_timestamp?: number,
  txn_type?: string,
  data?: {
    name: string,
    version: string,
    attr_names: string[]
  }
}

export interface ISchemaParams {
  schemaNo: number,
  name: string,
  schemaAttrs?: ISchemaAttrs,
}

export class Schema extends VCXBase {
  protected _releaseFn = rustAPI().vcx_schema_release
  protected _serializeFn = rustAPI().vcx_schema_serialize
  protected _deserializeFn = rustAPI().vcx_schema_deserialize
  private _name: string
  private _schemaNo: number
  private _schemaAttrs: ISchemaAttrs

  constructor (sourceId, { name, schemaNo, schemaAttrs }: ISchemaParams) {
    super(sourceId)
    this._name = name
    this._schemaNo = schemaNo
    this._schemaAttrs = schemaAttrs
  }

  /**
   * @memberof Schema
   * @description Builds a generic Schema object
   * @static
   * @async
   * @function create
   * @param {ISchema} data
   * @example <caption>Example of ISchema</caption>
   * {sourceId: '123', data: {name: 'name', version: '1.0', attrNames:['name', 'address', 'city']}}
   * @returns {Promise<Schema>} A Schema Object
   */
  static async create (data: ISchema): Promise<Schema> {
    const schema = new Schema(data.sourceId, { name: data.data.name, schemaNo: 0, schemaAttrs: data.data })
    const commandHandle = 0
    try {
      await schema._create((cb) => rustAPI().vcx_schema_create(
      commandHandle,
      schema.sourceId,
      schema._name,
      JSON.stringify(schema._convertAttrToSnakeCase(data.data)),
      cb
      ))
      await schema.getSchemaNo()
      return schema
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), 'vcx_schema_create')
    }
  }

  /**
   * @memberof Schema
   * @description Builds Schema object with defined attributes.
   * Attributes are often provided by a previous call to the serialize function.
   * @static
   * @async
   * @function deserialize
   * @param {ISchemaObj} schema - contains the information that will be used to build a Schema object
   * @example <caption>Example of schema.</caption>
   * { source_id: string, handle: string, name: string, data: ISchemaTxn, sequence_num: number}
   * @returns {Promise<Schema>} A Schema Object
   */
  static async deserialize (schema: ISchemaObj) {
    const schemaAttrs = schema.data.data
    try {
      const schemaParams = {
        name: schema.name,
        schemaAttrs: {name: schemaAttrs.name, version: schemaAttrs.version, attrNames: schemaAttrs.attr_names},
        schemaNo: schema.sequence_num
      }
      return await super._deserialize(Schema, schema, schemaParams)
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), 'vcx_schema_deserialize')
    }
  }

  /**
   * @memberof Schema
   * @description Looks up the attributes of an already created Schema.
   * @async
   * @function lookup
   * @param {obj} data - contains sourceId and sequence number of schema to look up
   * @returns {Promise<Schema>} - A schema object with the attributes set
   */
  static async lookup (data: { sourceId: string, seqNo: number }): Promise<Schema> {
    try {
      let rc = null
      const schemaData = await createFFICallbackPromise<string>(
          (resolve, reject, cb) => {
            rc = rustAPI().vcx_schema_get_attributes(0, data.sourceId, data.seqNo, cb)
            if (rc) {
              reject(rc)
            }
          },
          (resolve, reject) => ffi.Callback('void', ['uint32', 'uint32', 'string'], (handle, err, _schemaData) => {
            if (err) {
              reject(err)
              return
            } else if (_schemaData == null) {
              reject('no schema attrs')
            }
            resolve(_schemaData)
          })
    )
      const schemaObj: ISchemaObj = JSON.parse(schemaData)
      const schemaAttrs = schemaObj.data.data
      const schemaParams = {
        name: schemaObj.name,
        schemaAttrs: {name: schemaAttrs.name, version: schemaAttrs.version, attrNames: schemaAttrs.attr_names},
        schemaNo: schemaObj.sequence_num
      }
      const newSchema = new Schema(data.sourceId, schemaParams)
      newSchema._handle = JSON.stringify(schemaObj.handle)
      return newSchema
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), 'vcx_schema_get_attributes')
    }
  }

  /**
   * @memberof Schema
   * @description Serializes a Schema object.
   * Data returned can be used to recreate a Schema object by passing it to the deserialize function.
   * @async
   * @function serialize
   * @returns {Promise<ISchemaObj>} - Jason object with all of the underlying Rust attributes.
   * Same json object structure that is passed to the deserialize function.
   */
  async serialize (): Promise<ISchemaObj> {
    try {
      const data: ISchemaObj = JSON.parse(await super._serialize())
      return data
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), 'vcx_schema_serialize')
    }
  }

  async getSchemaNo (): Promise<number> {
    try {
      const schemaNo = await createFFICallbackPromise<number>(
          (resolve, reject, cb) => {
            const rc = rustAPI().vcx_schema_get_sequence_no(0, this.handle, cb)
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
      throw new VCXInternalError(err, VCXBase.errorMessage(err), 'vcx_proof_create')
    }
  }

  getSchemaAttrs (): ISchemaAttrs {
    return this._schemaAttrs
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
