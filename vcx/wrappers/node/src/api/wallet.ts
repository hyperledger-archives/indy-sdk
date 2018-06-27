import { Callback } from 'ffi'

import { VCXInternalError } from '../errors'
import { rustAPI } from '../rustlib'
import { createFFICallbackPromise } from '../utils/ffi-helpers'
import { IUTXO } from './common'

export type PaymentAddress = string
export type PaymentAmount = number
export type PaymentHandle = number
/**
 * @interface An interface representing a record that can be added to the wallet
 */
export interface IRecord {
  type_: string,
  id: string,
  value: any,
  tags: any,
}

export interface IRecordUpdate {
  type_: string,
  id: string,
  value: any
}

export interface ISendTokens {
  payment: PaymentHandle,
  tokens: PaymentAmount,
  recipient: PaymentAddress
}

export interface IDeleteRecordTagsOptions {
  tagList: string[]
}

export interface IDeleteRecordData {
  type: string,
  id: string
}

export type IGerRecordData = IDeleteRecordData

export interface IOpenSearchData {
  type: string,
  queryJson: string,
  options: string
}

export interface ISearchNextRecordsOptions {
  count: number
}

export interface IPaymentAddress {
  address: string,
  balance: number,
  utxo: IUTXO[]
}

export interface IWalletTokenInfo {
  balance: number,
  addresses: IPaymentAddress[]
}

/**
 * @class Class representing a Wallet
 */
export class Wallet {

