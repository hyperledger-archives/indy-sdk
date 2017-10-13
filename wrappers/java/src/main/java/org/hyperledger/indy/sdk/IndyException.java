package org.hyperledger.indy.sdk;

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
				return new LedgerConsensusException(sdkErrorCode);
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
				return new AnoncredsAccumulatorFullException(sdkErrorCode);
			case AnoncredsNotIssuedError:
				return new AnoncredsNotIssuedException(sdkErrorCode);
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


