package org.hyperledger.indy.sdk;

import org.hyperledger.indy.sdk.anoncreds.*;
import org.hyperledger.indy.sdk.did.DidAlreadyExistsException;
import org.hyperledger.indy.sdk.ledger.ConsensusException;
import org.hyperledger.indy.sdk.ledger.LedgerInvalidTransactionException;
import org.hyperledger.indy.sdk.ledger.LedgerSecurityException;
import org.hyperledger.indy.sdk.ledger.TimeoutException;
import org.hyperledger.indy.sdk.payments.*;
import org.hyperledger.indy.sdk.pool.*;
import org.hyperledger.indy.sdk.crypto.UnknownCryptoException;
import org.hyperledger.indy.sdk.wallet.*;

/**
 * Thrown when an Indy specific error has occurred.
 */
public class IndyException extends Exception {

	private static final long serialVersionUID = 2650355290834266477L;
	private int sdkErrorCode;
	private String sdkMessage;
	private String sdkBacktrace; // Collecting of backtrace can be enabled by:
								 //   1) setting environment variable `RUST_BACKTRACE=1`
								 //   2) calling `setRuntimeConfig` API function with `collect_backtrace: true`

	/**
	 * Initializes a new IndyException with the specified message.
	 *
	 * @param message The message for the exception.
	 * @param sdkErrorCode The SDK error code to construct the exception from.
	 */
	protected IndyException(String message, int sdkErrorCode) {
		super(message);
		this.sdkErrorCode = sdkErrorCode;
	}
	/**
	 * Initializes a new IndyException with the specified message.
	 *
	 * @param message The message for the exception.
	 * @param sdkErrorCode The SDK error code to construct the exception from.
	 * @param sdkBacktrace The SDK backtrace to construct the exception from.
	 */
	protected IndyException(String message, int sdkErrorCode, String sdkBacktrace) {
		super(message);
		this.sdkErrorCode = sdkErrorCode;
		this.sdkBacktrace = sdkBacktrace;
	}

	/**
	 * Gets the SDK error code for the exception.
	 *
	 * @return The SDK error code used to construct the exception.
	 */
	public int getSdkErrorCode() {
		return sdkErrorCode;
	}

	/**
	 * Sets the SDK error message for the exception.
	 */
	private void setSdkMessage(String sdkMessage) {
		this.sdkMessage = sdkMessage;
	}

	/**
	 * Gets the SDK error backtrace for the exception.
	 *
	 * @return The SDK backtrace.
	 */
	public String getSdkMessage() {
		return sdkMessage;
	}

	/**
	 * Sets the SDK error backtrace for the exception.
	 */
	private void setSdkBacktrace(String sdkBacktrace) {
		this.sdkBacktrace = sdkBacktrace;
	}

	/**
	 * Gets the SDK error backtrace for the exception.
	 *
	 * @return The SDK backtrace.
	 */
	public String getSdkBacktrace() {
		return sdkBacktrace;
	}

