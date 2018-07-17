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

	/**
	 * Initializes a new IndyException with the specified message.
	 *
	 * @param message The message for the exception.
	 */
	protected IndyException(String message, int sdkErrorCode) {
		super(message);
		this.sdkErrorCode = sdkErrorCode;
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
	 * Initializes a new IndyException using the specified SDK error code.
	 *
	 * @param sdkErrorCode The SDK error code to construct the exception from.
	 */
	public static IndyException fromSdkError(int sdkErrorCode) {

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
				return new InvalidParameterException(sdkErrorCode);
			case CommonInvalidState:
				return new InvalidStateException();
			case CommonInvalidStructure:
				return new InvalidStructureException();
			case CommonIOError:
				return new IOException();
			case WalletInvalidHandle:
				return new InvalidWalletException();
			case WalletUnknownTypeError:
				return new UnknownWalletTypeException();
			case WalletTypeAlreadyRegisteredError:
				return new DuplicateWalletTypeException();
			case WalletAlreadyExistsError:
				return new WalletExistsException();
			case WalletNotFoundError:
				return new WalletNotFoundException();
			case WalletInputError:
				return new WalletInputException();
			case WalletDecodingError:
				return new WalletDecodingException();
			case WalletStorageError:
				return new WalletStorageException();
			case WalletEncryptionError:
				return new WalletEncryptionException();
			case WalletItemNotFound:
				return new WalletItemNotFoundException();
			case WalletItemAlreadyExists:
				return new WalletItemAlreadyExistsException();
			case WalletQueryError:
				return new WalletInvalidQueryException();
			case WalletIncompatiblePoolError:
				return new WrongWalletForPoolException();
			case WalletAlreadyOpenedError:
				return new WalletAlreadyOpenedException();
			case WalletAccessFailed:
				return new WalletAccessFailedException();
			case PoolLedgerNotCreatedError:
				return new PoolConfigNotCreatedException();
			case PoolLedgerInvalidPoolHandle:
				return new InvalidPoolException();
			case PoolLedgerTerminated:
				return new PoolLedgerTerminatedException();
			case LedgerNoConsensusError:
				return new ConsensusException();
			case LedgerInvalidTransaction:
				return new LedgerInvalidTransactionException();
			case LedgerSecurityError:
				return new LedgerSecurityException();
			case PoolLedgerConfigAlreadyExistsError:
				return new PoolLedgerConfigExistsException();
			case PoolLedgerTimeout:
				return new TimeoutException();
			case PoolIncompatibleProtocolVersion:
				return new PoolIncompatibleProtocolVersionException();
			case AnoncredsRevocationRegistryFullError:
				return new RevocationRegistryFullException();
			case AnoncredsInvalidUserRevocId:
				return new AnoncredsInvalidUserRevocId();
			case AnoncredsMasterSecretDuplicateNameError:
				return new DuplicateMasterSecretNameException();
			case AnoncredsProofRejected:
				return new ProofRejectedException();
			case AnoncredsCredentialRevoked:
				return new CredentialRevokedException();
			case AnoncredsCredDefAlreadyExistsError:
				return new CredDefAlreadyExistsException();
			case UnknownCryptoTypeError:
				return new UnknownCryptoException();
			case DidAlreadyExistsError:
				return new DidAlreadyExistsException();
			case UnknownPaymentMethod:
				return new UnknownPaymentMethodException();
			case IncompatiblePaymentError:
				return new IncompatiblePaymentException();
			case InsufficientFundsError:
				return new InsufficientFundsException();
			case PaymentSourceDoesNotExistError:
				return new PaymentSourceDoesNotExistException();
			default:
				String message = String.format("An unmapped error with the code '%s' was returned by the SDK.", sdkErrorCode);
				return new IndyException(message, sdkErrorCode);
		}
	}
}


