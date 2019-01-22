package org.hyperledger.indy.sdk.wallet;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.IndyException;

/**
 * Exception thrown occurred during encryption-related operations.
 */
public class WalletEncryptionException extends IndyException
{
	private static final long serialVersionUID = 1829076830401150667L;
	private final static String message = "Error during encryption-related operations.";

	/**
	 * Initializes a new WalletEncryptionException.
	 */
	public WalletEncryptionException()
	{
		super(message, ErrorCode.WalletEncryptionError.value());
	}

	/**
	 * Initializes a new WalletEncryptionException.
	 *
	 * @param sdkMessage The SDK error message.
	 * @param sdkBacktrace The SDK error backtrace.
	 */
	public WalletEncryptionException(String sdkMessage, String sdkBacktrace)
    {
    	super(sdkMessage, ErrorCode.WalletEncryptionError.value(), sdkBacktrace);
    }
}