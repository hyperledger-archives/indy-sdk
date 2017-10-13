package org.hyperledger.indy.sdk;

public class UnknownWalletTypeException extends IndyException
{
	private final static String message = "The wallet type specified has not been registered.";

	UnknownWalletTypeException(int sdkErrorCode) 
    {
    	super(message, sdkErrorCode);
    }
}