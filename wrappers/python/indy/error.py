from enum import IntEnum
from typing import Optional


class ErrorCode(IntEnum):
    """ ErrorCode To Enum """
    Success = 0

    # Common errors

    # Caller passed invalid value as param 1 (null, invalid json and etc..)
    CommonInvalidParam1 = 100

    # Caller passed invalid value as param 2 (null, invalid json and etc..)
    CommonInvalidParam2 = 101

    # Caller passed invalid value as param 3 (null, invalid json and etc..)
    CommonInvalidParam3 = 102

    # Caller passed invalid value as param 4 (null, invalid json and etc..)
    CommonInvalidParam4 = 103

    # Caller passed invalid value as param 5 (null, invalid json and etc..)
    CommonInvalidParam5 = 104

    # Caller passed invalid value as param 6 (null, invalid json and etc..)
    CommonInvalidParam6 = 105

    # Caller passed invalid value as param 7 (null, invalid json and etc..)
    CommonInvalidParam7 = 106

    # Caller passed invalid value as param 8 (null, invalid json and etc..)
    CommonInvalidParam8 = 107

    # Caller passed invalid value as param 9 (null, invalid json and etc..)
    CommonInvalidParam9 = 108

    # Caller passed invalid value as param 10 (null, invalid json and etc..)
    CommonInvalidParam10 = 109

    # Caller passed invalid value as param 11 (null, invalid json and etc..)
    CommonInvalidParam11 = 110

    # Caller passed invalid value as param 12 (null, invalid json and etc..)
    CommonInvalidParam12 = 111

    # Invalid library state was detected in runtime. It signals library bug
    CommonInvalidState = 112

    # Object (json, config, key, credential, etc) passed by library caller has invalid structure
    CommonInvalidStructure = 113

    # IO Error
    CommonIOError = 114

    # Wallet errors
    # Caller passed invalid wallet handle
    WalletInvalidHandle = 200

    # Unknown type of wallet was passed on create_wallet
    WalletUnknownTypeError = 201

    # Attempt to register already existing wallet type
    WalletTypeAlreadyRegisteredError = 202

    # Attempt to create wallet with name used for another exists wallet
    WalletAlreadyExistsError = 203

    # Requested entity id isn't present in wallet
    WalletNotFoundError = 204

    # Trying to use wallet with pool that has different name
    WalletIncompatiblePoolError = 205

    # Trying to open wallet that was opened already
    WalletAlreadyOpenedError = 206

    # Attempt to open encrypted wallet with invalid credentials
    WalletAccessFailed = 207

    # Input provided to wallet operations is considered not valid
    WalletInputError = 208

    # Decoding of wallet data during input/output failed
    WalletDecodingError = 209

    # Storage error occurred during wallet operation
    WalletStorageError = 210

    # Error during encryption-related operations
    WalletEncryptionError = 211

    # Requested wallet item not found
    WalletItemNotFound = 212

    # Returned if wallet's add_record operation is used with record name that already exists
    WalletItemAlreadyExists = 213

    # Returned if provided wallet query is invalid
    WalletQueryError = 214

    # Ledger errors
    # Trying to open pool ledger that wasn't created before
    PoolLedgerNotCreatedError = 300

    # Caller passed invalid pool ledger handle
    PoolLedgerInvalidPoolHandle = 301

    # Pool ledger terminated
    PoolLedgerTerminated = 302

    # No consensus during ledger operation
    LedgerNoConsensusError = 303

    # Attempt to parse invalid transaction response
    LedgerInvalidTransaction = 304

    # Attempt to send transaction without the necessary privileges
    LedgerSecurityError = 305

    # Attempt to create pool ledger config with name used for another existing pool
    PoolLedgerConfigAlreadyExistsError = 306

    # Timeout for action
    PoolLedgerTimeout = 307

    # Attempt to open Pool for witch Genesis Transactions are not compatible with set Protocol
    # version. Call pool.indy_set_protocol_version to set correct Protocol version.
    PoolIncompatibleProtocolVersion = 308

    # Item not found on ledger.
    LedgerNotFound = 309

    # Revocation registry is full and creation of new registry is necessary
    AnoncredsRevocationRegistryFullError = 400

    AnoncredsInvalidUserRevocId = 401

    # Attempt to generate master secret with duplicated name
    AnoncredsMasterSecretDuplicateNameError = 404

    AnoncredsProofRejected = 405

    AnoncredsCredentialRevoked = 406

    # Attempt to create credential definition with duplicated did schema pair
    AnoncredsCredDefAlreadyExistsError = 407

    # Crypto errors
    # Unknown format of DID entity keys
    UnknownCryptoTypeError = 500

    # Attempt to create duplicate did
    DidAlreadyExistsError = 600

    # Unknown payment method was given
    PaymentUnknownMethodError = 700

    # No method were scraped from inputs/outputs or more than one were scraped
    PaymentIncompatibleMethodsError = 701

    # Insufficient funds on inputs
    PaymentInsufficientFundsError = 702

    # No such source on a ledger
    PaymentSourceDoesNotExistError = 703

    # Operation is not supported for payment method
    PaymentOperationNotSupportedError = 704

    # Extra funds on inputs
    PaymentExtraFundsError = 705

    # The transaction is not allowed to a requester
    TransactionNotAllowedError = 706


