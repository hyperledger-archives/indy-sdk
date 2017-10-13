package org.hyperledger.indy.sdk;

public class WalletValueNotFoundException extends IndyException
{
	private final static String message = "No value with the specified key exists in the wallet from which it was requested.";

	WalletValueNotFoundException(int sdkErrorCode) 
    {
    	super(message, sdkErrorCode);
    }
}
