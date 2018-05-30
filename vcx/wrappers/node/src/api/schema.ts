
import * as ffi from 'ffi'
import { VCXInternalError } from '../errors'
import { rustAPI } from '../rustlib'
import { createFFICallbackPromise } from '../utils/ffi-helpers'
import { VCXBase } from './VCXBase'

export interface ISchemaCreateData {
  sourceId: string,
  data: ISchemaAttrs,
  paymentHandle: number
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

export interface ISchemaLookupData {
  sourceId: string,
  schemaId: string
}

export class Schema extends VCXBase {
  protected _releaseFn = rustAPI().vcx_schema_release
  protected _serializeFn = rustAPI().vcx_schema_serialize
  protected _deserializeFn = rustAPI().vcx_schema_deserialize
  private _name: string
  private _schemaId: string
  private _schemaAttrs: ISchemaAttrs

  constructor (sourceId: string, { name, schemaId, schemaAttrs }: ISchemaParams) {
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
   * @param {ISchemaCreateData} data
   * @example <caption>Example of ISchema</caption>
   * {sourceId: '123', data: {name: 'name', version: '1.0', attrNames:['name', 'address', 'city']}}
   * @returns {Promise<Schema>} A Schema Object
   */
  static async create ({ paymentHandle, data, sourceId }: ISchemaCreateData): Promise<Schema> {
    const schema = new Schema(sourceId, { name: data.name, schemaId: '', schemaAttrs: data })
    const commandHandle = 0
    try {
      await schema._create((cb) => rustAPI().vcx_schema_create(
      commandHandle,
      schema.sourceId,
      schema._name,
      data.version,
      JSON.stringify(data.attrNames),
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
      return await super._deserialize<Schema, ISchemaParams>(Schema, schema, schemaParams)
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
  static async lookup ({ sourceId, schemaId }: ISchemaLookupData): Promise<Schema> {
    try {
      let rc = null
      const [ schemaDataStr, schemaHandle ] = await createFFICallbackPromise<string>(
          (resolve, reject, cb) => {
            rc = rustAPI().vcx_schema_get_attributes(0, sourceId, schemaId, cb)
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
      const { name, version, data }: ISchemaObj = JSON.parse(schemaDataStr)
      const schemaParams = {
        name,
        schemaAttrs: {
          attrNames: data,
          name,
          version
        },
        schemaId
      }
      const newSchema = new Schema(sourceId, schemaParams)
      newSchema._handle = schemaHandle.toString()
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
          (xcommandHandle, err, schemaIdVal) => {
            if (err) {
              reject(err)
              return
            }
            this._schemaId = schemaIdVal
            resolve(schemaIdVal)
          })
        )
      return schemaId
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), 'vcx_schema_get_schema_id')
    }
  }

  get schemaAttrs (): ISchemaAttrs {
    return this._schemaAttrs
  }
}