class IndyError(Exception):
    """ Base Exception for all errors raised by Indy """
    # error_code: ErrorCode - libindy error code
    # message: Optional[str] - human-readable error description
    # indy_backtrace: Optional[str] - error backtrace.
    #         Collecting of backtrace can be enabled by:
    #             1) setting environment variable `RUST_BACKTRACE=1`
    #             2) calling `set_runtime_config` function with `collect_backtrace: true`

    def __init__(self, error_code: ErrorCode, error_details: Optional[dict] = None):
        super().__init__()
        self.error_code = error_code
        if error_details:
            self.message = error_details.get('message')
            self.indy_backtrace = error_details.get('backtrace')


class CommonInvalidParam1(IndyError):
    """ Caller passed invalid value as param 1 (null, invalid json and etc..) """

class CommonInvalidParam2(IndyError):
    """ Caller passed invalid value as param 2 (null, invalid json and etc..) """

class CommonInvalidParam3(IndyError):
    """ Caller passed invalid value as param 3 (null, invalid json and etc..) """

class CommonInvalidParam4(IndyError):
    """ Caller passed invalid value as param 4 (null, invalid json and etc..) """

class CommonInvalidParam5(IndyError):
    """ Caller passed invalid value as param 5 (null, invalid json and etc..) """

class CommonInvalidParam6(IndyError):
    """ Caller passed invalid value as param 6 (null, invalid json and etc..) """

class CommonInvalidParam7(IndyError):
    """ Caller passed invalid value as param 7 (null, invalid json and etc..) """

class CommonInvalidParam8(IndyError):
    """ Caller passed invalid value as param 8 (null, invalid json and etc..) """

class CommonInvalidParam9(IndyError):
    """ Caller passed invalid value as param 9 (null, invalid json and etc..) """

class CommonInvalidParam10(IndyError):
    """ Caller passed invalid value as param 10 (null, invalid json and etc..) """

class CommonInvalidParam11(IndyError):
    """ Caller passed invalid value as param 11 (null, invalid json and etc..) """

class CommonInvalidParam12(IndyError):
    """ Caller passed invalid value as param 12 (null, invalid json and etc..) """

class CommonInvalidState(IndyError):
    """ Invalid library state was detected in runtime. It signals library bug """

class CommonInvalidStructure(IndyError):
    """ Object (json, config, key, credential, etc) passed by library caller has
        invalid structure
    """

class CommonIOError(IndyError):
    """ IO Error """

# Wallet errors
class WalletInvalidHandle(IndyError):
    """ Caller passed invalid wallet handle """

class WalletUnknownTypeError(IndyError):
    """ Unknown type of wallet was passed on create_wallet """

class WalletTypeAlreadyRegisteredError(IndyError):
    """ Attempt to register already existing wallet type """

class WalletAlreadyExistsError(IndyError):
    """ Attempt to create wallet with name used for another exists wallet """

class WalletNotFoundError(IndyError):
    """ Requested entity id isn't present in wallet """

