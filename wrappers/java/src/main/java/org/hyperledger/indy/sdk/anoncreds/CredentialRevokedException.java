package org.hyperledger.indy.sdk.anoncreds;

import org.hyperledger.indy.sdk.ErrorCode;
import org.hyperledger.indy.sdk.IndyException;

/**
 * Exception thrown when attempting to use a credential that has been revoked.
 */
public class CredentialRevokedException extends IndyException
{
	private static final long serialVersionUID = 8269746965241515882L;
	private final static String message = "The credential has been revoked.";

	/**
	 * Initializes a new CredentialRevokedException.
	 */
	public CredentialRevokedException() {
		super(message, ErrorCode.AnoncredsCredentialRevoked.value());
	}

	/**
	 * Initializes a new CredentialRevokedException.
	 *
	 * @param sdkMessage The SDK error message.
	 * @param sdkBacktrace The SDK error backtrace.
	 */
	public CredentialRevokedException(String sdkMessage, String sdkBacktrace)
    {
    	super(sdkMessage, ErrorCode.AnoncredsCredentialRevoked.value(), sdkBacktrace);
    }
}