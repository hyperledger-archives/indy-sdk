package org.hyperledger.indy.sdk;

public class LedgerSecurityException extends IndyException
{
	private final static String message = "The transaction cannot be sent as the privileges for the current pool connection don't allow it.";

	LedgerSecurityException(int sdkErrorCode) 
    {
    	super(message, sdkErrorCode);
    }
}