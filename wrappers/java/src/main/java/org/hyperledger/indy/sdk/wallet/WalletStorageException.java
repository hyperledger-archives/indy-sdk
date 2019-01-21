package org.hyperledger.indy.sdk.wallet;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.IndyException;

/**
 * Exception thrown occurred during wallet operation.
 */
public class WalletStorageException extends IndyException
{
	private static final long serialVersionUID = 1829076830401150667L;
	private final static String message = "Storage error occurred during wallet operation.";

	/**
	 * Initializes a new WalletStorageException.
	 */
	public WalletStorageException()
	{
		super(message, ErrorCode.WalletStorageError.value());
	}

	/**
	 * Initializes a new WalletStorageException.
	 *
	 * @param sdkMessage The SDK error message.
	 * @param sdkBacktrace The SDK error backtrace.
	 */
	public WalletStorageException(String sdkMessage, String sdkBacktrace)
    {
    	super(sdkMessage, ErrorCode.WalletStorageError.value(), sdkBacktrace);
    }
}