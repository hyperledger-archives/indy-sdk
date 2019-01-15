package org.hyperledger.indy.sdk.anoncreds;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.IndyException;

/**
 * Exception thrown when a invalid user revocation index is used.
 */
public class AnoncredsInvalidUserRevocId extends IndyException
{
	private static final long serialVersionUID = 4969718227042210813L;
	private final static String message = "The user revocation registry index specified is invalid.";

	/**
	 * Initializes a new AnoncredsInvalidUserRevocId.
	 */
	public AnoncredsInvalidUserRevocId()
	{
		super(message, ErrorCode.AnoncredsInvalidUserRevocId.value());
	}
	
	/**
	 * Initializes a new AnoncredsInvalidUserRevocId.
	 *
	 * @param sdkMessage The SDK error message.
	 * @param sdkBacktrace The SDK error backtrace.
	 */
	public AnoncredsInvalidUserRevocId(String sdkMessage, String sdkBacktrace)
    {
    	super(sdkMessage, ErrorCode.AnoncredsInvalidUserRevocId.value(), sdkBacktrace);
    }
}