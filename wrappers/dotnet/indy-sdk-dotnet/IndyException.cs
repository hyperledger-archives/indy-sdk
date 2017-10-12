using System;
using System.Diagnostics;

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
        /// Generates an IndyException from the provided error code.
        /// </summary>
        /// <param name="sdkErrorCode">The error code.</param>
        /// <returns>An IndyException instance.</returns>
        public static IndyException FromErrorCode(int sdkErrorCode)
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

    /// <summary>
    /// Exception indicating that one of the parameters provided to an SDK call contained a valid that was considered invalid.
    /// </summary>
    public class InvalidParameterException : IndyException
    {
        private static int GetParamIndex(int sdkErrorCode)
        {
            Debug.Assert((int)sdkErrorCode >= 100 && (int)sdkErrorCode <= 111);

            return (int)sdkErrorCode - 99;
        }

        private static string BuildMessage(int sdkErrorCode)
        {
            return string.Format("The value passed to parameter {0} is not valid.", GetParamIndex(sdkErrorCode));
        }

        internal InvalidParameterException(int sdkErrorCode) : base(BuildMessage(sdkErrorCode), sdkErrorCode)
        {
            ParameterIndex = GetParamIndex(sdkErrorCode);
        }

        /// <summary>
        /// Gets the index of the parameter that contained the invalid value.
        /// </summary>
        public int ParameterIndex { get; private set; }
    }

    /// <summary>
    /// Exception indicating that the SDK library experienced an unexpected internal error.
    /// </summary>
    public class InvalidStateException : IndyException
    {
        private const string message = "The SDK library experienced an unexpected internal error.";

        internal InvalidStateException(int sdkErrorCode) : base(message, sdkErrorCode)
        {
        }
    }

    /// <summary>
    /// Exception indicating that a value being processed was not considered a valid value.
    /// </summary>
    public class InvalidStructureException : IndyException
    {
        const string message = "A value being processed is not valid.";

        internal InvalidStructureException(int sdkErrorCode) : base(message, sdkErrorCode)
        {

        }
    }

    /// <summary>
    /// Exception indicating that an IO error occurred.
    /// </summary>
    public class IOException : IndyException
    {
        const string message = "An IO error occurred.";

        internal IOException(int sdkErrorCode) : base(message, sdkErrorCode)
        {

        }
    }

    /// <summary>
    /// Exception thrown when an attempt is made to use a closed wallet.
    /// </summary>
    public class WalletClosedException : IndyException
    {
        const string message = "The wallet is closed and cannot be used.";

        internal WalletClosedException(int sdkErrorCode) : base(message, sdkErrorCode)
        {

        }
    }
    
    /// <summary>
    /// Exception thrown when attempting to open a wallet with a type that has not been registered.
    /// </summary>
    public class UnknownWalletTypeException : IndyException
    {
        const string message = "The wallet type specified has not been registered.";

        internal UnknownWalletTypeException(int sdkErrorCode) : base(message, sdkErrorCode)
        {

        }
    }

    /// <summary>
    /// Exception thrown when registering a wallet type that has already been registered.
    /// </summary>
    public class DuplicateWalletTypeException : IndyException
    {
        const string message = "A wallet type with the specified name has already been registered.";

        internal DuplicateWalletTypeException(int sdkErrorCode) : base(message, sdkErrorCode)
        {

        }
    }

    /// <summary>
    /// Exception thrown when creating a wallet and a wallet with the same name already exists.
    /// </summary>
    public class WalletExistsException : IndyException
    {
        const string message = "A wallet with the specified name already exists.";

        internal WalletExistsException(int sdkErrorCode) : base(message, sdkErrorCode)
        {

        }
    }

    /// <summary>
    /// Exception thrown when requesting a value from a wallet and the specified key does not exist.
    /// </summary>
    public class WalletValueNotFoundException : IndyException
    {
        const string message = "The no value with the specified key exists in the wallet from which it was requested.";

        internal WalletValueNotFoundException(int sdkErrorCode) : base(message, sdkErrorCode)
        {

        }
    }

    /// <summary>
    /// Exception thrown when attempting to use a wallet with the wrong pool.
    /// </summary>
    public class WrongWalletForPoolException : IndyException
    {
        const string message = "The wallet specified is not compatible with the open pool.";

        internal WrongWalletForPoolException(int sdkErrorCode) : base(message, sdkErrorCode)
        {

        }
    }

    /// <summary>
    /// Exception thrown when attempting to open a wallet that was already opened.
    /// </summary>
    public class WalletAlreadyOpenedException : IndyException
    {
        const string message = "The wallet is already open.";

        internal WalletAlreadyOpenedException(int sdkErrorCode) : base(message, sdkErrorCode)
        {

        }
    }

    /// <summary>
    /// Exception thrown when attempting to open pool which does not yet have a created configuration.
    /// </summary>
    public class PoolConfigNotCreatedException : IndyException
    {
        const string message = "The requested pool cannot be opened because it does not have an existing configuration.";

        internal PoolConfigNotCreatedException(int sdkErrorCode) : base(message, sdkErrorCode)
        {

        }
    }

    /// <summary>
    /// Exception thrown when attempting to use a pool that has been closed.
    /// </summary>
    public class PoolClosedException : IndyException
    {
        const string message = "The pool is closed and cannot be used.";

        internal PoolClosedException(int sdkErrorCode) : base(message, sdkErrorCode)
        {

        }
    }

    /// <summary>
    /// Exception thrown when the pool ledger was terminated.
    /// </summary>
    public class PoolLedgerTerminatedException : IndyException
    {
        const string message = "The pool ledger was terminated.";

        internal PoolLedgerTerminatedException(int sdkErrorCode) : base(message, sdkErrorCode)
        {

        }
    }

    /// <summary>
    /// Exception thrown when the no consensus was reached during a ledger operation.
    /// </summary>
    public class LedgerConsensusException : IndyException
    {
        const string message = "No consensus was reached during the ledger operation";

        internal LedgerConsensusException(int sdkErrorCode) : base(message, sdkErrorCode)
        {

        }
    }

    /// <summary>
    /// Exception thrown when attempting to send an unknown or incomplete ledger message.
    /// </summary>
    public class InvalidLedgerTransactionException : IndyException
    {
        const string message = "The ledger message is unknown or malformed.";

        internal InvalidLedgerTransactionException(int sdkErrorCode) : base(message, sdkErrorCode)
        {

        }
    }

    /// <summary>
    /// Exception thrown when attempting to send a transaction without the necessary privileges.
    /// </summary>
    public class LedgerSecurityException : IndyException
    {
        const string message = "The transaction cannot be sent as the privileges for the current pool connection don't allow it.";

        internal LedgerSecurityException(int sdkErrorCode) : base(message, sdkErrorCode)
        {

        }
    }

    /// <summary>
    /// Exception thrown when attempting to create a pool ledger config with same name as an existing pool ledger config.
    /// </summary>
    public class PoolLedgerConfigExistsException : IndyException
    {
        const string message = "A pool ledger configuration already exists with the specified name.";

        internal PoolLedgerConfigExistsException(int sdkErrorCode) : base(message, sdkErrorCode)
        {

        }
    }

    /// <summary>
    /// Exception thrown when attempting to use a full revocation registry.
    /// </summary>
    public class RevocationRegistryFullException : IndyException
    {
        const string message = "The specified revocation registry is full.  Another revocation registry must be created.";

        internal RevocationRegistryFullException(int sdkErrorCode) : base(message, sdkErrorCode)
        {

        }
    }

    /// <summary>
    /// Exception thrown when an invalid user revocation registry index is used.
    /// </summary>
    public class InvalidUserRevocIndexException : IndyException
    {
        const string message = "The user revocation registry index specified is invalid.";

        internal InvalidUserRevocIndexException(int sdkErrorCode) : base(message, sdkErrorCode)
        {

        }
    }

    /// <summary>
    /// Exception thrown when an anoncreds accululator is full.
    /// </summary>
    public class AnoncredsAccumulatorFullException : IndyException
    {
        const string message = "The anoncreds accumulator is full.";

        internal AnoncredsAccumulatorFullException(int sdkErrorCode) : base(message, sdkErrorCode)
        {

        }
    }

    /// <summary>
    /// Exception thrown when an anoncreds is not issued.
    /// </summary>
    public class AnoncredsNotIssuedException : IndyException
    {
        const string message = "The anoncreds is not issued.";

        internal AnoncredsNotIssuedException(int sdkErrorCode) : base(message, sdkErrorCode)
        {

        }
    }

    /// <summary>
    /// Exception thrown when an attempt to create a master-secret with the same name as an existing master-secret.
    /// </summary>
    public class DuplicateMasterSecretNameException : IndyException
    {
        const string message = "Another master-secret with the specified name already exists.";

        internal DuplicateMasterSecretNameException(int sdkErrorCode) : base(message, sdkErrorCode)
        {

        }
    }

    /// <summary>
    /// Exception thrown when a proof has been rejected.
    /// </summary>
    public class ProofRejectedException : IndyException
    {
        const string message = "The proof has been rejected.";

        internal ProofRejectedException(int sdkErrorCode) : base(message, sdkErrorCode)
        {

        }
    }

    /// <summary>
    /// Exception thrown when a claim has been revoked.
    /// </summary>
    public class ClaimRevokedException : IndyException
    {
        const string message = "The claim has been revoked.";

        internal ClaimRevokedException(int sdkErrorCode) : base(message, sdkErrorCode)
        {

        }
    }

    /// <summary>
    /// Exception thrown when an unknown crypto format is used for DID entity keys.
    /// </summary>
    public class UnknownCryptoException : IndyException
    {
        const string message = "An unknown crypto format has been used for a DID entity key.";

        internal UnknownCryptoException(int sdkErrorCode) : base(message, sdkErrorCode)
        {

        }
    }

}
