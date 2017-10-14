package org.hyperledger.indy.sdk;

import org.hyperledger.indy.sdk.anoncreds.AccumulatorFullException;
import org.hyperledger.indy.sdk.anoncreds.NotIssuedException;
import org.hyperledger.indy.sdk.anoncreds.DuplicateMasterSecretNameException;
import org.hyperledger.indy.sdk.anoncreds.InvalidUserRevocIndexException;
import org.hyperledger.indy.sdk.anoncreds.ProofRejectedException;
import org.hyperledger.indy.sdk.anoncreds.RevocationRegistryFullException;
import org.hyperledger.indy.sdk.pool.InvalidLedgerTransactionException;
import org.hyperledger.indy.sdk.pool.ConsensusException;
import org.hyperledger.indy.sdk.pool.LedgerSecurityException;
import org.hyperledger.indy.sdk.pool.PoolClosedException;
import org.hyperledger.indy.sdk.pool.PoolConfigNotCreatedException;
import org.hyperledger.indy.sdk.pool.PoolLedgerConfigExistsException;
import org.hyperledger.indy.sdk.pool.PoolLedgerTerminatedException;
import org.hyperledger.indy.sdk.signus.UnknownCryptoException;
import org.hyperledger.indy.sdk.wallet.DuplicateWalletTypeException;
import org.hyperledger.indy.sdk.wallet.UnknownWalletTypeException;
import org.hyperledger.indy.sdk.wallet.WalletAlreadyOpenedException;
import org.hyperledger.indy.sdk.wallet.WalletClosedException;
import org.hyperledger.indy.sdk.wallet.WalletExistsException;
import org.hyperledger.indy.sdk.wallet.WalletValueNotFoundException;
import org.hyperledger.indy.sdk.wallet.WrongWalletForPoolException;

/**
 * Indy specific exception.
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
	 * Gets the ErrorCode for the exception.
	 * 
	 * @return The ErrorCode used to construct the exception.
	 */
	public int getSdkErrorCode() {
		return sdkErrorCode;
	}
	
	/**
	 * Initializes a new IndyException using the specified ErrorCode.
	 * 
	 * @param errorCode The error code for the exception.
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
				return new InvalidStateException(sdkErrorCode);
			case CommonInvalidStructure:
				return new InvalidStructureException(sdkErrorCode);
			case CommonIOError:
				return new IOException(sdkErrorCode);
			case WalletInvalidHandle:
				return new WalletClosedException(sdkErrorCode);
			case WalletUnknownTypeError:
				return new UnknownWalletTypeException(sdkErrorCode);
			case WalletTypeAlreadyRegisteredError:
				return new DuplicateWalletTypeException(sdkErrorCode);
			case WalletAlreadyExistsError:
				return new WalletExistsException(sdkErrorCode);
			case WalletNotFoundError:
				return new WalletValueNotFoundException(sdkErrorCode);
			case WalletIncompatiblePoolError:
				return new WrongWalletForPoolException(sdkErrorCode);
			case WalletAlreadyOpenedError:
				return new WalletAlreadyOpenedException(sdkErrorCode);
			case PoolLedgerNotCreatedError:
				return new PoolConfigNotCreatedException(sdkErrorCode);
			case PoolLedgerInvalidPoolHandle:
				return new PoolClosedException(sdkErrorCode);
			case PoolLedgerTerminated:
				return new PoolLedgerTerminatedException(sdkErrorCode);
			case LedgerNoConsensusError:
				return new ConsensusException(sdkErrorCode);
			case LedgerInvalidTransaction:
				return new InvalidLedgerTransactionException(sdkErrorCode);
			case LedgerSecurityError:
				return new LedgerSecurityException(sdkErrorCode);
			case PoolLedgerConfigAlreadyExistsError:
				return new PoolLedgerConfigExistsException(sdkErrorCode);
			case AnoncredsRevocationRegistryFullError:
				return new RevocationRegistryFullException(sdkErrorCode);
			case AnoncredsInvalidUserRevocIndex:
				return new InvalidUserRevocIndexException(sdkErrorCode);
			case AnoncredsAccumulatorIsFull:
				return new AccumulatorFullException(sdkErrorCode);
			case AnoncredsNotIssuedError:
				return new NotIssuedException(sdkErrorCode);
			case AnoncredsMasterSecretDuplicateNameError:
				return new DuplicateMasterSecretNameException(sdkErrorCode);
			case AnoncredsProofRejected:
				return new ProofRejectedException(sdkErrorCode);
			case SignusUnknownCryptoError:
				return new UnknownCryptoException(sdkErrorCode);
			default:
				String message = String.format("An unmapped error with the code '%s' was returned by the SDK.", sdkErrorCode);
				return new IndyException(message, sdkErrorCode);			
		}
	}
}


