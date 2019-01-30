package org.hyperledger.indy.sdk;

/**
 * Exception thrown when the SDK experienced an IO error.
 */
public class IOException extends IndyException
{
	private static final long serialVersionUID = -1581785238453075780L;
	private final static String message = "An IO error occurred.";

	/**
	 * Initializes a new IOException.
	 */
	public IOException()
	{
		super(message, ErrorCode.CommonIOError.value());
	}

	/**
	 * Initializes a new IOException.
	 *
	 * @param sdkMessage The SDK error message.
	 * @param sdkBacktrace The SDK error backtrace.
	 */
	public IOException(String sdkMessage, String sdkBacktrace)
    {
    	super(sdkMessage, ErrorCode.CommonIOError.value(), sdkBacktrace);
    }
}