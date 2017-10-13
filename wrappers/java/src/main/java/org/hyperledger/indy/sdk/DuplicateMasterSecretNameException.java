package org.hyperledger.indy.sdk;

public class DuplicateMasterSecretNameException extends IndyException
{
	private final static String message = "Another master-secret with the specified name already exists.";

	DuplicateMasterSecretNameException(int sdkErrorCode) 
    {
    	super(message, sdkErrorCode);
    }
}