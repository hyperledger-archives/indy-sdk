package org.hyperledger.indy.sdk.wallet;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.IndyException;

/**
 * Exception thrown when decoding of wallet data during input/output failed.
 */
public class WalletDecodingException extends IndyException
{
	private static final long serialVersionUID = 1829076830401150667L;
	private final static String message = "Decoding of wallet data during input/output failed.";

	/**
	 * Initializes a new WalletDecodingException.
	 */
	public WalletDecodingException()
	{
		super(message, ErrorCode.WalletDecodingError.value());
	}

	/**
	 * Initializes a new WalletDecodingException.
	 *
	 * @param sdkMessage The SDK error message.
	 * @param sdkBacktrace The SDK error backtrace.
	 */
	public WalletDecodingException(String sdkMessage, String sdkBacktrace)
    {
    	super(sdkMessage, ErrorCode.WalletDecodingError.value(), sdkBacktrace);
    }
}