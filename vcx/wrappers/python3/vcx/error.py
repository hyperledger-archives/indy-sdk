import json

LIBRARY = "libvcx.so"
from vcx.cdll import _cdll
from enum import IntEnum
from ctypes import c_char_p, byref, c_uint32
import logging
from typing import Optional


class ErrorCode(IntEnum):
    Success = 0,
    IndyInvalidWalletHandle = 200,
    IndyWalletRecordNotFound = 212,
    IndyDuplicateWalletRecord = 213,
    UnknownError = 1001,
    ConnectionError = 1002,
    InvalidConnectionHandle = 1003,
    InvalidConfiguration = 1004,
    NotReady = 1005,
    NoEndpoint = 1006,
    InvalidOption = 1007,
    InvalidDid = 1008,
    InvalidVerkey = 1009,
    CouldNotConnect = 1010,
    InvalidNonce = 1011,
    InvalidKeyDelegate = 1012,
    InvalidUrl = 1013,
    NotBase58 = 1014,
    InvalidIssuerCredentialHandle = 1015,
    InvalidJson = 1016,
    InvalidProofHandle = 1017,
    InvalidCredentialRequest = 1018,
    InvalidMessagePack = 1019,
    InvalidMessages = 1020,
    InvalidAttributesStructure = 1021,
    BigNumberError = 1022,
    InvalidProof = 1023,
    InvalidGenesisTxnPath = 1024,
    CreatePoolConfigParameters = 1025,
    CreatePoolConifg = 1026,
    InvalidProofCredentialData = 1027,
    IndySubmitRequestError = 1028,
    BuildCredentialDefReqErr = 1029,
    NoPoolOpen = 1030,
    InvalidSchema = 1031,
    FailedPoolCompliance = 1032,
    InvalidHttpResponse = 1033,
    CreateCredentialDefErr = 1034,
    UnknownLibindyError = 1035,
    InvalidCredentialDef = 1036,
    InvalidCredentialDefHandle = 1037,
    TimeoutLibindyError = 1038,
    CredentialDefAlreadyCreated = 1039,
    InvalidSchemaSequenceNumber = 1040,
    InvalidSchemaCreation = 1041,
    InvalidSchemaHandle = 1042,
    InvalidMasterSecret = 1043,
    AlreadyInitialized = 1044,
    InvalidInviteDetails = 1045,
    InvalidSelfAttestedVal = 1046,
    InvalidPredicate = 1047,
    InvalidObjHandle = 1048,
    InvalidDisclosedProofHandle = 1049,
    SerializationError = 1050,
    WalletAlreadyExists = 1051,
    WalletAlreadyOpen = 1052,
    InvalidCredentialHandle = 1053,
    InvalidCredentialJson = 1054,
    CreateCredentialFailed = 1055,
    CreateProofError = 1056,
    InvalidWalletHandle = 1057,
    InvalidWalletCreation = 1058,
    InvalidPoolName = 1059,
    CannotDeleteConnection = 1060,
    CreateConnectionError = 1061,
    InvalidWalletSetup = 1062,
    CommonError = 1063,
    InsufficientTokenAmount = 1064,
    UnknownTxnType = 1065,
    InvalidPaymentAddress = 1066,
    InvalidLibindyParam = 1067,
    InvalidPayment = 1068,
    MissingWalletKey = 1069,
    ObjectCacheError = 1070,
    NoPaymentInformation = 1071,
    DuplicateWalletRecord = 1072,
    WalletRecordNotFound = 1073,
    IOError = 1074,
    InvalidWalletStorageParam = 1075,
    MissingWalletName = 1076,
    MissingExportedWalletPath = 1077,
    MissingBackupKey = 1078,
    WalletNotFound = 1079,
    LibindyInvalidStructure = 1080,
    InvalidState = 1081,
    InvalidLedgerResponse = 1082,
    DidAlreadyExistsInWallet = 1083,
    DuplicateMasterSecret = 1084,
    ThreadError = 1085,
    InvalidProofRequest = 1086,
    MissingPaymentMethod = 1087,
    DuplicateSchema = 1088,
    UnknownLibindyRejection = 1089,
    LoggingError = 1090
    InvalidRevocationDetails = 1091,
    InvalidRevEntry = 1092,
    InvalidRevocationTimestamp = 1093,
    UnknownSchemaRejection = 1094,
    InvalidRevRegDefCreation = 1095


class VcxError(Exception):
    # error_code: ErrorCode
    # error_msg: Optional[str] - human-readable error description
    # sdk_error_full_message: Optional[str] - vcx full error message.
    # sdk_error_cause: Optional[str] - vcx error cause.
    # sdk_error_backtrace: Optional[str] - vcx error backtrace.
    #   Collecting of backtrace can be enabled by setting environment variable `RUST_BACKTRACE=1`

    def __init__(self, error_code: ErrorCode, error_details: Optional[dict] = None):
        self.error_code = error_code
        self.error_msg = error_message(error_code)
        if error_details:
            self.error_msg = error_details['error']
            self.sdk_error_full_message = error_details['message']
            self.sdk_error_cause = error_details['cause']
            self.sdk_error_backtrace = error_details['backtrace']


def error_message(error_code: int) -> str:
    logger = logging.getLogger(__name__)
    name = 'vcx_error_c_message'
    c_error_code = c_uint32(error_code)
    getattr(_cdll(), name).restype = c_char_p
    err_msg = getattr(_cdll(), name)(c_error_code)
    logger.debug("error_message: Function %s[%s] returned error_message: %s", name, error_code, err_msg)
    return err_msg.decode()


def get_error_details() -> Optional[dict]:
    logger = logging.getLogger(__name__)
    logger.debug("get_error_details: >>>")

    error_c = c_char_p()
    getattr(_cdll(), 'vcx_get_current_error')(byref(error_c))
    error_details = json.loads(error_c.value.decode()) if error_c.value else None

    logger.debug("get_error_details: <<< error_details: %s", error_details)
    return error_details
