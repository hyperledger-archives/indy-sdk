import * as ffi from 'ffi'
import { VCXInternalError } from '../errors'
import { rustAPI } from '../rustlib'
import { createFFICallbackPromise } from '../utils/ffi-helpers'
import { StateType } from './common'
import { VCXBase } from './VCXBase'
import { VCXBaseWithState } from './VCXBaseWithState'

/**
 * @description Interface that represents the attributes of a Connection object.
 * This data is expected as the type for deserialize's parameter and serialize's return value
 * @interface
 */
export interface IConnectionData {
  source_id: string
  invite_detail: string,
  handle: number,
  pw_did: string,
  pw_verkey: string,
  did_endpoint: string,
  endpoint: string,
  uuid: string,
  wallet: string,
  state: StateType
}

export interface IRecipientInfo {
  id: string
}

export interface IConnectOptions {
  phone?: string,
  timeout?: number
}

/**
 * @class Class representing a Connection
 */
export class Connection extends VCXBaseWithState {
  protected _releaseFn = rustAPI().vcx_connection_release
  protected _updateStFn = rustAPI().vcx_connection_update_state
  protected _getStFn = rustAPI().vcx_connection_get_state
  protected _serializeFn = rustAPI().vcx_connection_serialize
  protected _deserializeFn = rustAPI().vcx_connection_deserialize
  protected _inviteDetailFn = rustAPI().vcx_connection_invite_details

  /**
   * @memberof Connection
   * @description Builds a generic Connection object.
   * @static
   * @async
   * @function create
   * @param {IRecipientInfo} recipientInfo
   * @example <caption>Example of recipientInfo</caption>
   * {id: "123"}
   * @returns {Promise<Connection>} A Connection Object
   */
  static async create ( recipientInfo: IRecipientInfo): Promise<Connection> {
    const connection = new Connection(recipientInfo.id)
    const commandHandle = 0
    try {
      await connection._create((cb) => rustAPI().vcx_connection_create(commandHandle, recipientInfo.id, cb))
      return connection
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), 'vcx_connection_create')
    }
  }

  /**
   * @memberof Connection
   * @description Builds a Connection object with defined attributes.
   * The attributes are often provided by a previous call to the serialize function
   * @static
   * @async
   * @function deserialize
   * @param {IConnectionData} connectionData - contains the information that will be used to build a connection object
   * @example <caption>Example of Connection Data </caption>
   * {source_id:"234",handle:560373036,pw_did:"did",pw_verkey:"verkey",did_endpoint:"",state:2,uuid:"",endpoint:"",
   * invite_detail:{e:"",rid:"",sakdp:"",sn:"",sD:"",lu:"",sVk:"",tn:""}}
   * @returns {Promise<Connection>} A Connection Object
   */
  static async deserialize (connectionData: IConnectionData) {
    try {
      const connection = await super._deserialize(Connection, connectionData)
      return connection
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), 'vcx_connection_deserialize')
    }
  }

  /**
   * @memberof Connection
   * @description Creates a connection between enterprise and end user.
   * @async
   * @function connect
   * @param {IConnectOptions} options - data determining if connection is established by SMS or QR code. Default is SMS
   * @example <caption>Example of IConnectionOptions</caption>
   * { phone: "800", timeout: 30 }
   * @returns {Promise<void>}
   */
  async connect ( options: IConnectOptions = {} ): Promise<string> {
    const phone = options.phone
    const connectionType: string = phone ? 'SMS' : 'QR'
    const connectionData: string = JSON.stringify({connection_type: connectionType, phone})
    try {
      return await createFFICallbackPromise<string>(
          (resolve, reject, cb) => {
            const rc = rustAPI().vcx_connection_connect(0, this._handle, connectionData, cb)
            if (rc) {
              resolve(rc)
            }
          },
          (resolve, reject) => ffi.Callback('void', ['uint32', 'uint32', 'string'], (xHandle, err, details) => {
            if (err) {
              reject(err)
              return
            } else if (details == null) {
              reject('no details returned')
            }
            resolve(details)
          })
        )
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), 'vcx_connection_connect')
    }
  }

  /**
   * @memberof Connection
   * @description Serializes a connection object.
   * Data returned can be used to recreate a Connection object by passing it to the deserialize function.
   * @async
   * @function serialize
   * @returns {Promise<IConnectionData>} - Json object with all of the underlying Rust attributes.
   * Same json object structure that is passed to the deserialize function.
   */
  async serialize (): Promise<IConnectionData> {
    try {
      const data: IConnectionData = JSON.parse(await super._serialize())
      return data
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), 'vcx_connection_serialize')
    }
  }

  /**
   * @memberof Connection
   * @description Communicates with the agent service for polling and setting the state of the Connection.
   * @async
   * @function updateState
   * @returns {Promise<void>}
   */
  async updateState (): Promise<void> {
    try {
      await this._updateState()
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), 'vcx_connection_updateState')
    }
  }

  /**
   * @memberof Connection
   * @description Gets the state of the connection.
   * @async
   * @function getState
   * @returns {Promise<number>}
   */
  async getState (): Promise<number> {
    try {
      return await this._getState()
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), 'vcx_connection_get_state')
    }
  }

  /**
   * @memberof Connection
   * @description
   * Gets the details of the invitation that was returned from the Agent Service.
   * @async
   * @function inviteDetails
   * @returns {Promise<string>} - String with the details
   */
  async inviteDetails (abbr: boolean = false): Promise<string> {
    try {
      const data: string = await this._inviteDetails(abbr)
      return data
    } catch (err) {
      throw new VCXInternalError(err, VCXBase.errorMessage(err), 'vcx_connection_invite_details')
    }
  }

  protected async _inviteDetails (abbr: boolean = false): Promise<string> {
    const connHandle = this._handle
    let rc = null
    const data = await createFFICallbackPromise<string>(
        (resolve, reject, cb) => {
          rc = this._inviteDetailFn(0, connHandle, abbr, cb)
          if (rc) {
            reject(rc)
          }
        },
        (resolve, reject) => ffi.Callback('void', ['uint32', 'uint32', 'string'], (handle, err, details) => {
          if (err) {
            reject(err)
            return
          } else if (details == null) {
            reject('no details returned')
          }
          resolve(details)
        })
    )
    return data
  }
}