	/**
	 * Initializes a new IndyException using the specified SDK error code.
	 *
	 * @param sdkErrorCode The SDK error code to construct the exception from.
	 *
	 * @return IndyException correspondent to SDK error code
	 */
	public static IndyException fromSdkError(int sdkErrorCode, String sdkMessage, String sdkBacktrace) {

		ErrorCode errorCode = ErrorCode.valueOf(sdkErrorCode);

		switch (errorCode) {
			case CommonInvalidParam1:
			case CommonInvalidParam2:
			case CommonInvalidParam3:
			case CommonInvalidParam4:
			case CommonInvalidParam5:
			case CommonInvalidParam6:
			case CommonInvalidParam7:
			case CommonInvalidParam8:
			case CommonInvalidParam9:
			case CommonInvalidParam10:
			case CommonInvalidParam11:
			case CommonInvalidParam12:
			case CommonInvalidParam13:
			case CommonInvalidParam14:
				return new InvalidParameterException(sdkErrorCode, sdkMessage, sdkBacktrace);
			case CommonInvalidState:
				return new InvalidStateException(sdkMessage, sdkBacktrace);
			case CommonInvalidStructure:
				return new InvalidStructureException(sdkMessage, sdkBacktrace);
			case CommonIOError:
				return new IOException(sdkMessage, sdkBacktrace);
			case WalletInvalidHandle:
				return new InvalidWalletException(sdkMessage, sdkBacktrace);
			case WalletUnknownTypeError:
				return new UnknownWalletTypeException(sdkMessage, sdkBacktrace);
			case WalletTypeAlreadyRegisteredError:
				return new DuplicateWalletTypeException(sdkMessage, sdkBacktrace);
			case WalletAlreadyExistsError:
				return new WalletExistsException(sdkMessage, sdkBacktrace);
			case WalletNotFoundError:
				return new WalletNotFoundException(sdkMessage, sdkBacktrace);
			case WalletInputError:
				return new WalletInputException(sdkMessage, sdkBacktrace);
			case WalletDecodingError:
				return new WalletDecodingException(sdkMessage, sdkBacktrace);
			case WalletStorageError:
				return new WalletStorageException(sdkMessage, sdkBacktrace);
			case WalletEncryptionError:
				return new WalletEncryptionException(sdkMessage, sdkBacktrace);
			case WalletItemNotFound:
				return new WalletItemNotFoundException(sdkMessage, sdkBacktrace);
			case WalletItemAlreadyExists:
				return new WalletItemAlreadyExistsException(sdkMessage, sdkBacktrace);
			case WalletQueryError:
				return new WalletInvalidQueryException(sdkMessage, sdkBacktrace);
			case WalletIncompatiblePoolError:
				return new WrongWalletForPoolException(sdkMessage, sdkBacktrace);
			case WalletAlreadyOpenedError:
				return new WalletAlreadyOpenedException(sdkMessage, sdkBacktrace);
			case WalletAccessFailed:
				return new WalletAccessFailedException(sdkMessage, sdkBacktrace);
			case PoolLedgerNotCreatedError:
				return new PoolConfigNotCreatedException(sdkMessage, sdkBacktrace);
			case PoolLedgerInvalidPoolHandle:
				return new InvalidPoolException(sdkMessage, sdkBacktrace);
			case PoolLedgerTerminated:
				return new PoolLedgerTerminatedException(sdkMessage, sdkBacktrace);
			case LedgerNoConsensusError:
				return new ConsensusException(sdkMessage, sdkBacktrace);
			case LedgerInvalidTransaction:
				return new LedgerInvalidTransactionException(sdkMessage, sdkBacktrace);
			case LedgerSecurityError:
				return new LedgerSecurityException(sdkMessage, sdkBacktrace);
			case PoolLedgerConfigAlreadyExistsError:
				return new PoolLedgerConfigExistsException(sdkMessage, sdkBacktrace);
			case PoolLedgerTimeout:
				return new TimeoutException(sdkMessage, sdkBacktrace);
			case PoolIncompatibleProtocolVersion:
				return new PoolIncompatibleProtocolVersionException(sdkMessage, sdkBacktrace);
			case LedgerNotFound:
				return new LedgerNotFoundException(sdkMessage, sdkBacktrace);
			case AnoncredsRevocationRegistryFullError:
				return new RevocationRegistryFullException(sdkMessage, sdkBacktrace);
			case AnoncredsInvalidUserRevocId:
				return new AnoncredsInvalidUserRevocId(sdkMessage, sdkBacktrace);
			case AnoncredsMasterSecretDuplicateNameError:
				return new DuplicateMasterSecretNameException(sdkMessage, sdkBacktrace);
			case AnoncredsProofRejected:
				return new ProofRejectedException(sdkMessage, sdkBacktrace);
			case AnoncredsCredentialRevoked:
				return new CredentialRevokedException(sdkMessage, sdkBacktrace);
			case AnoncredsCredDefAlreadyExistsError:
				return new CredDefAlreadyExistsException(sdkMessage, sdkBacktrace);
			case UnknownCryptoTypeError:
				return new UnknownCryptoException(sdkMessage, sdkBacktrace);
			case DidAlreadyExistsError:
				return new DidAlreadyExistsException(sdkMessage, sdkBacktrace);
			case UnknownPaymentMethod:
				return new UnknownPaymentMethodException(sdkMessage, sdkBacktrace);
			case IncompatiblePaymentError:
				return new IncompatiblePaymentException(sdkMessage, sdkBacktrace);
			case InsufficientFundsError:
				return new InsufficientFundsException(sdkMessage, sdkBacktrace);
			case ExtraFundsError:
				return new ExtraFundsException(sdkMessage, sdkBacktrace);
			case PaymentSourceDoesNotExistError:
				return new PaymentSourceDoesNotExistException(sdkMessage, sdkBacktrace);
			case PaymentOperationNotSupportedError:
				return new PaymentOperationNotSupportedException(sdkMessage, sdkBacktrace);
			default:
				String message = String.format("An unmapped error with the code '%s' was returned by the SDK.", sdkErrorCode);
				return new IndyException(message, sdkErrorCode, sdkBacktrace);
		}
	}
}


