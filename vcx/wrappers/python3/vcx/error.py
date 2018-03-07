from enum import IntEnum


class ErrorCode(IntEnum):
    Success = 0,
    UnknownError = 1001,
    ConnectionError = 1002,
    InvalidConnectionHandle = 1003,
    InvalidConfiguration = 1004,
    NotReady = 1005,
    InvalidOption = 1007,
    InvalidDid = 1008,
    InvalidIssuerClaimHandle = 1015,
    InvalidJson = 1016,
    InvalidProofHandle = 1017,
    InvalidProof = 1023,
    InvalidSchema = 1031,
    UnknownLibindyError = 1035,
    InvalidClaimDef = 1036,
    InvalidClaimDefHandle = 1037,
    InvalidSchemaHandle = 1042,
    InvalidSchemaSequenceNumber = 1040,
    AlreadyInitialized = 1044,


class VcxError(Exception):
    # error_code: ErrorCode

    def __init__(self, error_code: ErrorCode, error_msg: str):
        self.error_code = error_code
        self.error_msg = error_msg
