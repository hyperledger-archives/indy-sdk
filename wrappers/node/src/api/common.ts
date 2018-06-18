export enum Error {
    SUCCESS = 0,
    UNKNOWN_ERROR = 1001,
    CONNECTION_ERROR = 1002,
    INVALID_CONNECTION_HANDLE = 1003,
    INVALID_CONFIGURATION = 1004,
    NOT_READY = 1005,
    NO_ENDPOINT = 1006,
    INVALID_OPTION = 1007,
    INVALID_DID = 1008,
    INVALID_VERKEY = 1009,
    POST_MSG_FAILURE = 1010,
    INVALID_NONCE = 1011,
    INVALID_KEY_DELEGATE = 1012,
    INVALID_URL = 1013,
    NOT_BASE58 = 1014,
    INVALID_ISSUER_CREDENTIAL_HANDLE = 1015
}

export enum StateType {
    None = 0,
    Initialized = 1,
    OfferSent = 2,
    RequestReceived = 3,
    Accepted = 4,
    Unfulfilled = 5,
    Expired = 6,
    Revoked = 7
}

export enum IndyTransactions {
  NODE = '0',
  NYM = '1',
  ATTRIB = '100',
  SCHEMA = '101',
  CLAIM_DEF = '102',

  DISCLO = '103',
  GET_ATTR = '104',
  GET_NYM = '105',
  GET_TXNS = '3',
  GET_SCHEMA = '107',
  GET_CLAIM_DEF = '108',

  POOL_UPGRADE = '109',
  NODE_UPGRADE = '110',

  POOL_CONFIG = '111',

  CHANGE_KEY = '112',

  REVOC_REG_DEF = '113',
  REVOC_REG_ENTRY = '114',
  GET_REVOC_REG_DEF = '115',
  GET_REVOC_REG = '116',
  GET_REVOC_REG_DELTA = '117',

  POOL_RESTART = '118',
  VALIDATOR_INFO = '119'
}

export interface IInitVCXOptions {
  libVCXPath?: string
}
