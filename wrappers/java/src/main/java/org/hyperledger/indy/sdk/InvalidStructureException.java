package org.hyperledger.indy.sdk;

public class InvalidStructureException extends IndyException
{
	private final static String message = "A value being processed is not valid.";

	InvalidStructureException(int sdkErrorCode) 
    {
    	super(message, sdkErrorCode);
    }
}
