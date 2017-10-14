package org.hyperledger.indy.sdk.anoncreds;

import org.hyperledger.indy.sdk.IndyException;

public class InvalidUserRevocIndexException extends IndyException
{
	/**
	 * 
	 */
	private static final long serialVersionUID = 4969718227042210813L;
	private final static String message = "The user revocation registry index specified is invalid.";

	public InvalidUserRevocIndexException(int sdkErrorCode) 
    {
    	super(message, sdkErrorCode);
    }
}