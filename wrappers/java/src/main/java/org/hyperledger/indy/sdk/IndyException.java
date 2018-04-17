package org.hyperledger.indy.sdk;

import org.hyperledger.indy.sdk.anoncreds.*;
import org.hyperledger.indy.sdk.did.DidAlreadyExistsException;
import org.hyperledger.indy.sdk.ledger.ConsensusException;
import org.hyperledger.indy.sdk.ledger.LedgerSecurityException;
import org.hyperledger.indy.sdk.ledger.TimeoutException;
import org.hyperledger.indy.sdk.pool.InvalidPoolException;
import org.hyperledger.indy.sdk.pool.PoolConfigNotCreatedException;
import org.hyperledger.indy.sdk.pool.PoolLedgerConfigExistsException;
import org.hyperledger.indy.sdk.pool.PoolLedgerTerminatedException;
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
		
		switch(errorCode){
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
				return new WalletValueNotFoundException();
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
			case LedgerSecurityError:
				return new LedgerSecurityException();
			case PoolLedgerConfigAlreadyExistsError:
				return new PoolLedgerConfigExistsException();
			case PoolLedgerTimeout:
				return new TimeoutException();
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
			default:
				String message = String.format("An unmapped error with the code '%s' was returned by the SDK.", sdkErrorCode);
				return new IndyException(message, sdkErrorCode);			
		}
	}
}


