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