class WalletIncompatiblePoolError(IndyError):
    """ Trying to use wallet with pool that has different name """

class WalletAlreadyOpenedError(IndyError):
    """ Trying to open wallet that was opened already """

class WalletAccessFailed(IndyError):
    """ Attempt to open encrypted wallet with invalid credentials """

class WalletInputError(IndyError):
    """ Input provided to wallet operations is considered not valid """

class WalletDecodingError(IndyError):
    """ Decoding of wallet data during input/output failed """

class WalletStorageError(IndyError):
    """ Storage error occurred during wallet operation """

class WalletEncryptionError(IndyError):
    """ Error during encryption-related operations """

class WalletItemNotFound(IndyError):
    """ Requested wallet item not found """

class WalletItemAlreadyExists(IndyError):
    """ Returned if wallet's add_record operation is used with record name that
        already exists
    """

class WalletQueryError(IndyError):
    """ Returned if provided wallet query is invalid """

# Ledger errors
class PoolLedgerNotCreatedError(IndyError):
    """ Trying to open pool ledger that wasn't created before """

class PoolLedgerInvalidPoolHandle(IndyError):
    """ Caller passed invalid pool ledger handle """

class PoolLedgerTerminated(IndyError):
    """ Pool ledger terminated """

class LedgerNoConsensusError(IndyError):
    """ No consensus during ledger operation """

class LedgerInvalidTransaction(IndyError):
    """ Attempt to parse invalid transaction response """

class LedgerSecurityError(IndyError):
    """ Attempt to send transaction without the necessary privileges """

class PoolLedgerConfigAlreadyExistsError(IndyError):
    """ Attempt to create pool ledger config with name used for another existing pool """

class PoolLedgerTimeout(IndyError):
    """ Timeout for action """

class PoolIncompatibleProtocolVersion(IndyError):
    """ Attempt to open Pool for witch Genesis Transactions are not compatible with set
        Protocol version. Call pool.indy_set_protocol_version to set correct Protocol version.
    """

class LedgerNotFound(IndyError):
    """ Item not found on ledger. """

class AnoncredsRevocationRegistryFullError(IndyError):
    """ Revocation registry is full and creation of new registry is necessary """

class AnoncredsInvalidUserRevocId(IndyError):
    """ Invalid User Revoc ID"""

class AnoncredsMasterSecretDuplicateNameError(IndyError):
    """ Attempt to generate master secret with duplicated name """

class AnoncredsProofRejected(IndyError):
    """ Proof Rejected """

class AnoncredsCredentialRevoked(IndyError):
    """ Credential Revoked """

class AnoncredsCredDefAlreadyExistsError(IndyError):
    """ Attempt to create credential definition with duplicated did schema pair """

# Crypto errors
class UnknownCryptoTypeError(IndyError):
    """ Unknown format of DID entity keys """

class DidAlreadyExistsError(IndyError):
    """ Attempt to create duplicate did """

class PaymentUnknownMethodError(IndyError):
    """ Unknown payment method was given """

class PaymentIncompatibleMethodsError(IndyError):
    """ No method were scraped from inputs/outputs or more than one were scraped """

class PaymentInsufficientFundsError(IndyError):
    """ Insufficient funds on inputs """

class PaymentSourceDoesNotExistError(IndyError):
    """ No such source on a ledger """

class PaymentOperationNotSupportedError(IndyError):
    """ Operation is not supported for payment method """

class PaymentExtraFundsError(IndyError):
    """ Extra funds on inputs """


