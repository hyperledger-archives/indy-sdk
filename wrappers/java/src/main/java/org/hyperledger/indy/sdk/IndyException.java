package org.hyperledger.indy.sdk;

import com.sun.jna.ptr.PointerByReference;
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
import org.json.JSONObject;

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

	private static class ErrorDetails{
		String message;
		String backtrace;

		private ErrorDetails() {
			PointerByReference errorDetailsJson = new PointerByReference();

			LibIndy.api.indy_get_current_error(errorDetailsJson);

			JSONObject errorDetails = new JSONObject(errorDetailsJson.getValue().getString(0));
			this.message = errorDetails.optString("message");
			this.backtrace = errorDetails.optString("backtrace");
		}
	}

	/**
	 * Initializes a new IndyException using the specified SDK error code.
	 *
	 * @param sdkErrorCode The SDK error code to construct the exception from.
	 *
	 * @return IndyException correspondent to SDK error code
	 */
	public static IndyException fromSdkError(int sdkErrorCode) {

		ErrorCode errorCode = ErrorCode.valueOf(sdkErrorCode);
		ErrorDetails errorDetails = new ErrorDetails();

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
				return new InvalidParameterException(sdkErrorCode, errorDetails.message, errorDetails.backtrace);
			case CommonInvalidState:
				return new InvalidStateException(errorDetails.message, errorDetails.backtrace);
			case CommonInvalidStructure:
				return new InvalidStructureException(errorDetails.message, errorDetails.backtrace);
			case CommonIOError:
				return new IOException(errorDetails.message, errorDetails.backtrace);
			case WalletInvalidHandle:
				return new InvalidWalletException(errorDetails.message, errorDetails.backtrace);
			case WalletUnknownTypeError:
				return new UnknownWalletTypeException(errorDetails.message, errorDetails.backtrace);
			case WalletTypeAlreadyRegisteredError:
				return new DuplicateWalletTypeException(errorDetails.message, errorDetails.backtrace);
			case WalletAlreadyExistsError:
				return new WalletExistsException(errorDetails.message, errorDetails.backtrace);
			case WalletNotFoundError:
				return new WalletNotFoundException(errorDetails.message, errorDetails.backtrace);
			case WalletInputError:
				return new WalletInputException(errorDetails.message, errorDetails.backtrace);
			case WalletDecodingError:
				return new WalletDecodingException(errorDetails.message, errorDetails.backtrace);
			case WalletStorageError:
				return new WalletStorageException(errorDetails.message, errorDetails.backtrace);
			case WalletEncryptionError:
				return new WalletEncryptionException(errorDetails.message, errorDetails.backtrace);
			case WalletItemNotFound:
				return new WalletItemNotFoundException(errorDetails.message, errorDetails.backtrace);
			case WalletItemAlreadyExists:
				return new WalletItemAlreadyExistsException(errorDetails.message, errorDetails.backtrace);
			case WalletQueryError:
				return new WalletInvalidQueryException(errorDetails.message, errorDetails.backtrace);
			case WalletIncompatiblePoolError:
				return new WrongWalletForPoolException(errorDetails.message, errorDetails.backtrace);
			case WalletAlreadyOpenedError:
				return new WalletAlreadyOpenedException(errorDetails.message, errorDetails.backtrace);
			case WalletAccessFailed:
				return new WalletAccessFailedException(errorDetails.message, errorDetails.backtrace);
			case PoolLedgerNotCreatedError:
				return new PoolConfigNotCreatedException(errorDetails.message, errorDetails.backtrace);
			case PoolLedgerInvalidPoolHandle:
				return new InvalidPoolException(errorDetails.message, errorDetails.backtrace);
			case PoolLedgerTerminated:
				return new PoolLedgerTerminatedException(errorDetails.message, errorDetails.backtrace);
			case LedgerNoConsensusError:
				return new ConsensusException(errorDetails.message, errorDetails.backtrace);
			case LedgerInvalidTransaction:
				return new LedgerInvalidTransactionException(errorDetails.message, errorDetails.backtrace);
			case LedgerSecurityError:
				return new LedgerSecurityException(errorDetails.message, errorDetails.backtrace);
			case PoolLedgerConfigAlreadyExistsError:
				return new PoolLedgerConfigExistsException(errorDetails.message, errorDetails.backtrace);
			case PoolLedgerTimeout:
				return new TimeoutException(errorDetails.message, errorDetails.backtrace);
			case PoolIncompatibleProtocolVersion:
				return new PoolIncompatibleProtocolVersionException(errorDetails.message, errorDetails.backtrace);
			case LedgerNotFound:
				return new LedgerNotFoundException(errorDetails.message, errorDetails.backtrace);
			case AnoncredsRevocationRegistryFullError:
				return new RevocationRegistryFullException(errorDetails.message, errorDetails.backtrace);
			case AnoncredsInvalidUserRevocId:
				return new AnoncredsInvalidUserRevocId(errorDetails.message, errorDetails.backtrace);
			case AnoncredsMasterSecretDuplicateNameError:
				return new DuplicateMasterSecretNameException(errorDetails.message, errorDetails.backtrace);
			case AnoncredsProofRejected:
				return new ProofRejectedException(errorDetails.message, errorDetails.backtrace);
			case AnoncredsCredentialRevoked:
				return new CredentialRevokedException(errorDetails.message, errorDetails.backtrace);
			case AnoncredsCredDefAlreadyExistsError:
				return new CredDefAlreadyExistsException(errorDetails.message, errorDetails.backtrace);
			case UnknownCryptoTypeError:
				return new UnknownCryptoException(errorDetails.message, errorDetails.backtrace);
			case DidAlreadyExistsError:
				return new DidAlreadyExistsException(errorDetails.message, errorDetails.backtrace);
			case UnknownPaymentMethod:
				return new UnknownPaymentMethodException(errorDetails.message, errorDetails.backtrace);
			case IncompatiblePaymentError:
				return new IncompatiblePaymentException(errorDetails.message, errorDetails.backtrace);
			case InsufficientFundsError:
				return new InsufficientFundsException(errorDetails.message, errorDetails.backtrace);
			case ExtraFundsError:
				return new ExtraFundsException(errorDetails.message, errorDetails.backtrace);
			case PaymentSourceDoesNotExistError:
				return new PaymentSourceDoesNotExistException(errorDetails.message, errorDetails.backtrace);
			case PaymentOperationNotSupportedError:
				return new PaymentOperationNotSupportedException(errorDetails.message, errorDetails.backtrace);
			default:
				String message = String.format("An unmapped error with the code '%s' was returned by the SDK.", sdkErrorCode);
				return new IndyException(errorDetails.message, sdkErrorCode, errorDetails.backtrace);
		}
	}
}


