package org.hyperledger.indy.sdk.anoncreds;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.IndyException;

/**
 * Exception thrown when an anoncreds accumulator is full.
 */
public class CredDefAlreadyExistsException extends IndyException
{
	private static final long serialVersionUID = -6792822612990030627L;
	private final static String message = "The anoncreds accumulator is full.";

	/**
	 * Initializes a new CredDefAlreadyExistsException.
	 */
	public CredDefAlreadyExistsException() {
		super(message, ErrorCode.AnoncredsCredDefAlreadyExistsError.value());
	}

	/**
	 * Initializes a new CredDefAlreadyExistsException.
	 *
	 * @param sdkMessage The SDK error message.
	 * @param sdkBacktrace The SDK error backtrace.
	 */
	public CredDefAlreadyExistsException(String sdkMessage, String sdkBacktrace)
    {
    	super(sdkMessage, ErrorCode.AnoncredsCredDefAlreadyExistsError.value(), sdkBacktrace);
    }
}