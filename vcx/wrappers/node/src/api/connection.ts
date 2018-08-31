import * as ffi from 'ffi'
import { VCXInternalError } from '../errors'
import { rustAPI } from '../rustlib'
import { createFFICallbackPromise } from '../utils/ffi-helpers'
import { ISerializedData, StateType } from './common'
import { VCXBaseWithState } from './vcx-base-with-state'

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

export interface IConnectionCreateData {
  id: string
}

export type IConnectionInvite = string

export interface IRecipientInviteInfo extends IConnectionCreateData {
  invite: IConnectionInvite
}

export interface IConnectOptions {
  phone?: string
}

/**
 * @class Class representing a Connection
 */
export class Connection extends VCXBaseWithState<IConnectionData> {
  /**
   * Create a connection object, represents a single endpoint and can be used for sending and receiving
   * credentials and proofs
   *
   * Example:
   * ```
   * source_id = 'foobar123'
   * connection = await Connection.create(source_id)
   * ```
   */
  public static async create ({ id }: IConnectionCreateData): Promise<Connection> {
    try {
      const connection = new Connection(id)
      const commandHandle = 0
      await connection._create((cb) => rustAPI().vcx_connection_create(commandHandle, id, cb))
      return connection
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

  /**
   * Create a connection object with a provided invite, represents a single endpoint and can be used for
   * sending and receiving credentials and proofs.
   * Invite details are provided by the entity offering a connection and generally pulled from a provided QRCode.
   *
   * Example:
   * ```
   * sourceId = 'foobar123'
   * connection_handle = await Connection.createWithInvite({sourceId, inviteDetails})
   * ```
   */
  public static async createWithInvite ({ id, invite }: IRecipientInviteInfo): Promise<Connection> {
    const connection = new Connection(id)
    const commandHandle = 0
    try {
      await connection._create((cb) => rustAPI().vcx_connection_create_with_invite(commandHandle,
                                                 id, invite, cb))

      return connection
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

  /**
   * Create the object from a previously serialized object.
   * Example:
   * data = await connection1.serialize()
   * connection2 = await Connection.deserialize(data)
   */
  public static async deserialize (connectionData: ISerializedData<IConnectionData>) {
    const connection = await super._deserialize(Connection, connectionData)
    return connection
  }

  protected _releaseFn = rustAPI().vcx_connection_release
  protected _updateStFn = rustAPI().vcx_connection_update_state
  protected _getStFn = rustAPI().vcx_connection_get_state
  protected _serializeFn = rustAPI().vcx_connection_serialize
  protected _deserializeFn = rustAPI().vcx_connection_deserialize
  protected _inviteDetailFn = rustAPI().vcx_connection_invite_details

  /**
   * Delete the object from the agency and release any memory associated with it
   *
   * Example:
   * ```
   * def connection = await Connection.create(source_id)
   * await connection.delete()
   * ```
   */
  public async delete (): Promise<void> {
    try {
      await createFFICallbackPromise<void>(
        (resolve, reject, cb) => {
          const rc = rustAPI().vcx_connection_delete_connection(0, this.handle, cb)
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
   * Creates a connection between enterprise and end user.
   *
   * Example:
   * ```
   * phoneNumber = '8019119191'
   * connection = await Connection.create('foobar123')
   * inviteDetails = await connection.connect({phone: phoneNumber})
   * ```
   * @returns {Promise<string}
   */
  public async connect ({ phone }: IConnectOptions = {}): Promise<string> {
    const connectionType: string = phone ? 'SMS' : 'QR'
    const connectionData: string = JSON.stringify({ connection_type: connectionType, phone })
    try {
      return await createFFICallbackPromise<string>(
          (resolve, reject, cb) => {
            const rc = rustAPI().vcx_connection_connect(0, this.handle, connectionData, cb)
            if (rc) {
              reject(rc)
            }
          },
          (resolve, reject) => ffi.Callback(
            'void',
            ['uint32', 'uint32', 'string'],
            (xHandle: number, err: number, details: string) => {
              if (err) {
                reject(err)
                return
              }
              if (!details) {
                reject(`Connection ${this.sourceId} connect returned empty string`)
                return
              }
              resolve(details)
            })
        )
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

  /**
   * Gets the details of the invitation that was returned from the Agent Service.
   *
   * Example:
   * ```
   * phoneNumber = '8019119191'
   * connection = await Connection.create('foobar123')
   * inviteDetails = await connection.connect({phone: phoneNumber})
   * inivteDetailsAgain = await connection.inviteDetails()
   * ```
   */
  public async inviteDetails (abbr: boolean = false): Promise<IConnectionInvite> {
    try {
      const data = await createFFICallbackPromise<string>(
        (resolve, reject, cb) => {
          const rc = this._inviteDetailFn(0, this.handle, abbr, cb)
          if (rc) {
            reject(rc)
          }
        },
        (resolve, reject) => ffi.Callback(
          'void',
          ['uint32', 'uint32', 'string'],
          (handle: number, err: number, details: string) => {
            if (err) {
              reject(err)
              return
            }
            if (!details) {
              reject('no details returned')
              return
            }
            resolve(details)
          })
      )
      return data
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }
}
