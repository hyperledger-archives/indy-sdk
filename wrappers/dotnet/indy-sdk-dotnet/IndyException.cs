using Hyperledger.Indy.AnonCredsApi;
using Hyperledger.Indy.LedgerApi;
using Hyperledger.Indy.PoolApi;
using Hyperledger.Indy.SignusApi;
using Hyperledger.Indy.WalletApi;
using System;

namespace Hyperledger.Indy
{
    /// <summary>
    /// Exception indicating a problem originating from the Indy SDK.
    /// </summary>
    public class IndyException : Exception
    {
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
            if (!Enum.IsDefined(typeof(ErrorCode), sdkErrorCode))
                return MakeExceptionForUnknownError(sdkErrorCode);

            switch ((ErrorCode)sdkErrorCode)
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
                    return new InvalidParameterException(sdkErrorCode);
                case ErrorCode.CommonInvalidState:
                    return new InvalidStateException(sdkErrorCode);
                case ErrorCode.CommonInvalidStructure:
                    return new InvalidStructureException(sdkErrorCode);
                case ErrorCode.CommonIOError:
                    return new IOException(sdkErrorCode);
                case ErrorCode.WalletInvalidHandle:
                    return new WalletClosedException(sdkErrorCode); 
                case ErrorCode.WalletUnknownTypeError:
                    return new UnknownWalletTypeException(sdkErrorCode); 
                case ErrorCode.WalletTypeAlreadyRegisteredError:
                    return new DuplicateWalletTypeException(sdkErrorCode);
                case ErrorCode.WalletAlreadyExistsError:
                    return new WalletExistsException(sdkErrorCode);
                case ErrorCode.WalletNotFoundError:
                    return new WalletValueNotFoundException(sdkErrorCode);
                case ErrorCode.WalletIncompatiblePoolError:
                    return new WrongWalletForPoolException(sdkErrorCode);
                case ErrorCode.WalletAlreadyOpenedError:
                    return new WalletAlreadyOpenedException(sdkErrorCode);
                case ErrorCode.PoolLedgerNotCreatedError:
                    return new PoolConfigNotCreatedException(sdkErrorCode);
                case ErrorCode.PoolLedgerInvalidPoolHandle:
                    return new PoolClosedException(sdkErrorCode);
                case ErrorCode.PoolLedgerTerminated:
                    return new PoolLedgerTerminatedException(sdkErrorCode);
                case ErrorCode.LedgerNoConsensusError:
                    return new LedgerConsensusException(sdkErrorCode);
                case ErrorCode.LedgerInvalidTransaction:
                    return new InvalidLedgerTransactionException(sdkErrorCode);
                case ErrorCode.LedgerSecurityError:
                    return new LedgerSecurityException(sdkErrorCode);
                case ErrorCode.PoolLedgerConfigAlreadyExistsError:
                    return new PoolLedgerConfigExistsException(sdkErrorCode);
                case ErrorCode.AnoncredsRevocationRegistryFullError:
                    return new RevocationRegistryFullException(sdkErrorCode);
                case ErrorCode.AnoncredsInvalidUserRevocIndex:
                    return new InvalidUserRevocIndexException(sdkErrorCode);
                case ErrorCode.AnoncredsAccumulatorIsFull:
                    return new AnoncredsAccumulatorFullException(sdkErrorCode);
                case ErrorCode.AnoncredsNotIssuedError:
                    return new AnoncredsNotIssuedException(sdkErrorCode);
                case ErrorCode.AnoncredsMasterSecretDuplicateNameError:
                    return new DuplicateMasterSecretNameException(sdkErrorCode);
                case ErrorCode.AnoncredsProofRejected:
                    return new ProofRejectedException(sdkErrorCode);
                case ErrorCode.AnoncredsClaimRevoked:
                    return new ClaimRevokedException(sdkErrorCode);
                case ErrorCode.SignusUnknownCryptoError:
                    return new UnknownCryptoException(sdkErrorCode);
                default:
                    return MakeExceptionForUnknownError(sdkErrorCode);
            }      
        }

        private static IndyException MakeExceptionForUnknownError(int sdkErrorCode)
        {
            var message = string.Format("An unmapped error with the code '{0}' was returned by the SDK.", sdkErrorCode);
            return new IndyException(message, sdkErrorCode);
        }

        /// <summary>
        /// Gets the error code for the exception.
        /// </summary>
        public int SdkErrorCode { get; private set; }
    }

}