  /**
   * @memberof Wallet
   * @description Gets wallet token info
   * @static
   * @async
   * @param {paymentAddress} address
   * @returns {Promise<string>} Wallet info, balance, addresses, etc
   */
  public static async getTokenInfo (handle?: PaymentHandle): Promise<IWalletTokenInfo> {
    try {
      const walletInfoStr = await createFFICallbackPromise<string>(
        (resolve, reject, cb) => {
          const rc = rustAPI().vcx_wallet_get_token_info(0, handle, cb)
          if (rc) {
            reject(rc)
          }
        },
        (resolve, reject) => Callback(
          'void',
          ['uint32','uint32','string'],
          (xhandle: number, err: number, info: string) => {
            if (err) {
              reject(err)
              return
            }
            resolve(info)
          })
      )
      const walletInfo = JSON.parse(walletInfoStr)
      return walletInfo
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

  /**
   * @memberof Wallet
   * @description Creates payment address inside wallet
   * @static
   * @async
   * @param
   * @returns {Promise<string>} New address
   */
  public static async createPaymentAddress (): Promise<string> {
    try {
      return await createFFICallbackPromise<string>(
        (resolve, reject, cb) => {
          const rc = rustAPI().vcx_wallet_create_payment_address(0, cb)
          if (rc) {
            reject(rc)
          }
        },
        (resolve, reject) => Callback(
          'void',
          ['uint32','uint32','string'],
          (xhandle: number, err: number, info: string) => {
            if (err) {
              reject(err)
              return
            }
            resolve(info)
          })
      )
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

  /**
   * @memberof Wallet
   * @description Sends token to a specified address
   * @static
   * @async
   * @param {ISendTokens} sendTokensData
   * @returns {Promise<string>} The receipt
   */
  public static async sendTokens ({ payment, tokens, recipient }: ISendTokens): Promise<string> {
    try {
      return await createFFICallbackPromise<string>(
        (resolve, reject, cb) => {
          const rc = rustAPI().vcx_wallet_send_tokens(0, payment, tokens, recipient, cb)
          if (rc) {
            reject(rc)
          }
        },
        (resolve, reject) => Callback(
          'void',
          ['uint32','uint32','string'],
          (xhandle: number, err: number, receipt: string) => {
            if (err) {
              reject(err)
              return
            }
            resolve(receipt)
          })
      )
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

  /**
   * @memberof Wallet
   * @description Adds a record to the wallet for storage
   * @static
   * @async
   * @param {Record} record
   * @returns {Promise<void>}
   */
  public static async addRecord (record: IRecord): Promise<void> {
    const commandHandle = 0
    try {
      await createFFICallbackPromise<string>(
        (resolve, reject, cb) => {
          const rc = rustAPI().vcx_wallet_add_record(commandHandle,
            record.type_,
            record.id, record.value,
            JSON.stringify(record.tags),
            cb)
          if (rc) {
            reject(rc)
          }
        },
        (resolve, reject) => Callback(
          'void',
          ['uint32','uint32','string'],
          (xhandle: number, err: number, receipt: string) => {
            if (err) {
              reject(err)
              return
            }
            resolve(receipt)
          })
      )
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

  /**
   * @memberof Wallet
   * @description Updates a record already in the wallet
   * @static
   * @async
   * @param {Record} record
   * @returns {Promise<void>}
   */
  public static async updateRecordValue (record: IRecordUpdate): Promise<void> {
    const commandHandle = 0
    try {
      await createFFICallbackPromise<string>(
        (resolve, reject, cb) => {
          const rc = rustAPI().vcx_wallet_update_record_value(commandHandle,
            record.type_,
            record.id, record.value,
            cb)
          if (rc) {
            reject(rc)
          }
        },
        (resolve, reject) => Callback(
          'void',
          ['uint32','uint32','string'],
          (xhandle: number, err: number, receipt: string) => {
            if (err) {
              reject(err)
              return
            }
            resolve(receipt)
          })
      )
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

  /**
   * @memberof Wallet
   * @description Updates a record's tags already in the wallet
   * @static
   * @async
   * @param {Record} record
   * @returns {Promise<void>}
   */
  public static async updateRecordTags (record: IRecord): Promise<void> {
    const commandHandle = 0
    try {
      await createFFICallbackPromise<string>(
        (resolve, reject, cb) => {
          const rc = rustAPI().vcx_wallet_update_record_tags(commandHandle,
            record.type_,
            record.id,
            JSON.stringify(record.tags),
            cb)
          if (rc) {
            reject(rc)
          }
        },
        (resolve, reject) => Callback(
          'void',
          ['uint32','uint32','string'],
          (xhandle: number, err: number, receipt: string) => {
            if (err) {
              reject(err)
              return
            }
            resolve(receipt)
          })
      )
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

  /**
   * @memberof Wallet
   * @description Adds tags to a record already in the wallet
   * @static
   * @async
   * @param {Record} record
   * @returns {Promise<void>}
   */
  public static async addRecordTags (record: IRecord): Promise<void> {
    const commandHandle = 0
    try {
      await createFFICallbackPromise<string>(
        (resolve, reject, cb) => {
          const rc = rustAPI().vcx_wallet_add_record_tags(commandHandle,
            record.type_,
            record.id,
            JSON.stringify(record.tags),
            cb)
          if (rc) {
            reject(rc)
          }
        },
        (resolve, reject) => Callback(
          'void',
          ['uint32','uint32','string'],
          (xhandle: number, err: number, receipt: string) => {
            if (err) {
              reject(err)
              return
            }
            resolve(receipt)
          })
      )
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

  /**
   * @memberof Wallet
   * @description Tags to delete from a record already in the wallet
   * @static
   * @async
   * @param {Record} record
   * @param {IDeleteRecordTagsOptions} options
   * @returns {Promise<void>}
   */
  public static async deleteRecordTags (record: IRecord, { tagList }: IDeleteRecordTagsOptions): Promise<void> {
    const commandHandle = 0
    try {
      await createFFICallbackPromise<string>(
        (resolve, reject, cb) => {
          const rc = rustAPI().vcx_wallet_delete_record_tags(commandHandle,
            record.type_,
            record.id,
            JSON.stringify(tagList),
            cb)
          if (rc) {
            reject(rc)
          }
        },
        (resolve, reject) => Callback(
          'void',
          ['uint32','uint32','string'],
          (xhandle: number, err: number, receipt: string) => {
            if (err) {
              reject(err)
              return
            }
            resolve(receipt)
          })
      )
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

  /**
   * @memberof Wallet
   * @description Delete a record already in the wallet
   * @static
   * @async
   * @param {Record} record
   * @param {List} tagList
   * @returns {Promise<void>}
   */
  public static async deleteRecord ({ type, id }: IDeleteRecordData): Promise<void> {
    const commandHandle = 0
    try {
      await createFFICallbackPromise<string>(
        (resolve, reject, cb) => {
          const rc = rustAPI().vcx_wallet_delete_record(commandHandle,
            type,
            id,
            cb)
          if (rc) {
            reject(rc)
          }
        },
        (resolve, reject) => Callback(
          'void',
          ['uint32','uint32','string'],
          (xhandle: number, err: number, receipt: string) => {
            if (err) {
              reject(err)
              return
            }
            resolve(receipt)
          })
      )
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

  /**
   * @memberof Wallet
   * @description Retrieve a record already in the wallet
   * @static
   * @async
   * @param {String} type
   * @param {String} id
   * @returns {Promise<string>}
   */
  public static async getRecord ({ type, id }: IGerRecordData): Promise<string> {
    const commandHandle = 0
    try {
      return await createFFICallbackPromise<string>(
        (resolve, reject, cb) => {
          const rc = rustAPI().vcx_wallet_get_record(commandHandle,
            type,
            id,
            cb)
          if (rc) {
            reject(rc)
          }
        },
        (resolve, reject) => Callback(
          'void',
          ['uint32','uint32','string'],
          (xhandle: number, err: number, info: string) => {
            if (err) {
              reject(err)
              return
            }
            resolve(info)
          })
      )
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

   /**
    * @memberof Wallet
    * @description Open a search handle
    * @static
    * @async
    * @param {IOpenSearchData} searchData
    * @returns {Promise<string>}
    */
  public static async openSearch ({ type, queryJson, options }: IOpenSearchData): Promise<number> {
    const commandHandle = 0
    try {
      return await createFFICallbackPromise<number>(
        (resolve, reject, cb) => {
          const rc = rustAPI().vcx_wallet_open_search(commandHandle,
            type,
            queryJson,
            options,
            cb)
          if (rc) {
            reject(rc)
          }
        },
        (resolve, reject) => Callback(
          'void',
          ['uint32','uint32','uint32'],
          (xhandle: number, err: number, handle: number) => {
            if (err) {
              reject(err)
              return
            }
            resolve(handle)
          })
      )
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

  /**
   * @memberof Wallet
   * @description Open a search handle
   * @static
   * @async
   * @param {String} type
   * @param {String} id
   * @returns {Promise<string>}
   */
  public static async closeSearch (handle: number): Promise<void> {
    const commandHandle = 0
    try {
      await createFFICallbackPromise<number>(
        (resolve, reject, cb) => {
          const rc = rustAPI().vcx_wallet_close_search(commandHandle,
            handle,
            cb)
          if (rc) {
            reject(rc)
          }
        },
        (resolve, reject) => Callback(
          'void',
          ['uint32','uint32'],
          (xhandle: number, err: number) => {
            if (err) {
              reject(err)
              return
            }
            resolve(handle)
          })
      )
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

  /**
   * @memberof Wallet
   * @description Initiate or continue a search
   * @static
   * @async
   * @param {number} searchHandle
   * @param {number} count
   * @returns {Promise<string>}
   */
  public static async searchNextRecords (handle: number, { count }: ISearchNextRecordsOptions): Promise<string> {
    const commandHandle = 0
    try {
      return await createFFICallbackPromise<string>(
        (resolve, reject, cb) => {
          const rc = rustAPI().vcx_wallet_search_next_records(commandHandle,
            handle,
            count,
            cb)
          if (rc) {
            reject(rc)
          }
        },
        (resolve, reject) => Callback(
          'void',
          ['uint32','uint32','string'],
          (xhandle: number, err: number, info: string) => {
            if (err) {
              reject(err)
              return
            }
            resolve(info)
          })
      )
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }
}
