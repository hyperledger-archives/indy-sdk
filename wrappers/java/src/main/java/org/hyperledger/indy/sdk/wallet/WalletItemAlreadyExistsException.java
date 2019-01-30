package org.hyperledger.indy.sdk.wallet;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.IndyException;

/**
 * Exception thrown when add record operation is used with record name that already exists.
 */
public class WalletItemAlreadyExistsException extends IndyException
{
	private static final long serialVersionUID = 667964860056778208L;
	private final static String message = "Item already exists.";

	/**
	 * Initializes a new WalletItemNotFoundException.
	 */
	public WalletItemAlreadyExistsException() {
		super(message, ErrorCode.WalletItemAlreadyExists.value());
	}

	/**
	 * Initializes a new WalletItemNotFoundException.
	 *
	 * @param sdkMessage The SDK error message.
	 * @param sdkBacktrace The SDK error backtrace.
	 */
	public WalletItemAlreadyExistsException(String sdkMessage, String sdkBacktrace)
    {
    	super(sdkMessage, ErrorCode.WalletItemAlreadyExists.value(), sdkBacktrace);
    }
}
