package org.hyperledger.indy.sdk;

public class InvalidUserRevocIndexException extends IndyException
{
	private final static String message = "The user revocation registry index specified is invalid.";

	InvalidUserRevocIndexException(int sdkErrorCode) 
    {
    	super(message, sdkErrorCode);
    }
}