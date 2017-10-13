package org.hyperledger.indy.sdk;

public class ClaimRevokedException extends IndyException
{
	private final static String message = "The claim has been revoked.";

	ClaimRevokedException(int sdkErrorCode) 
    {
    	super(message, sdkErrorCode);
    }
}