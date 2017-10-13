package org.hyperledger.indy.sdk;

public class WalletExistsException extends IndyException
{
	private final static String message = "A wallet with the specified name already exists.";

	WalletExistsException(int sdkErrorCode) 
    {
    	super(message, sdkErrorCode);
    }
}