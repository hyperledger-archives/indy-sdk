package org.hyperledger.indy.sdk;

public class DuplicateWalletTypeException extends IndyException
{
	private final static String message = "A wallet type with the specified name has already been registered.";

	DuplicateWalletTypeException(int sdkErrorCode) 
    {
    	super(message, sdkErrorCode);
    }
}