
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
  version: string,
  data: string[],
  schema_id: string,
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
  schemaId: string,
  name: string,
  schemaAttrs?: ISchemaAttrs,
}

export class Schema extends VCXBase {
  protected _releaseFn = rustAPI().vcx_schema_release
  protected _serializeFn = rustAPI().vcx_schema_serialize
  protected _deserializeFn = rustAPI().vcx_schema_deserialize
  private _name: string
  private _schemaId: string
  private _schemaAttrs: ISchemaAttrs

  constructor (sourceId, { name, schemaId, schemaAttrs }: ISchemaParams) {
    // Todo: update constructor to take name, schemaId, version, and attrs
    super(sourceId)
    this._name = name
    this._schemaId = schemaId
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
  static async create (data: ISchema, paymentHandle: number): Promise<Schema> {
    const schema = new Schema(data.sourceId, { name: data.data.name, schemaId: '', schemaAttrs: data.data })
    const commandHandle = 0
    try {
      await schema._create((cb) => rustAPI().vcx_schema_create(
      commandHandle,
      schema.sourceId,
      schema._name,
      data.data.version,
      JSON.stringify(data.data.attrNames),
      paymentHandle,
      cb
      ))
      await schema.getSchemaId()
      return schema
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), 'vcx_schema_create')
    }
  }

  /**
   * @memberof Schema
   * @description Builds Schema object with defined attributes.
   * Attributes are provided by a previous call to the serialize function.
   * @static
   * @async
   * @function deserialize
   * @param {ISchemaObj} schema - contains the information that will be used to build a Schema object
   * @returns {Promise<Schema>} A Schema Object
   */
  static async deserialize (schema: ISchemaObj) {
    try {
      const schemaParams = {
        name: schema.name,
        schemaAttrs: {name: schema.name, version: schema.version, attrNames: schema.data},
        schemaId: schema.schema_id
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
   * @param {obj} data - contains sourceId and schema id
   * @returns {Promise<Schema>} - A schema object with the attributes set
   */
  static async lookup (data: { sourceId: string, schemaId: string }): Promise<Schema> {
    try {
      let rc = null
      const schemaData = await createFFICallbackPromise<string>(
          (resolve, reject, cb) => {
            rc = rustAPI().vcx_schema_get_attributes(0, data.sourceId, data.schemaId, cb)
            if (rc) {
              reject(rc)
            }
          },
          (resolve, reject) => ffi.Callback('void', ['uint32', 'uint32', 'uint32', 'string'],
          (handle, err, _schemaHandle, _schemaData) => {
            if (err) {
              reject(err)
              return
            } else if (_schemaData == null) {
              reject('no schema attrs')
            }
            resolve([_schemaData, _schemaHandle])
          })
    )
      const schemaObj: ISchemaObj = JSON.parse(schemaData[0])
      schemaObj.handle = schemaData[1]
      const schemaParams = {
        name: schemaObj.name,
        schemaAttrs: {name: schemaObj.name, version: schemaObj.version, attrNames: schemaObj.data},
        schemaId: data.schemaId
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

  /**
   * @memberof Schema
   * @description Retrieves the schema id associated with the created schema.
   * @async
   * @function getSchemaId
   * @returns {Promise<string>} - Schema's Identifier
   */
  async getSchemaId (): Promise<string> {
    try {
      const schemaId = await createFFICallbackPromise<string>(
          (resolve, reject, cb) => {
            const rc = rustAPI().vcx_schema_get_schema_id(0, this.handle, cb)
            if (rc) {
              reject(rc)
            }
          },
          (resolve, reject) => ffi.Callback('void', ['uint32', 'uint32', 'string'],
          (xcommandHandle, err, _schemaId) => {
            if (err) {
              reject(err)
              return
            }
            this._setScemaId(_schemaId)
            resolve(_schemaId)
          })
        )
      return schemaId
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), 'vcx_schema_get_schema_id')
    }
  }

  getSchemaAttrs (): ISchemaAttrs {
    return this._schemaAttrs
  }

  _setScemaId (schemaId: string) {
    this._schemaId = schemaId
  }

  _convertAttrToSnakeCase (data: ISchemaAttrs): any {
    return _.mapKeys(data, (value, key) => {
      return snakeCase(key)
    })
  }
}
