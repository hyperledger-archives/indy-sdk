package org.hyperledger.indy.sdk;

public class IOException extends IndyException
{
	private final static String message = "An IO error occurred.";

	IOException(int sdkErrorCode) 
    {
    	super(message, sdkErrorCode);
    }
}