def errorcode_to_exception(errorcode):
    """ Map ErrorCode to an exception class. """
    return {
        # Common Errors
        ErrorCode.CommonInvalidParam1: CommonInvalidParam1,
        ErrorCode.CommonInvalidParam2: CommonInvalidParam2,
        ErrorCode.CommonInvalidParam3: CommonInvalidParam3,
        ErrorCode.CommonInvalidParam4: CommonInvalidParam4,
        ErrorCode.CommonInvalidParam5: CommonInvalidParam5,
        ErrorCode.CommonInvalidParam6: CommonInvalidParam6,
        ErrorCode.CommonInvalidParam7: CommonInvalidParam7,
        ErrorCode.CommonInvalidParam8: CommonInvalidParam8,
        ErrorCode.CommonInvalidParam9: CommonInvalidParam9,
        ErrorCode.CommonInvalidParam10: CommonInvalidParam10,
        ErrorCode.CommonInvalidParam11: CommonInvalidParam11,
        ErrorCode.CommonInvalidParam12: CommonInvalidParam12,
        ErrorCode.CommonInvalidState: CommonInvalidState,
        ErrorCode.CommonInvalidStructure: CommonInvalidStructure,
        ErrorCode.CommonIOError: CommonIOError,
        # Wallet Errors
        ErrorCode.WalletInvalidHandle: WalletInvalidHandle,
        ErrorCode.WalletUnknownTypeError: WalletUnknownTypeError,
        ErrorCode.WalletTypeAlreadyRegisteredError: WalletTypeAlreadyRegisteredError,
        ErrorCode.WalletAlreadyExistsError: WalletAlreadyExistsError,
        ErrorCode.WalletNotFoundError: WalletNotFoundError,
        ErrorCode.WalletIncompatiblePoolError: WalletIncompatiblePoolError,
        ErrorCode.WalletAlreadyOpenedError: WalletAlreadyOpenedError,
        ErrorCode.WalletAccessFailed: WalletAccessFailed,
        ErrorCode.WalletInputError: WalletInputError,
        ErrorCode.WalletDecodingError: WalletDecodingError,
        ErrorCode.WalletStorageError: WalletStorageError,
        ErrorCode.WalletEncryptionError: WalletEncryptionError,
        ErrorCode.WalletItemNotFound: WalletItemNotFound,
        ErrorCode.WalletItemAlreadyExists: WalletItemAlreadyExists,
        ErrorCode.WalletQueryError: WalletQueryError,
        # Pool Errors
        ErrorCode.PoolLedgerNotCreatedError: PoolLedgerNotCreatedError,
        ErrorCode.PoolLedgerInvalidPoolHandle: PoolLedgerInvalidPoolHandle,
        ErrorCode.PoolLedgerTerminated: PoolLedgerTerminated,
        ErrorCode.LedgerNoConsensusError: LedgerNoConsensusError,
        ErrorCode.LedgerInvalidTransaction: LedgerInvalidTransaction,
        ErrorCode.LedgerSecurityError: LedgerSecurityError,
        ErrorCode.PoolLedgerConfigAlreadyExistsError: PoolLedgerConfigAlreadyExistsError,
        ErrorCode.PoolLedgerTimeout: PoolLedgerTimeout,
        ErrorCode.PoolIncompatibleProtocolVersion: PoolIncompatibleProtocolVersion,
        ErrorCode.LedgerNotFound: LedgerNotFound,
        # Anoncreds Errors
        ErrorCode.AnoncredsRevocationRegistryFullError: AnoncredsRevocationRegistryFullError,
        ErrorCode.AnoncredsInvalidUserRevocId: AnoncredsInvalidUserRevocId,
        ErrorCode.AnoncredsMasterSecretDuplicateNameError: AnoncredsMasterSecretDuplicateNameError,
        ErrorCode.AnoncredsProofRejected: AnoncredsProofRejected,
        ErrorCode.AnoncredsCredentialRevoked: AnoncredsCredentialRevoked,
        ErrorCode.AnoncredsCredDefAlreadyExistsError: AnoncredsCredDefAlreadyExistsError,
        # Crypto Errors
        ErrorCode.UnknownCryptoTypeError: UnknownCryptoTypeError,
        ErrorCode.DidAlreadyExistsError: DidAlreadyExistsError,
        ErrorCode.PaymentUnknownMethodError: PaymentUnknownMethodError,
        ErrorCode.PaymentIncompatibleMethodsError: PaymentIncompatibleMethodsError,
        ErrorCode.PaymentInsufficientFundsError: PaymentInsufficientFundsError,
        ErrorCode.PaymentSourceDoesNotExistError: PaymentSourceDoesNotExistError,
        ErrorCode.PaymentOperationNotSupportedError: PaymentOperationNotSupportedError,
        ErrorCode.PaymentExtraFundsError: PaymentExtraFundsError,
    }.get(errorcode, None)
