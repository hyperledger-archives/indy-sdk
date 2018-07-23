var util = require('util')

var errors = {
  100: 'CommonInvalidParam1',
  101: 'CommonInvalidParam2',
  102: 'CommonInvalidParam3',
  103: 'CommonInvalidParam4',
  104: 'CommonInvalidParam5',
  105: 'CommonInvalidParam6',
  106: 'CommonInvalidParam7',
  107: 'CommonInvalidParam8',
  108: 'CommonInvalidParam9',
  109: 'CommonInvalidParam10',
  110: 'CommonInvalidParam11',
  111: 'CommonInvalidParam12',
  112: 'CommonInvalidState',
  113: 'CommonInvalidStructure',
  114: 'CommonIOError',
  200: 'WalletInvalidHandle',
  201: 'WalletUnknownTypeError',
  202: 'WalletTypeAlreadyRegisteredError',
  203: 'WalletAlreadyExistsError',
  204: 'WalletNotFoundError',
  205: 'WalletIncompatiblePoolError',
  206: 'WalletAlreadyOpenedError',
  207: 'WalletAccessFailed',
  300: 'PoolLedgerNotCreatedError',
  301: 'PoolLedgerInvalidPoolHandle',
  302: 'PoolLedgerTerminated',
  303: 'LedgerNoConsensusError',
  304: 'LedgerInvalidTransaction',
  305: 'LedgerSecurityError',
  306: 'PoolLedgerConfigAlreadyExistsError',
  307: 'PoolLedgerTimeout',
  308: 'PoolIncompatibleProtocolVersion',
  400: 'AnoncredsRevocationRegistryFullError',
  401: 'AnoncredsInvalidUserRevocId',
  404: 'AnoncredsMasterSecretDuplicateNameError',
  405: 'AnoncredsProofRejected',
  406: 'AnoncredsCredentialRevoked',
  407: 'AnoncredsCredDefAlreadyExistsError',
  500: 'UnknownCryptoTypeError',
  600: 'DidAlreadyExistsError',
  700: 'PaymentUnknownMethodError',
  701: 'PaymentIncompatibleMethodsError',
  702: 'PaymentInsufficientFundsError',
  703: 'PaymentSourceDoesNotExistError',
  704: 'PaymentOperationNotSupportedError'
}

function IndyError (err) {
  Error.call(this)
  Error.captureStackTrace(this, this.constructor)
  this.name = this.constructor.name
  if (errors.hasOwnProperty(err)) {
    this.message = errors[err]
    this.indyCode = err
    this.indyName = errors[err]
  } else {
    this.message = (err + '')
  }
}
util.inherits(IndyError, Error)

module.exports = IndyError
