using Hyperledger.Indy.AnonCredsApi;
using Hyperledger.Indy.DidApi;
using Hyperledger.Indy.LedgerApi;
using Hyperledger.Indy.PaymentsApi;
using Hyperledger.Indy.PoolApi;
using Hyperledger.Indy.WalletApi;
using System;

namespace Hyperledger.Indy
{
    /// <summary>
    /// Exception indicating a problem originating from the Indy SDK.
    /// </summary>
    public class IndyException : Exception
    {
        /// <summary>
        /// Initializes a new IndyException with the specified message and SDK error code.
        /// </summary>
        /// <param name="message">The message for the exception.</param>
        /// <param name="sdkErrorCode">The SDK error code for the exception.</param>
        internal IndyException(String message, int sdkErrorCode) : base(message)
        {
            SdkErrorCode = sdkErrorCode;
        }

        /// <summary>
        /// Generates an IndyException or one of its subclasses from the provided SDK error code.
        /// </summary>
        /// <param name="sdkErrorCode">The error code.</param>
        /// <returns>An IndyException or subclass instance.</returns>
        internal static IndyException FromSdkError(int sdkErrorCode)
        {
            var errorCode = (ErrorCode)sdkErrorCode;
            
            switch (errorCode)
            {
                case ErrorCode.CommonInvalidParam1:
                case ErrorCode.CommonInvalidParam2:
                case ErrorCode.CommonInvalidParam3:
                case ErrorCode.CommonInvalidParam4:
                case ErrorCode.CommonInvalidParam5:
                case ErrorCode.CommonInvalidParam6:
                case ErrorCode.CommonInvalidParam7:
                case ErrorCode.CommonInvalidParam8:
                case ErrorCode.CommonInvalidParam9:
                case ErrorCode.CommonInvalidParam10:
                case ErrorCode.CommonInvalidParam11:
                case ErrorCode.CommonInvalidParam12:
                case ErrorCode.CommonInvalidParam13:
                case ErrorCode.CommonInvalidParam14:
                case ErrorCode.CommonInvalidParam15:
                case ErrorCode.CommonInvalidParam16:
                case ErrorCode.CommonInvalidParam17:
                case ErrorCode.CommonInvalidParam18:
                case ErrorCode.CommonInvalidParam19:
                case ErrorCode.CommonInvalidParam20:
                case ErrorCode.CommonInvalidParam21:
                case ErrorCode.CommonInvalidParam22:
                case ErrorCode.CommonInvalidParam23:
                case ErrorCode.CommonInvalidParam24:
                case ErrorCode.CommonInvalidParam25:
                case ErrorCode.CommonInvalidParam26:
                case ErrorCode.CommonInvalidParam27:
                    return new InvalidParameterException(sdkErrorCode);
                case ErrorCode.CommonInvalidState:
                    return new InvalidStateException();
                case ErrorCode.CommonInvalidStructure:
                    return new InvalidStructureException();
                case ErrorCode.CommonIOError:
                    return new IOException();
                case ErrorCode.WalletInvalidHandle:
                    return new InvalidWalletException(); 
                case ErrorCode.WalletUnknownTypeError:
                    return new UnknownWalletTypeException(); 
                case ErrorCode.WalletTypeAlreadyRegisteredError:
                    return new DuplicateWalletTypeException();
                case ErrorCode.WalletAlreadyExistsError:
                    return new WalletExistsException();
                case ErrorCode.WalletNotFoundError:
                    return new WalletNotFoundException();
                case ErrorCode.WalletIncompatiblePoolError:
                    return new WrongWalletForPoolException();
                case ErrorCode.WalletAlreadyOpenedError:
                    return new WalletAlreadyOpenedException();
                case ErrorCode.WalletAccessFailed:
                    return new WalletAccessFailedException();
                case ErrorCode.PoolLedgerNotCreatedError:
                    return new PoolConfigNotCreatedException();
                case ErrorCode.PoolLedgerInvalidPoolHandle:
                    return new InvalidPoolException();
                case ErrorCode.PoolLedgerTerminated:
                    return new PoolLedgerTerminatedException();
                case ErrorCode.PoolIncompatibleProtocolVersionError:
                    return new PoolIncompatibleProtocolVersionException();
                case ErrorCode.PoolLedgerConfigAlreadyExistsError:
                    return new PoolLedgerConfigExistsException();
                case ErrorCode.LedgerNoConsensusError:
                    return new LedgerConsensusException();
                case ErrorCode.LedgerInvalidTransaction:
                    return new InvalidLedgerTransactionException();
                case ErrorCode.LedgerSecurityError:
                    return new LedgerSecurityException();
                case ErrorCode.AnoncredsRevocationRegistryFullError:
                    return new RevocationRegistryFullException();
                case ErrorCode.AnoncredsInvalidUserRevocId:
                    return new InvalidUserRevocIdException();
                case ErrorCode.AnoncredsMasterSecretDuplicateNameError:
                    return new DuplicateMasterSecretNameException();
                case ErrorCode.AnoncredsProofRejected:
                    return new ProofRejectedException();
                case ErrorCode.AnoncredsCredentialRevoked:
                    return new CredentialRevokedException();
                case ErrorCode.AnoncredsCredDefAlreadyExistsError:
                    return new CredentialDefinitionAlreadyExistsException();
                case ErrorCode.UnknownCryptoTypeError:
                    return new UnknownCryptoTypeException();
                case ErrorCode.WalletItemNotFoundError:
                    return new WalletItemNotFoundException();
                case ErrorCode.WalletItemAlreadyExistsError:
                    return new WalletItemAlreadyExistsException();
                case ErrorCode.WalletQueryError:
                    return new WalletInvalidQueryException();
                case ErrorCode.WalletStorageError:
                    return new WalletStorageException();
                case ErrorCode.WalletDecodingError:
                    return new WalletDecodingException();
                case ErrorCode.WalletEncryptionError:
                    return new WalletEncryptionException();
                case ErrorCode.WalletInputError:
                    return new WalletInputException();
                case ErrorCode.PaymentExtraFundsError:
                    return new ExtraFundsException();
                case ErrorCode.PaymentIncompatibleMethodsError:
                    return new IncompatiblePaymentMethodsException();
                case ErrorCode.PaymentInsufficientFundsError:
                    return new InsufficientFundsException();
                case ErrorCode.PaymentOperationNotSupportedError:
                    return new PaymentOperationNotSupportedException();
                case ErrorCode.PaymentSourceDoesNotExistError:
                    return new PaymentSourceDoesNotExistException();
                case ErrorCode.PaymentUnknownMethodError:
                    return new UnknownPaymentMethodException();

                default:
                    var message = $"An unmapped error with the code '{sdkErrorCode}' was returned by the SDK.";
                    return new IndyException(message, sdkErrorCode);
            }      
        }

        /// <summary>
        /// Gets the error code for the exception.
        /// </summary>
        public int SdkErrorCode { get; private set; }
    }

}
