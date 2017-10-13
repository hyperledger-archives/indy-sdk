package org.hyperledger.indy.sdk;

public class InvalidStateException extends IndyException
{
	private final static String message = "The SDK library experienced an unexpected internal error.";

    InvalidStateException(int sdkErrorCode) 
    {
    	super(message, sdkErrorCode);
    }
}
